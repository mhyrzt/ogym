use crate::env::environment::{Environment, Error, Experience};
use crate::env::rapier::world::PhysicsWorld as GeneralPhysicsWorld;
use nalgebra::{Point2, SVector};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rapier2d::prelude::*;

use super::config::BipedalWalkerConfig;
use super::terrain::TerrainGenerator;
use super::utils::{BipedalWalkerPhysicsExt, LegData};

const STATE_SIZE: usize = 24;

pub struct BipedalWalker {
    config: BipedalWalkerConfig,
    world: GeneralPhysicsWorld,

    // Entity handles
    hull_handle: RigidBodyHandle,
    leg_handles: Vec<RigidBodyHandle>,
    joint_handles: Vec<ImpulseJointHandle>,
    terrain_handles: Vec<RigidBodyHandle>,

    legs: Vec<LegData>,

    state: SVector<f64, STATE_SIZE>,
    steps: u32,
    prev_shaping: Option<f64>,
    game_over: bool,
    scroll: f64,

    terrain_x: Vec<f64>,
    terrain_y: Vec<f64>,

    rng: rand::rngs::StdRng,

    lidar_fractions: Vec<f64>,
}

impl BipedalWalker {
    pub fn new(config: BipedalWalkerConfig) -> Self {
        let world = GeneralPhysicsWorld::new(-10.0);

        Self {
            config,
            world,
            hull_handle: RigidBodyHandle::invalid(),
            leg_handles: Vec::new(),
            joint_handles: Vec::new(),
            terrain_handles: Vec::new(),
            legs: Vec::new(),
            state: SVector::zeros(),
            steps: 0,
            prev_shaping: None,
            game_over: false,
            scroll: 0.0,
            terrain_x: Vec::new(),
            terrain_y: Vec::new(),
            rng: StdRng::seed_from_u64(10),
            lidar_fractions: vec![1.0; 10],
        }
    }

    fn destroy_world(&mut self) {
        // Clear all bodies
        for handle in self.terrain_handles.drain(..) {
            if self.world.rigid_body_set.contains(handle) {
                self.world.rigid_body_set.remove(
                    handle,
                    &mut self.world.island_manager,
                    &mut self.world.collider_set,
                    &mut self.world.impulse_joint_set,
                    &mut self.world.multibody_joint_set,
                    true,
                );
            }
        }

        if self.world.rigid_body_set.contains(self.hull_handle) {
            self.world.rigid_body_set.remove(
                self.hull_handle,
                &mut self.world.island_manager,
                &mut self.world.collider_set,
                &mut self.world.impulse_joint_set,
                &mut self.world.multibody_joint_set,
                true,
            );
        }

        for handle in self.leg_handles.drain(..) {
            if self.world.rigid_body_set.contains(handle) {
                self.world.rigid_body_set.remove(
                    handle,
                    &mut self.world.island_manager,
                    &mut self.world.collider_set,
                    &mut self.world.impulse_joint_set,
                    &mut self.world.multibody_joint_set,
                    true,
                );
            }
        }
        self.joint_handles.clear();
        self.legs.clear();
    }

    fn generate_terrain(&mut self) {
        let generator = TerrainGenerator::new(self.config.clone());
        let (handles, x, y) = generator.generate(
            &mut self.rng,
            &mut self.world.rigid_body_set,
            &mut self.world.collider_set,
            self.config.hardcore,
        );
        self.terrain_handles = handles;
        self.terrain_x = x;
        self.terrain_y = y;
    }

    fn create_hull(&mut self) {
        let hull_vertices = self.config.get_scaled_hull_vertices();
        let poly_points: Vec<Point<Real>> = hull_vertices
            .iter()
            .map(|v| Point2::new(v.x, v.y))
            .collect();
        let collider = ColliderBuilder::convex_hull(&poly_points)
            .unwrap()
            .density(5.0)
            .build();
        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![
                (self.config.terrain_step * self.config.terrain_startpad as f64 / 2.0) as f32,
                (self.config.terrain_height + 2.0 * self.config.leg_h) as f32
            ])
            .build();
        let handle = self.world.rigid_body_set.insert(rb);
        self.world.collider_set.insert_with_parent(
            collider,
            handle,
            &mut self.world.rigid_body_set,
        );
        self.hull_handle = handle;
    }

    pub fn reset_env(&mut self) {
        self.destroy_world();
        self.world.clear_collisions();
        self.game_over = false;
        self.prev_shaping = None;
        self.scroll = 0.0;
        self.steps = 0;
        self.generate_terrain();
        self.create_hull();
    }

    fn compute_state(&self) -> SVector<f64, STATE_SIZE> {
        // Construct the flat feature vector like the Gym version.
        let hull = self.world.rigid_body_set[self.hull_handle].clone();
        let linear_velocity = hull.linvel();
        let angular_velocity = hull.angvel();
        let angle = hull.rotation().angle();
        let mut state_vec = vec![
            angle as f64,
            2.0 * angular_velocity as f64 / self.config.fps as f64,
            0.3 * linear_velocity.x as f64 * (self.config.viewport_w / self.config.scale)
                / self.config.fps as f64,
            0.3 * linear_velocity.y as f64 * (self.config.viewport_h / self.config.scale)
                / self.config.fps as f64,
        ];
        // Append simplified joint and lidar features placeholders
        state_vec.extend(self.lidar_fractions.clone());
        let mut out = SVector::<f64, STATE_SIZE>::zeros();
        for (i, val) in state_vec.iter().take(STATE_SIZE).enumerate() {
            out[i] = *val;
        }
        out
    }

    fn compute_reward(&mut self, pos_x: f64, angle: f64, action: &[f64]) -> f64 {
        let mut reward = 0.0;
        let mut shaping = 130.0 * pos_x / self.config.scale;
        shaping -= 5.0 * angle.abs();

        if let Some(prev) = self.prev_shaping {
            reward = shaping - prev;
        }
        self.prev_shaping = Some(shaping);

        for &a in action {
            reward -= 0.00035 * self.config.motors_torque * a.abs().clamp(0.0, 1.0);
        }

        reward
    }
}

impl Environment for BipedalWalker {
    type Action = SVector<f64, 4>;
    type State = SVector<f64, STATE_SIZE>;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        if let Some(s) = seed {
            self.rng = rand::rngs::StdRng::seed_from_u64(s);
        }
        self.reset_env();
        Ok((self.compute_state(), ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let curr_state = self.compute_state();

        // Apply simplified actuation
        for (&a, &joint_handle) in action.iter().zip(&self.joint_handles) {
            if let Some(joint) = self.world.impulse_joint_set.get_mut(joint_handle, true) {
                let speed = if self.config.control_speed {
                    self.config.speed_hip * a.clamp(-1.0, 1.0)
                } else {
                    self.config.speed_hip * a.signum()
                };
                joint.data.set_motor_velocity(
                    JointAxis::AngX,
                    speed as f32,
                    self.config.motors_torque as f32,
                );
            }
        }

        self.world.step_with_dt(1.0 / self.config.fps as f32);
        self.steps += 1;

        // Compute basic reward and termination
        let hull = &self.world.rigid_body_set[self.hull_handle];
        let pos = hull.translation();
        let angle = hull.rotation().angle() as f64;
        let reward = self.compute_reward(pos.x as f64, angle, action.as_slice());
        let next_state = self.compute_state();
        let terminal = self.to_terminal()?;

        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            (),
            terminal,
            self.steps,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(self.game_over || self.steps >= self.config.max_episode_steps)
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        Ok(self.compute_state())
    }
}
