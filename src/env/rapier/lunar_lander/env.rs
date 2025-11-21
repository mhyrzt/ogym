use super::terrain::generate_moon;
use super::utils::{Helipad, Leg, LANDER_POLY, LANDER_POLY_WIDTH};
use crate::{
    env::{
        environment::{self, Environment, Error, Experience, Terminal},
        rapier::{lunar_lander::config::LunarLanderConfig, world::PhysicsWorld},
    },
    spaces::{Boxed, EnvSpace, Mixed, MixedItem},
};
use nalgebra::{point, Isometry2, SVector, Vector2};
use rand::Rng;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, CollisionEvent, InteractionGroups, RevoluteJointBuilder,
    RigidBodyBuilder, RigidBodyHandle, RigidBodyType,
};
use std::f32::consts::PI;
use std::f64::consts::TAU;

const ACTION_SIZE: usize = 2;
const STATE_SIZE: usize = 8;

type State = SVector<f32, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

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
    prev_shaping: Option<f32>,
    wind_idx: f32,
    torque_idx: f32,
}

impl LunarLander {
    pub fn new(config: LunarLanderConfig) -> Result<Self, Error> {
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
            world: PhysicsWorld::new(config.gravity),
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

    fn get_init_xy(&self) -> (f32, f32) {
        (
            self.config.get_scaled_width() / 2.0,
            self.config.get_scaled_height(),
        )
    }

    fn create_lander(&mut self) {
        let init_y = self.config.get_scaled_height();
        let init_x = self.config.get_scaled_width() / 2.0;
        let lander_poly = LANDER_POLY
            .iter()
            .map(|&(x, y)| point![x as f32 / self.config.scale, y as f32 / self.config.scale])
            .collect();
        let lander_pos = nalgebra::Isometry2::new(Vector2::new(init_x, init_y), 0.0);
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
                    init_x - i * self.config.leg_offset_x / self.config.scale,
                    init_y,
                ),
                i * 0.05,
            );

            let body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .position(pos)
                .build();
            let handle = self.world.rigid_body_set.insert(body);

