use crate::{
    env::{
        environment::{self, Environment, Error, Experience, Terminal},
        rapier::{lunar_lander::config::LunarLanderConfig, utils::PhysicsWorld},
    },
    spaces::{Boxed, EnvSpace, Mixed, MixedItem},
};
use nalgebra::{point, Isometry2, SVector, Vector2};
use rand::Rng;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, CollisionEvent, ImpulseJointHandle, InteractionGroups,
    RevoluteJointBuilder, RigidBodyBuilder, RigidBodyHandle, RigidBodyType,
};
use std::f64::consts::{PI, TAU};

const CHUNKS: usize = 11;
const MIDDLE: usize = CHUNKS / 2;
const LANDER_POLY: [(i32, i32); 6] = [
    (-14, 17),
    (-17, 0),
    (-17, -10),
    (17, -10),
    (17, 0),
    (14, 17),
];
const LANDER_POLY_WIDTH: f64 = 34.0;
const ACTION_SIZE: usize = 2;
const STATE_SIZE: usize = 8;

type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

struct Leg {
    pub body: RigidBodyHandle,
    pub joint: ImpulseJointHandle,
    pub ground_contact: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct Helipad {
    y: f64,
    x1: f64,
    x2: f64,
}

pub struct LunarLander {
    config: LunarLanderConfig,
    t: u32,
    state: Option<State>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
    helipad: Helipad,
    world: PhysicsWorld,
    lander: RigidBodyHandle,
    legs: Vec<Leg>,
    moon: RigidBodyHandle,
    crash: bool,
    prev_shaping: Option<f64>,
    wind_idx: f64,
    torque_idx: f64,
}

impl LunarLander {
    fn new(config: LunarLanderConfig) -> Result<Self, Error> {
        let ha = SVector::from_vec(vec![1.0, 1.0]);
        let hs = SVector::from_vec(vec![2.5, 2.5, 10.0, 10.0, TAU, 10.0, 1.0, 1.0]);
        let space = EnvSpace {
            state: Boxed::new(-hs, hs)?,
            action: match config.continuous {
                true => Mixed::continuous(-ha, ha)?,
                false => Mixed::discrete(4)?,
            },
        };
        let mut lunar_lander = Self {
            config,
            t: 0,
            state: None,
            space,
            helipad: Helipad::default(),
            world: PhysicsWorld::new(config.gravity as f32),
            lander: Default::default(),
            legs: Vec::new(),
            moon: Default::default(),
            prev_shaping: None,
            crash: false,
            wind_idx: 0.0,
            torque_idx: 0.0,
        };

        lunar_lander.reset(None)?;

        Ok(lunar_lander)
    }

    fn get_init_xy(&self) -> (f64, f64) {
        (
            self.config.get_scaled_width() / 2.0,
            self.config.get_scaled_height(),
        )
    }

    fn create_terrain(&mut self) {
        let w = self.config.get_scaled_width();
        let h = self.config.get_scaled_height();
        self.helipad.y = h / 4.0;

        let mut rng = rand::rng();
        let height: Vec<f64> = (0..=CHUNKS)
            .map(|i| match i {
                x if ((MIDDLE - 2)..=(MIDDLE + 2)).contains(&x) => self.helipad.y,
                _ => rng.random_range(0.0..h / 2.0),
            })
            .collect();
        let chunk_x: Vec<f64> = (0..CHUNKS)
            .map(|i| w / (CHUNKS - 1) as f64 * i as f64)
            .collect();

        self.helipad.x1 = chunk_x[MIDDLE - 1];
        self.helipad.x2 = chunk_x[MIDDLE + 1];

        let smooth_y: Vec<f64> = (0..CHUNKS)
            .map(|i| {
                0.33 * (height[i]
                    + height[i + 1]
                    + match i {
                        0 => height[CHUNKS],
                        _ => height[i - 1],
                    })
            })
            .collect();

        let moon_body = RigidBodyBuilder::new(RigidBodyType::Fixed).build();
        let moon_handle = self.world.rigid_body_set.insert(moon_body);
        self.moon = moon_handle;

        (0..(CHUNKS - 1)).for_each(|i| {
            let p1 = point![chunk_x[i] as f32, smooth_y[i] as f32];
            let p2 = point![chunk_x[i + 1] as f32, smooth_y[i + 1] as f32];
            let coll = ColliderBuilder::segment(p1, p2).friction(0.1).build();
            self.world.collider_set.insert_with_parent(
                coll,
                moon_handle,
                &mut self.world.rigid_body_set,
            );
        });
    }

