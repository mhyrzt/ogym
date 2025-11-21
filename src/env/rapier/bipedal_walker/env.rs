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

    hull_handle: RigidBodyHandle,
    leg_handles: Vec<RigidBodyHandle>,
    joint_handles: Vec<ImpulseJointHandle>,
    terrain_handles: Vec<RigidBodyHandle>,

    legs: Vec<LegData>,

    _state: SVector<f32, STATE_SIZE>,
    steps: u32,
    prev_shaping: Option<f32>,
    game_over: bool,
    scroll: f32,

    terrain_x: Vec<f32>,
    terrain_y: Vec<f32>,

    rng: rand::rngs::StdRng,

    lidar_fractions: Vec<f32>,
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
            _state: SVector::zeros(),
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
                self.config.terrain_step * self.config.terrain_startpad as f32 / 2.0,
                self.config.terrain_height + 2.0 * self.config.leg_h
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

    fn compute_state(&self) -> SVector<f32, STATE_SIZE> {
        let hull = self.world.rigid_body_set[self.hull_handle].clone();
        let linear_velocity = hull.linvel();
        let angular_velocity = hull.angvel();
        let angle = hull.rotation().angle();
        let mut state_vec = vec![
            angle,
            2.0 * angular_velocity / self.config.fps as f32,
            0.3 * linear_velocity.x * (self.config.viewport_w / self.config.scale)
                / self.config.fps as f32,
            0.3 * linear_velocity.y * (self.config.viewport_h / self.config.scale)
                / self.config.fps as f32,
        ];
        state_vec.extend(self.lidar_fractions.clone());
        let mut out = SVector::<f32, STATE_SIZE>::zeros();
        for (i, val) in state_vec.iter().take(STATE_SIZE).enumerate() {
            out[i] = *val;
        }
        out
    }

    fn compute_reward(&mut self, pos_x: f32, angle: f32, action: &[f32]) -> f32 {
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
    type Action = SVector<f32, 4>;
    type State = SVector<f32, STATE_SIZE>;
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

        for (&a, &joint_handle) in action.iter().zip(&self.joint_handles) {
            if let Some(joint) = self.world.impulse_joint_set.get_mut(joint_handle, true) {
                let speed = if self.config.control_speed {
                    self.config.speed_hip * a.clamp(-1.0, 1.0)
                } else {
                    self.config.speed_hip * a.signum()
                };
                joint
                    .data
                    .set_motor_velocity(JointAxis::AngX, speed, self.config.motors_torque);
            }
        }

        self.world.step_with_dt(1.0 / self.config.fps as f32);
        self.steps += 1;

        let hull = &self.world.rigid_body_set[self.hull_handle];
        let pos = hull.translation();
        let angle = hull.rotation().angle();
        let reward = self.compute_reward(pos.x, angle, action.as_slice()) as f64;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::environment::Environment;
    use nalgebra::SVector;

    fn standard_config() -> BipedalWalkerConfig {
        BipedalWalkerConfig::default()
    }

    #[test]
    fn test_bipedal_walker_instantiation() {
        let config = standard_config();
        let walker = BipedalWalker::new(config.clone());

        assert_eq!(walker.steps, 0);
        assert!(!walker.game_over);
        assert_eq!(walker.lidar_fractions.len(), 10);

        assert_eq!(walker.hull_handle, RigidBodyHandle::invalid());
        assert!(walker.leg_handles.is_empty());
    }

    #[test]
    fn test_reset_initializes_physics_world() {
        let mut walker = BipedalWalker::new(standard_config());
        let (initial_state, _) = walker.reset(Some(42)).expect("Reset failed");

        assert_eq!(initial_state.len(), STATE_SIZE);

        assert_ne!(walker.hull_handle, RigidBodyHandle::invalid());
        assert!(!walker.terrain_handles.is_empty());

        assert_eq!(walker.terrain_x.len(), walker.config.terrain_length);
        assert_eq!(walker.terrain_y.len(), walker.config.terrain_length);
    }

    #[test]
    fn test_step_increases_counters() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(None).expect("Reset failed");

        let initial_steps = walker.steps;
        let action = SVector::<f32, 4>::zeros();

        let experience = walker.step(action).expect("Step failed");

        assert_eq!(walker.steps, initial_steps + 1);
        assert_eq!(experience.step, initial_steps + 1);
    }

    #[test]
    fn test_deterministic_seeding() {
        let mut walker1 = BipedalWalker::new(standard_config());
        let mut walker2 = BipedalWalker::new(standard_config());

        let seed = 12345;
        walker1.reset(Some(seed)).unwrap();
        walker2.reset(Some(seed)).unwrap();

        assert_eq!(walker1.terrain_y, walker2.terrain_y);

        let state1 = walker1.state().unwrap();
        let state2 = walker2.state().unwrap();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_different_seeds_produce_different_terrains() {
        let mut walker1 = BipedalWalker::new(standard_config());
        let mut walker2 = BipedalWalker::new(standard_config());

        walker1.reset(Some(100)).unwrap();
        walker2.reset(Some(200)).unwrap();

        assert_ne!(walker1.terrain_y, walker2.terrain_y);
    }

    #[test]
    fn test_episode_truncation_limit() {
        let mut config = standard_config();
        config.max_episode_steps = 5;
        let mut walker = BipedalWalker::new(config);

        walker.reset(None).unwrap();
        assert!(!walker.is_truncated());

        for _ in 0..5 {
            walker.step(SVector::zeros()).unwrap();
        }

        assert!(walker.is_truncated());

        assert!(walker.is_terminal().unwrap());
    }

    #[test]
    fn test_hardcore_mode_config() {
        let config = standard_config().with_hardcore(true);
        let mut walker = BipedalWalker::new(config);

        walker.reset(Some(1)).unwrap();

        assert!(!walker.terrain_handles.is_empty());
    }

    #[test]
    fn test_compute_state_vector_integrity() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(None).unwrap();

        let state = walker.compute_state();

        assert_eq!(state[4], 1.0);
        assert_eq!(state[13], 1.0);

        assert_eq!(state.len(), STATE_SIZE);
    }

    #[test]
    fn test_destroy_world_cleanup() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(None).unwrap();
        walker.reset(None).unwrap();
        assert_ne!(walker.hull_handle, RigidBodyHandle::invalid());
        assert!(walker.world.rigid_body_set.contains(walker.hull_handle));
    }

    #[test]
    fn test_motor_control_clamping() {
        let config = standard_config().with_control_speed(true);
        let mut walker = BipedalWalker::new(config);
        walker.reset(None).unwrap();
        let action = SVector::<f32, 4>::new(2.0, -2.0, 0.0, 0.0);

        let res = walker.step(action);
        assert!(res.is_ok());
    }
}
