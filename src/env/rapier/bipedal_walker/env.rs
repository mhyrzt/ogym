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
        let lidar_fractions = vec![1.0; config.lidar_count];

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
            lidar_fractions,
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

    fn spawn_xy(&self) -> (f32, f32) {
        (
            self.config.terrain_step * self.config.terrain_startpad as f32 / 2.0,
            self.config.terrain_height + 2.0 * self.config.leg_h,
        )
    }

    fn create_hull(&mut self) {
        let (init_x, init_y) = self.spawn_xy();
        let hull_vertices = self.config.get_scaled_hull_vertices();
        let poly_points: Vec<Point<Real>> = hull_vertices
            .iter()
            .map(|v| Point2::new(v.x, v.y))
            .collect();
        let collider = ColliderBuilder::convex_hull(&poly_points)
            .unwrap()
            .density(5.0)
            .friction(0.1)
            .collision_groups(InteractionGroups::new(0x0020.into(), 0x0001.into()))
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![init_x, init_y])
            .build();
        let handle = self.world.rigid_body_set.insert(rb);
        self.world.collider_set.insert_with_parent(
            collider,
            handle,
            &mut self.world.rigid_body_set,
        );
        self.hull_handle = handle;
    }

    /// Builds the two legs (upper+lower segment each) and their hip/knee revolute
    /// joints, mirroring Gymnasium's leg order: [hip(-1), knee(-1), hip(+1), knee(+1)].
    /// This order is what step()'s action-to-joint zip relies on.
    fn create_legs(&mut self) {
        let (init_x, init_y) = self.spawn_xy();
        let leg_h = self.config.leg_h;
        let leg_w = self.config.leg_w;
        let leg_down = self.config.leg_down;
        let torque = self.config.motors_torque;

        self.leg_handles.clear();
        self.joint_handles.clear();
        self.legs.clear();

        for &i in &[-1.0f32, 1.0f32] {
            let angle = i * 0.05;

            let upper_pos = Isometry::new(vector![init_x, init_y - leg_h / 2.0 - leg_down], angle);
            let upper_body = RigidBodyBuilder::dynamic().position(upper_pos).build();
            let upper_handle = self.world.rigid_body_set.insert(upper_body);
            let upper_collider = ColliderBuilder::cuboid(leg_w / 2.0, leg_h / 2.0)
                .density(1.0)
                .friction(0.2)
                .restitution(0.0)
                .active_events(ActiveEvents::COLLISION_EVENTS)
                .collision_groups(InteractionGroups::new(0x0020.into(), 0x0001.into()))
                .build();
            self.world.collider_set.insert_with_parent(
                upper_collider,
                upper_handle,
                &mut self.world.rigid_body_set,
            );

            let hip_joint = RevoluteJointBuilder::new()
                .local_anchor1(point![0.0, leg_down])
                .local_anchor2(point![0.0, leg_h / 2.0])
                .motor_velocity(0.0, torque)
                .motor_max_force(torque)
                .limits([-0.8, 1.1])
                .contacts_enabled(false)
                .build();
            let hip_handle = self.world.impulse_joint_set.insert(
                self.hull_handle,
                upper_handle,
                hip_joint,
                true,
            );

            self.leg_handles.push(upper_handle);
            self.joint_handles.push(hip_handle);
            self.legs.push(LegData::new(upper_handle));

            let lower_pos = Isometry::new(vector![init_x, init_y - leg_h * 1.5 - leg_down], angle);
            let lower_body = RigidBodyBuilder::dynamic().position(lower_pos).build();
            let lower_handle = self.world.rigid_body_set.insert(lower_body);
            let lower_collider = ColliderBuilder::cuboid(leg_w / 2.0, leg_h / 2.0)
                .density(1.0)
                .friction(0.2)
                .restitution(0.0)
                .active_events(ActiveEvents::COLLISION_EVENTS)
                .collision_groups(InteractionGroups::new(0x0020.into(), 0x0001.into()))
                .build();
            self.world.collider_set.insert_with_parent(
                lower_collider,
                lower_handle,
                &mut self.world.rigid_body_set,
            );

            let knee_joint = RevoluteJointBuilder::new()
                .local_anchor1(point![0.0, -leg_h / 2.0])
                .local_anchor2(point![0.0, leg_h / 2.0])
                .motor_velocity(1.0, torque)
                .motor_max_force(torque)
                .limits([-1.6, -0.1])
                .contacts_enabled(false)
                .build();
            let knee_handle =
                self.world
                    .impulse_joint_set
                    .insert(upper_handle, lower_handle, knee_joint, true);

            self.leg_handles.push(lower_handle);
            self.joint_handles.push(knee_handle);
            self.legs.push(LegData::new(lower_handle));
        }
    }

    pub fn reset_env(&mut self) {
        self.destroy_world();
        self.world.clear_collisions();
        self.game_over = false;
        self.prev_shaping = None;
        self.scroll = 0.0;
        self.steps = 0;
        self.lidar_fractions = vec![1.0; self.config.lidar_count];
        self.generate_terrain();
        self.create_hull();
        self.create_legs();
    }

    fn joint_relative_angle_speed(
        &self,
        parent: RigidBodyHandle,
        child: RigidBodyHandle,
    ) -> (f32, f32) {
        let parent_body = &self.world.rigid_body_set[parent];
        let child_body = &self.world.rigid_body_set[child];
        let angle = child_body.rotation().angle() - parent_body.rotation().angle();
        let speed = child_body.angvel() - parent_body.angvel();
        (angle, speed)
    }

    /// Casts one ray per lidar_count around the hull, keeping only terrain hits
    /// (excludes the hull itself and, via collision groups, the legs).
    fn update_lidar(&mut self) {
        let hull_handle = self.hull_handle;
        let origin = *self.world.rigid_body_set[hull_handle].translation();
        let lidar_range = self.config.lidar_range;
        let count = self.config.lidar_count.min(self.lidar_fractions.len());

        for (i, fraction) in self.lidar_fractions.iter_mut().enumerate().take(count) {
            let angle = 1.5 * i as f32 / self.config.lidar_count as f32;
            let dir = vector![angle.sin(), -angle.cos()];
            let ray = Ray::new(point![origin.x, origin.y], dir);
            let filter = QueryFilter::new()
                .exclude_rigid_body(hull_handle)
                .groups(InteractionGroups::new(Group::ALL, Group::GROUP_1));

            *fraction = match self.world.query_pipeline.cast_ray(
                &self.world.rigid_body_set,
                &self.world.collider_set,
                &ray,
                lidar_range,
                true,
                filter,
            ) {
                Some((_, toi)) => (toi / lidar_range).clamp(0.0, 1.0),
                None => 1.0,
            };
        }
    }

    fn apply_collision(&mut self, h1: ColliderHandle, h2: ColliderHandle, started: bool) {
        let parents = (
            self.world.collider_set.get(h1).and_then(|c| c.parent()),
            self.world.collider_set.get(h2).and_then(|c| c.parent()),
        );
        let (Some(p1), Some(p2)) = parents else {
            return;
        };

        if started && (p1 == self.hull_handle || p2 == self.hull_handle) {
            self.game_over = true;
        }

        for leg in self.legs.iter_mut() {
            if p1 == leg.handle || p2 == leg.handle {
                leg.ground_contact = started;
            }
        }
    }

    fn handle_collisions(&mut self) {
        while let Ok(event) = self.world.collision_recv.try_recv() {
            match event {
                CollisionEvent::Started(h1, h2, _) => self.apply_collision(h1, h2, true),
                CollisionEvent::Stopped(h1, h2, _) => self.apply_collision(h1, h2, false),
            }
        }
    }

    fn compute_state(&self) -> SVector<f32, STATE_SIZE> {
        let hull = self.world.rigid_body_set[self.hull_handle].clone();
        let linear_velocity = hull.linvel();
        let angular_velocity = hull.angvel();
        let angle = hull.rotation().angle();
        let fps = self.config.fps as f32;

        let (hip0_angle, hip0_speed) =
            self.joint_relative_angle_speed(self.hull_handle, self.leg_handles[0]);
        let (knee0_angle, knee0_speed) =
            self.joint_relative_angle_speed(self.leg_handles[0], self.leg_handles[1]);
        let (hip1_angle, hip1_speed) =
            self.joint_relative_angle_speed(self.hull_handle, self.leg_handles[2]);
        let (knee1_angle, knee1_speed) =
            self.joint_relative_angle_speed(self.leg_handles[2], self.leg_handles[3]);

        let mut state_vec = vec![
            angle,
            2.0 * angular_velocity / fps,
            0.3 * linear_velocity.x * (self.config.viewport_w / self.config.scale) / fps,
            0.3 * linear_velocity.y * (self.config.viewport_h / self.config.scale) / fps,
            hip0_angle,
            hip0_speed / self.config.speed_hip,
            knee0_angle + 1.0,
            knee0_speed / self.config.speed_knee,
            if self.legs[1].ground_contact {
                1.0
            } else {
                0.0
            },
            hip1_angle,
            hip1_speed / self.config.speed_hip,
            knee1_angle + 1.0,
            knee1_speed / self.config.speed_knee,
            if self.legs[3].ground_contact {
                1.0
            } else {
                0.0
            },
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

        for (idx, (&a, &joint_handle)) in action.iter().zip(&self.joint_handles).enumerate() {
            if let Some(joint) = self.world.impulse_joint_set.get_mut(joint_handle, true) {
                // Even indices are hip joints, odd indices are knee joints (see create_legs).
                let is_knee = idx % 2 == 1;
                let base_speed = if is_knee {
                    self.config.speed_knee
                } else {
                    self.config.speed_hip
                };
                if self.config.control_speed {
                    let speed = base_speed * a.clamp(-1.0, 1.0);
                    joint.data.set_motor_velocity(
                        JointAxis::AngX,
                        speed,
                        self.config.motors_torque,
                    );
                } else {
                    // f32::signum() returns 1.0 (not 0.0) for a == 0.0, which would
                    // drive every joint at full speed on a no-op action. Gymnasium
                    // avoids this by scaling max torque by |action| instead, so a
                    // zero action always ends up with zero motor authority.
                    let sign = if a > 0.0 {
                        1.0
                    } else if a < 0.0 {
                        -1.0
                    } else {
                        0.0
                    };
                    let speed = base_speed * sign;
                    let max_force = self.config.motors_torque * a.abs().clamp(0.0, 1.0);
                    joint
                        .data
                        .set_motor_velocity(JointAxis::AngX, speed, max_force);
                }
            }
        }

        self.world.step_with_dt(1.0 / self.config.fps as f32);
        self.handle_collisions();
        self.update_lidar();
        self.steps += 1;

        let hull = &self.world.rigid_body_set[self.hull_handle];
        let pos_x = hull.translation().x;
        let angle = hull.rotation().angle();
        let mut reward = self.compute_reward(pos_x, angle, action.as_slice()) as f64;
        if self.game_over || pos_x < 0.0 {
            reward = -100.0;
        }
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
        let pos_x = self.world.rigid_body_set[self.hull_handle].translation().x;
        let track_end = (self.config.terrain_length - self.config.terrain_grass) as f32
            * self.config.terrain_step;
        Ok(self.game_over || pos_x < 0.0 || pos_x > track_end)
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

        // A pure timeout is a truncation, not a termination (matches
        // Gymnasium's TimeLimit wrapper semantics: terminated stays false).
        assert!(!walker.is_terminal().unwrap());
        assert!(walker.to_terminal().unwrap().is_truncated());
        assert!(!walker.to_terminal().unwrap().is_terminated());
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

        assert_eq!(state.len(), STATE_SIZE);
        // Lidar hasn't been scanned yet (only happens during step()), so it
        // should still be at its default "nothing in range" value.
        for i in 14..STATE_SIZE {
            assert_eq!(state[i], 1.0);
        }
    }

    #[test]
    fn test_legs_and_joints_are_constructed() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(None).unwrap();

        assert_eq!(walker.leg_handles.len(), 4);
        assert_eq!(walker.joint_handles.len(), 4);
        assert_eq!(walker.legs.len(), 4);

        for &handle in &walker.leg_handles {
            assert!(walker.world.rigid_body_set.contains(handle));
        }
        for &handle in &walker.joint_handles {
            assert!(walker.world.impulse_joint_set.get(handle).is_some());
        }
    }

    #[test]
    fn test_action_drives_leg_motors() {
        // A single physics step isn't a reliable signal here: with the hull
        // and legs all dynamic and coupled, passive gravity swing can
        // transiently exceed a motor that's still ramping up from rest. Drive
        // for many steps instead and check the motor pushed the joint toward
        // its commanded limit, which passive dynamics alone won't do.
        let mut driven_positive = BipedalWalker::new(standard_config());
        driven_positive.reset(Some(1)).unwrap();
        let mut driven_negative = BipedalWalker::new(standard_config());
        driven_negative.reset(Some(1)).unwrap();

        let positive_action = SVector::<f32, 4>::new(1.0, 0.0, 0.0, 0.0);
        let negative_action = SVector::<f32, 4>::new(-1.0, 0.0, 0.0, 0.0);
        for _ in 0..40 {
            driven_positive.step(positive_action).unwrap();
            driven_negative.step(negative_action).unwrap();
        }

        let (hip0_angle_positive, _) = driven_positive.joint_relative_angle_speed(
            driven_positive.hull_handle,
            driven_positive.leg_handles[0],
        );
        let (hip0_angle_negative, _) = driven_negative.joint_relative_angle_speed(
            driven_negative.hull_handle,
            driven_negative.leg_handles[0],
        );

        assert!(
            hip0_angle_positive > hip0_angle_negative + 0.1,
            "driving hip0 with opposite-signed actions for 40 steps should push it to \
             clearly different angles: positive_action={}, negative_action={}",
            hip0_angle_positive,
            hip0_angle_negative
        );
    }

    #[test]
    fn test_lidar_reflects_real_distances() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(Some(3)).unwrap();

        walker.step(SVector::<f32, 4>::zeros()).unwrap();

        // The walker spawns a couple of leg-lengths above the terrain, so at
        // least the downward-ish rays should report something closer than
        // "nothing in range" (1.0) once the query pipeline has been scanned.
        assert!(
            walker.lidar_fractions.iter().any(|&f| f < 1.0),
            "expected at least one lidar ray to hit terrain: {:?}",
            walker.lidar_fractions
        );
        for &f in &walker.lidar_fractions {
            assert!((0.0..=1.0).contains(&f));
        }
    }

    #[test]
    fn test_fall_detection_sets_game_over() {
        let mut walker = BipedalWalker::new(standard_config());
        walker.reset(Some(7)).unwrap();

        let mut fell = false;
        for _ in 0..300 {
            walker.step(SVector::<f32, 4>::zeros()).unwrap();
            if walker.game_over {
                fell = true;
                break;
            }
        }

        assert!(
            fell,
            "an unactuated walker should topple onto its hull within 300 steps"
        );
        assert!(walker.is_terminal().unwrap());
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