    fn create_lander(&mut self) {
        let init_y = self.config.get_scaled_height();
        let init_x = self.config.get_scaled_width() / 2.0;
        let lander_poly = LANDER_POLY
            .iter()
            .map(|&(x, y)| {
                point![
                    x as f32 / self.config.scale as f32,
                    y as f32 / self.config.scale as f32
                ]
            })
            .collect();
        let lander_pos = nalgebra::Isometry2::new(Vector2::new(init_x as f32, init_y as f32), 0.0);
        let lander_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .position(lander_pos)
            .build();
        let lander_handle = self.world.rigid_body_set.insert(lander_body);

        let collider = ColliderBuilder::convex_polyline(lander_poly)
            .unwrap()
            .density(5.0)
            .friction(0.1)
            .restitution(0.0)
            .build();

        self.world.collider_set.insert_with_parent(
            collider,
            lander_handle,
            &mut self.world.rigid_body_set,
        );

        let mut rng = rand::rng();
        let force = self.config.initial_random;
        let force_x = rng.random_range(-force..force) as f32;
        let force_y = rng.random_range(-force..force) as f32;

        if let Some(body) = self.world.rigid_body_set.get_mut(lander_handle) {
            body.apply_impulse(Vector2::new(force_x, force_y), true);
        }

        self.lander = lander_handle;
    }

    fn create_legs(&mut self) {
        self.legs.clear();
        let (init_x, init_y) = self.get_init_xy();

        for i in [-1., 1.] {
            let pos = Isometry2::new(
                Vector2::new(
                    init_x as f32 - i * self.config.leg_offset_x as f32 / self.config.scale as f32,
                    init_y as f32,
                ),
                i * 0.05,
            );

            let body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .position(pos)
                .build();
            let handle = self.world.rigid_body_set.insert(body);

            let hx = self.config.leg_width / self.config.scale / 2.0;
            let hy = self.config.leg_length / self.config.scale / 2.0;
            let coll = ColliderBuilder::cuboid(hx as f32, hy as f32)
                .density(1.0)
                .restitution(0.0)
                .collision_groups(InteractionGroups::new(0x0020.into(), 0x001.into()))
                .build();

            self.world.collider_set.insert_with_parent(
                coll,
                handle,
                &mut self.world.rigid_body_set,
            );

            let joint = RevoluteJointBuilder::new()
                .local_anchor1(point![0.0, 0.0])
                .local_anchor2(point![
                    i * self.config.leg_offset_x as f32 / self.config.scale as f32,
                    self.config.leg_offset_y as f32 / self.config.scale as f32,
                ])
                .motor_velocity(0.3 * i, self.config.leg_spring_torque as f32)
                .limits(if i == -1. { [0.9, 0.4] } else { [-0.4, -0.9] })
                .build();

            let joint_handle =
                self.world
                    .impulse_joint_set
                    .insert(self.lander, handle, joint, true);

            self.legs.push(Leg {
                body: handle,
                joint: joint_handle,
                ground_contact: false,
            });
        }
    }

    fn apply_engine_forces(&mut self, action: &Action) -> (f32, f32) {
        let mut rng = rand::rng();
        let mut main_force = 0.0;
        let mut side_force = 0.0;

        // Extract needed data first
        let (angle, translation) =
            if let Some(lander_body) = self.world.rigid_body_set.get(self.lander) {
                (lander_body.rotation().angle(), *lander_body.translation())
            } else {
                return (0.0, 0.0);
            };

        let tip = (angle.sin(), angle.cos());
        let side = (-tip.1, tip.0);
        let dispersion = [
            rng.random_range(-1.0..1.0) / self.config.scale as f32,
            rng.random_range(-1.0..1.0) / self.config.scale as f32,
        ];

        // MAIN ENGINE FORCE
        let main_engine_active = match action {
            MixedItem::Discrete(2) => true,
            MixedItem::Continuous(matrix) => matrix[0] > 0.0,
            _ => false,
        };

        if main_engine_active {
            main_force = match action {
                MixedItem::Discrete(2) => 1.0,
                MixedItem::Continuous(matrix) => 0.5 * matrix[0].clamp(0.0, 1.0) + 0.5,
                _ => 0.0,
            } as f32;

            let ox = tip.0
                * (self.config.main_engine_y_position as f32 / self.config.scale as f32
                    + 2.0 * dispersion[0])
                + side.0 * dispersion[1];
            let oy = -tip.1
                * (self.config.main_engine_y_position as f32 / self.config.scale as f32
                    + 2.0 * dispersion[0])
                - side.1 * dispersion[1];
            let impulse_pos = point![translation.x + ox, translation.y + oy];
            let force = Vector2::new(
                -ox * self.config.main_engine_force as f32 * main_force,
                -oy * self.config.main_engine_force as f32 * main_force,
            );

            if let Some(body) = self.world.rigid_body_set.get_mut(self.lander) {
                body.apply_impulse_at_point(force, impulse_pos, true);
            }
        }

        // SIDE ENGINE FORCE
        let (side_engine_active, direction) = match action {
            MixedItem::Discrete(1) => (true, 1.0),  // Left engine
            MixedItem::Discrete(3) => (true, -1.0), // Right engine
            MixedItem::Continuous(actions) => (actions[1].abs() > 0.5, actions[1].signum()),
            _ => (false, 0.0),
        };

        if side_engine_active {
            side_force = match action {
                MixedItem::Continuous(actions) => actions[1].abs().clamp(0.5, 1.0),
                _ => 1.0,
            } as f32;

            let ox = tip.0 * dispersion[0]
                + side.0
                    * (3.0 * dispersion[1]
                        + (direction * self.config.side_engine_offset_x / self.config.scale)
                            as f32);
            let oy = -tip.1 * dispersion[0]
                - side.1
                    * (3.0 * dispersion[1]
                        + (direction * self.config.side_engine_offset_x / self.config.scale)
                            as f32);
            let impulse_pos = point![
                translation.x + ox
                    - tip.0 * LANDER_POLY_WIDTH as f32 / 2.0 / self.config.scale as f32,
                translation.y
                    + oy
                    + tip.1 * self.config.side_engine_offset_y as f32 / self.config.scale as f32
            ];
            let force = Vector2::new(
                -ox * side_force * self.config.side_engine_force as f32,
                -oy * side_force * self.config.side_engine_force as f32,
            );

            if let Some(body) = self.world.rigid_body_set.get_mut(self.lander) {
                body.apply_impulse_at_point(force, impulse_pos, true);
            }
        }

        (main_force, side_force)
    }