            let hx = self.config.leg_width / self.config.scale / 2.0;
            let hy = self.config.leg_length / self.config.scale / 2.0;
            let coll = ColliderBuilder::cuboid(hx, hy)
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
                    i * self.config.leg_offset_x / self.config.scale,
                    self.config.leg_offset_y / self.config.scale,
                ])
                .motor_velocity(0.3 * i, self.config.leg_spring_torque)
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

        let (angle, translation) =
            if let Some(lander_body) = self.world.rigid_body_set.get(self.lander) {
                (lander_body.rotation().angle(), *lander_body.translation())
            } else {
                return (0.0, 0.0);
            };

        let tip = (angle.sin(), angle.cos());
        let side = (-tip.1, tip.0);
        let dispersion = [
            rng.random_range(-1.0..1.0) / self.config.scale,
            rng.random_range(-1.0..1.0) / self.config.scale,
        ];

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
                * (self.config.main_engine_y_position / self.config.scale + 2.0 * dispersion[0])
                + side.0 * dispersion[1];
            let oy = -tip.1
                * (self.config.main_engine_y_position / self.config.scale + 2.0 * dispersion[0])
                - side.1 * dispersion[1];
            let impulse_pos = point![translation.x + ox, translation.y + oy];
            let force = Vector2::new(
                -ox * self.config.main_engine_force * main_force,
                -oy * self.config.main_engine_force * main_force,
            );

            if let Some(body) = self.world.rigid_body_set.get_mut(self.lander) {
                body.apply_impulse_at_point(force, impulse_pos, true);
            }
        }

        let (side_engine_active, direction) = match action {
            MixedItem::Discrete(1) => (true, 1.0),
            MixedItem::Discrete(3) => (true, -1.0),
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
                        + (direction as f32 * self.config.side_engine_offset_x
                            / self.config.scale));
            let oy = -tip.1 * dispersion[0]
                - side.1
                    * (3.0 * dispersion[1]
                        + (direction as f32 * self.config.side_engine_offset_x
                            / self.config.scale));
            let impulse_pos = point![
                translation.x + ox - tip.0 * LANDER_POLY_WIDTH / 2.0 / self.config.scale,
                translation.y + oy + tip.1 * self.config.side_engine_offset_y / self.config.scale
            ];
            let force = Vector2::new(
                -ox * side_force * self.config.side_engine_force,
                -oy * side_force * self.config.side_engine_force,
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
            lander_body.apply_impulse(Vector2::new(wind_mag, 0.0), true);
            lander_body.apply_torque_impulse(torque_mag, true);
        }
    }

    fn get_state(&self) -> State {
        if let Some(lander_body) = self.world.rigid_body_set.get(self.lander) {
            let pos = lander_body.translation();
            let vel = lander_body.linvel();
            let theta = lander_body.rotation().angle();
            let omega = lander_body.angvel();
            let fps = self.config.fps as f32;
            SVector::from_vec(vec![
                ((pos.x - self.config.get_scaled_width() / 2.0)
                    / (self.config.get_scaled_width() / 2.0)),
                ((pos.y - (self.helipad.y + self.config.leg_offset_y / self.config.scale))
                    / (self.config.get_scaled_height() / 2.0)),
                vel.x * (self.config.get_scaled_width() / 2.0) / fps,
                vel.y * (self.config.get_scaled_height() / 2.0) / fps,
                theta,
                20.0 * omega / fps,
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

    fn calc_reward(&self, state: &State, main_force: f32, side_force: f32) -> (f32, f32) {
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
        self.world = PhysicsWorld::new(self.config.gravity);
        self.t = 0;
        self.helipad = Helipad::default();
        self.prev_shaping = None;
        self.crash = false;

        if self.config.wind_strength.is_some() {
            self.wind_idx = 0.0;
            self.torque_idx = 0.0;
        }

        let (helipad, moon) = generate_moon(&self.config, &mut self.world);
        self.helipad = helipad;
        self.moon = moon;

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

        self.world.step_with_dt(1.0 / self.config.fps as f32);
        self.handle_collisions();

        let state = self.get_state();
        let (reward, shaping) = self.calc_reward(&state, m_power, s_power);
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
            reward: reward as f64,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::MixedItem;

    // Helper to get a standard config
    fn get_test_config() -> LunarLanderConfig {
        LunarLanderConfig::default()
    }

    #[test]
    fn test_initialization_and_reset() {
        let config = get_test_config();
        let env = LunarLander::new(config).expect("Failed to create environment");

        // Check initial internal state
        assert_eq!(env.t, 0);
        assert!(env.state.is_some());
        assert!(!env.crash);
        assert!(env.prev_shaping.is_none());

        // Verify state dimension
        let state = env.state().expect("State should exist");
        assert_eq!(state.len(), STATE_SIZE);

        // Verify observation bounds (rough check based on initialization)
        // X should be around 0.0 (middle of screen centered)
        // Y should be around 1.0 (top of screen)
        assert!(state[0].abs() < 0.1);
        assert!(state[1] > 0.8);
    }

    #[test]
    fn test_action_space_modes() {
        // Discrete Mode
        let config_discrete = get_test_config().with_discrete_action();
        let env_discrete = LunarLander::new(config_discrete).unwrap();

        if let crate::spaces::Mixed::Discrete(disccrete) = env_discrete.space.action {
            assert_eq!(disccrete.size(), 4);
        } else {
            panic!("Expected Discrete action space");
        }

        // Continuous Mode
        let config_continuous = get_test_config().with_continuous_action();
        let env_continuous = LunarLander::new(config_continuous).unwrap();

        if let crate::spaces::Mixed::Continuous(_) = env_continuous.space.action {
            // Success
        } else {
            panic!("Expected Continuous action space");
        }
    }

    #[test]
    fn test_step_gravity() {
        let config = get_test_config();
        let mut env = LunarLander::new(config).unwrap();
        eprintln!("Config: {:?}", config);

        // Take several steps doing nothing to ensure gravity effect is clear
        let initial_state = env.state().unwrap();
        let initial_y = initial_state[1];

        let action = MixedItem::Discrete(0);

        // Step multiple times to accumulate noticeable gravity effect
        for _ in 0..10 {
            env.step(action).unwrap();
        }

        let final_state = env.state().unwrap();
        let final_y = final_state[1];

        // Y position should decrease (falling down)
        assert!(
            final_y < initial_y,
            "Lander should fall due to gravity. Initial Y: {}, Final Y: {}",
            initial_y,
            final_y
        );
        assert_eq!(env.t, 10);
    }

    #[test]
    fn test_step_main_engine_discrete() {
        let config = get_test_config();
        let mut env = LunarLander::new(config).unwrap();

        // Let it fall for a frame to establish downward momentum
        env.step(MixedItem::Discrete(0)).unwrap();
        let state_before = env.state().unwrap();
        let vy_before = state_before[3];

        // Action 2 is main engine in discrete
        let action = MixedItem::Discrete(2);
        env.step(action).unwrap();

        let state_after = env.state().unwrap();
        let vy_after = state_after[3];

        // The engine is strong (force 13.0 vs gravity -10.0), so velocity should increase (become less negative or positive)
        // relative to just falling.
        assert!(
            vy_after > vy_before,
            "Main engine should push lander upwards"
        );
    }

    #[test]
    fn test_step_main_engine_continuous() {
        let config = get_test_config().with_continuous_action();
        let mut env = LunarLander::new(config).unwrap();

        // Let it fall for several frames to establish downward momentum
        let no_action = MixedItem::Continuous(SVector::from_vec(vec![0.0, 0.0]));
        for _ in 0..5 {
            env.step(no_action).unwrap();
        }

        let state_before = env.state().unwrap();
        let vy_before = state_before[3];

        // Apply full main engine for multiple steps
        let full_thrust = MixedItem::Continuous(SVector::from_vec(vec![1.0, 0.0]));
        for _ in 0..5 {
            env.step(full_thrust).unwrap();
        }

        let state_after = env.state().unwrap();
        let vy_after = state_after[3];

        assert!(
            vy_after > vy_before,
            "Continuous main engine should push lander upwards. VY before: {}, VY after: {}",
            vy_before,
            vy_after
        );
    }

    #[test]
    fn test_truncation() {
        let max_steps = 10;
        let config = get_test_config().with_fps(50).with_viewport_size(600, 400);
        // Create a config that limits steps
        let mut limited_env = LunarLander::new(LunarLanderConfig {
            max_steps,
            ..config
        })
        .unwrap();

        for _ in 0..max_steps {
            assert!(!limited_env.is_truncated());
            limited_env.step(MixedItem::Discrete(0)).unwrap();
        }

        assert!(
            limited_env.is_truncated(),
            "Environment should be truncated after max_steps"
        );

        // Verify Terminal flag in experience
        let experience = limited_env.step(MixedItem::Discrete(0)).unwrap();
        assert!(experience.terminal.is_truncated());
    }

    #[test]
    fn test_out_of_bounds_detection() {
        let config = get_test_config();
        let mut env = LunarLander::new(config).unwrap();

        // Manually force a state that is out of bounds
        // State[0] is x position. |x| >= 1.0 is out of bounds.
        let mut bad_state = SVector::<f32, STATE_SIZE>::zeros();
        bad_state[0] = 1.5;
        env.state = Some(bad_state);

        assert!(env.is_out_of_screen());
        assert!(env.is_game_over());
        assert!(env.is_terminal().unwrap());
    }

    #[test]
    fn test_reward_shaping() {
        let config = get_test_config();
        let env = LunarLander::new(config).unwrap();

        let mut state = SVector::<f32, STATE_SIZE>::zeros();

        // 1. Good state: centered, close to target, no velocity, upright, on ground
        state[0] = 0.0; // X centered
        state[1] = 0.0; // Y at target (not high up!)
        state[2] = 0.0; // VX
        state[3] = 0.0; // VY
        state[4] = 0.0; // Angle upright
        state[5] = 0.0; // Angular velocity
        state[6] = 1.0; // Left leg on ground (bonus!)
        state[7] = 1.0; // Right leg on ground (bonus!)

        // Compare shaping potentials (2nd return value) because the step reward (1st value)
        // is 0 when prev_shaping is None.
        let (_, shaping_good) = env.calc_reward(&state, 0.0, 0.0);

        // 2. Bad state: off-center, high up, moving fast, tilted, not on ground
        state[0] = 0.8; // Far from center
        state[1] = 0.8; // High up from target
        state[2] = 2.0; // Moving horizontally
        state[3] = -2.0; // Falling fast
        state[4] = 0.5; // Tilted
        state[5] = 1.0; // Spinning
        state[6] = 0.0; // Left leg not touching
        state[7] = 0.0; // Right leg not touching

        let (reward_bad, shaping_bad) = env.calc_reward(&state, 0.0, 0.0);

        assert!(
            shaping_good > shaping_bad,
            "Better state should yield higher shaping reward. Good: {}, Bad: {}",
            shaping_good,
            shaping_bad
        );

        // 3. Engine penalty
        // reward_bad is 0.0 (no engine, prev_shaping is None).
        // Engine use should result in negative reward here.
        let (reward_with_main_engine, _) = env.calc_reward(&state, 1.0, 0.0);
        let (reward_with_side_engine, _) = env.calc_reward(&state, 0.0, 1.0);

        assert!(
            reward_with_main_engine < reward_bad,
            "Using main engine should penalize reward"
        );
        assert!(
            reward_with_side_engine < reward_bad,
            "Using side engine should penalize reward"
        );
    }

    #[test]
    fn test_wind_application() {
        // Ensure enabling wind doesn't crash and affects simulation implicitly
        let config = get_test_config().with_wind_strength(10.0);
        let mut env = LunarLander::new(config).unwrap();

        // We can't easily inspect the internal force applied without mocking Rapier,
        // but we can ensure the step runs without error and internal indices update.

        assert_eq!(env.wind_idx, 0.0);
        env.step(MixedItem::Discrete(0)).unwrap();
        assert_eq!(env.wind_idx, 1.0);
    }
}