    fn apply_wind_effects(&mut self) {
        if self.legs[0].ground_contact
            || self.legs[1].ground_contact
            || self.config.wind_strength.is_none()
        {
            return;
        }
        self.wind_idx += 1.0;
        self.torque_idx += 1.0;
        let c = ((0.02 * self.wind_idx).sin() + (PI * 0.01 * self.wind_idx).sin()).tanh();
        let wind_power = self.config.wind_strength.unwrap_or(0.0);
        let wind_mag = c * wind_power;
        let torque_mag = ((0.02 * self.torque_idx).sin() + (PI * 0.01 * self.torque_idx).sin())
            .tanh()
            * self.config.turbulence_strength;
        if let Some(lander_body) = self.world.rigid_body_set.get_mut(self.lander) {
            lander_body.apply_impulse(Vector2::new(wind_mag as f32, 0.0), true);
            lander_body.apply_torque_impulse(torque_mag as f32, true);
        }
    }

    fn get_state(&self) -> State {
        if let Some(lander_body) = self.world.rigid_body_set.get(self.lander) {
            let pos = lander_body.translation();
            let vel = lander_body.linvel();
            let theta = lander_body.rotation().angle();
            let omega = lander_body.angvel();
            SVector::from_vec(vec![
                ((pos.x as f64 - self.config.get_scaled_width() / 2.0)
                    / (self.config.get_scaled_width() / 2.0)),
                ((pos.y as f64 - (self.helipad.y + self.config.leg_offset_y / self.config.scale))
                    / (self.config.get_scaled_height() / 2.0)),
                vel.x as f64 * (self.config.get_scaled_width() / 2.0) / self.config.fps as f64,
                vel.y as f64 * (self.config.get_scaled_height() / 2.0) / self.config.fps as f64,
                theta as f64,
                20.0 * omega as f64 / self.config.fps as f64,
                if self.legs[0].ground_contact {
                    1.0
                } else {
                    0.0
                },
                if self.legs[1].ground_contact {
                    1.0
                } else {
                    0.0
                },
            ])
        } else {
            SVector::zeros()
        }
    }

    fn calc_reward(&self, state: &State, main_force: f64, side_force: f64) -> (f64, f64) {
        let shaping = -100.0 * (state[0] * state[0] + state[1] * state[1]).sqrt()
            - 100.0 * (state[2] * state[2] + state[3] * state[3]).sqrt()
            - 100.0 * state[4].abs()
            + 10.0 * state[6]
            + 10.0 * state[7];

        let mut reward = 0.0;
        if let Some(prev) = self.prev_shaping {
            reward = shaping - prev;
        }

        reward -= main_force * 0.30;
        reward -= side_force * 0.03;

        (reward, shaping)
    }

    fn is_out_of_screen(&self) -> bool {
        if let Some(state) = &self.state {
            state[0].abs() >= 1.0
        } else {
            false
        }
    }

    fn has_collided<T: PartialEq + Copy>(&self, parents: (T, T), a: T, b: T) -> bool {
        parents == (a, b) || parents == (b, a)
    }

    fn to_rigid(
        &self,
        h1: ColliderHandle,
        h2: ColliderHandle,
    ) -> Result<(RigidBodyHandle, RigidBodyHandle), ()> {
        Ok((
            self.world
                .collider_set
                .get(h1)
                .and_then(|c| c.parent())
                .ok_or(())?,
            self.world
                .collider_set
                .get(h2)
                .and_then(|c| c.parent())
                .ok_or(())?,
        ))
    }

    fn has_crashed(&self, h1: ColliderHandle, h2: ColliderHandle) -> Option<bool> {
        let parents = (
            self.world.collider_set.get(h1)?.parent()?,
            self.world.collider_set.get(h2)?.parent()?,
        );
        Some(self.lander == parents.0 || self.lander == parents.1)
    }

    fn is_leg_collided(&self, h1: ColliderHandle, h2: ColliderHandle) -> Option<(bool, bool)> {
        let parents = (
            self.world.collider_set.get(h1)?.parent()?,
            self.world.collider_set.get(h2)?.parent()?,
        );

        let left_leg = self.has_collided(parents, self.moon, self.legs[0].body);
        let right_leg = self.has_collided(parents, self.moon, self.legs[1].body);
        Some((left_leg, right_leg))
    }

    fn handle_collisions(&mut self) {
        while let Ok(collision_event) = self.world.collision_recv.try_recv() {
            match collision_event {
                CollisionEvent::Started(h1, h2, _) => {
                    self.crash = self.has_crashed(h1, h2).is_some();
                    let on_ground = self.is_leg_collided(h1, h2).unwrap_or((false, false));
                    self.legs[0].ground_contact = on_ground.0;
                    self.legs[1].ground_contact = on_ground.1;
                }
                CollisionEvent::Stopped(h1, h2, _) => {
                    if let Some((left_leg, right_leg)) = self.is_leg_collided(h1, h2) {
                        self.legs[0].ground_contact &= !left_leg;
                        self.legs[1].ground_contact &= !right_leg;
                    }
                }
            }
        }
    }

    fn is_game_over(&self) -> bool {
        self.crash || self.is_out_of_screen()
    }

    fn is_landed(&self) -> bool {
        if let Some(lander_body) = self.world.rigid_body_set.get(self.lander) {
            // In Rapier, we don't have an "awake" flag, but we can check if the body has
            // minimal linear and angular velocity, and all legs are in contact with ground
            let linear_velocity = lander_body.linvel();
            let angular_velocity = lander_body.angvel();
            let linear_threshold = 1e-3;
            let angular_threshold = 1e-3;

            linear_velocity.magnitude() < linear_threshold
                && angular_velocity.abs() < angular_threshold
                && self.legs.iter().all(|leg| leg.ground_contact)
        } else {
            false
        }
    }
}

impl Environment for LunarLander {
    type Action = Action;

    type State = State;
    type Info = ();

    fn reset(&mut self, _seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.world = PhysicsWorld::new(self.config.gravity as f32);
        self.t = 0;
        self.helipad = Helipad::default();
        self.prev_shaping = None;
        self.crash = false;

        if self.config.wind_strength.is_some() {
            self.wind_idx = 0.0;
            self.torque_idx = 0.0;
        }

        self.create_terrain();
        self.create_lander();
        self.create_legs();
        let state = self.get_state();
        self.state = Some(state);
        Ok((state, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<crate::env::environment::Experience<Self::State, Self::Info, Self::Action>, Error>
    {
        let curr_state = match self.state {
            Some(state) => state,
            None => return Err(environment::Error::NotInitialized),
        };

        self.apply_wind_effects();
        let (m_power, s_power) = self.apply_engine_forces(&action);

        // Step physics simulation
        self.world.step_with_dt(1.0 / self.config.fps as f32);
        self.handle_collisions();

        let state = self.get_state();
        let (reward, shaping) = self.calc_reward(&state, m_power as f64, s_power as f64); // You'll need to track prev_shaping
        self.prev_shaping = Some(shaping);

        let terminated = self.is_game_over() || self.is_landed();
        let reward = match (self.is_game_over(), self.is_landed()) {
            (false, true) => 100.0,
            (true, false) => -100.0,
            _ => reward,
        };

        self.state = Some(state);
        self.t += 1;

        let terminal = Terminal::from_flags(terminated, self.is_truncated());
        Ok(Experience {
            curr_state,
            action,
            reward,
            next_state: state,
            terminal,
            info: (),
            step: self.t,
        })
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        if let Some(state) = &self.state {
            Ok(state[0].abs() >= 1.0)
        } else {
            Err(environment::Error::NotInitialized)
        }
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(environment::Error::NotInitialized),
        }
    }
}
