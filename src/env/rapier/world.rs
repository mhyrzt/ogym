use rapier2d::{
    crossbeam::{self, channel::Receiver},
    prelude::*,
};

pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: ChannelEventCollector,
    pub collision_recv: Receiver<CollisionEvent>,
    pub contact_force_recv: Receiver<ContactForceEvent>,
}

impl PhysicsWorld {
    pub fn new(gravity_y: f32) -> Self {
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            gravity: vector![0.0, gravity_y],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            event_handler,
            collision_recv,
            contact_force_recv,
        }
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub fn step_with_dt(&mut self, dt: f32) {
        let mut params = self.integration_parameters;
        params.dt = dt;
        self.physics_pipeline.step(
            &self.gravity,
            &params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub fn reset(&mut self) {
        self.rigid_body_set = RigidBodySet::new();
        self.collider_set = ColliderSet::new();
        self.impulse_joint_set = ImpulseJointSet::new();
        self.multibody_joint_set = MultibodyJointSet::new();
        self.island_manager = IslandManager::new();
        self.broad_phase = DefaultBroadPhase::new();
        self.narrow_phase = NarrowPhase::new();
        self.ccd_solver = CCDSolver::new();
        self.query_pipeline = QueryPipeline::new();
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new(-9.81)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_physics_world_initialization() {
        let gravity_y = -20.0;
        let world = PhysicsWorld::new(gravity_y);

        assert_eq!(world.gravity.x, 0.0);
        assert_eq!(world.gravity.y, gravity_y);
        assert_eq!(world.rigid_body_set.len(), 0);
        assert_eq!(world.collider_set.len(), 0);
        assert_eq!(world.impulse_joint_set.len(), 0);
        assert_eq!(world.multibody_joint_set.iter().count(), 0);
    }

    #[test]
    fn test_default_physics_world() {
        let world = PhysicsWorld::default();

        assert_eq!(world.gravity.x, 0.0);
        assert_eq!(world.gravity.y, -9.81);
    }

    #[test]
    fn test_add_rigid_body_and_collider() {
        let mut world = PhysicsWorld::default();

        let rigid_body = RigidBodyBuilder::dynamic().build();
        let handle = world.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(0.5).build();
        world
            .collider_set
            .insert_with_parent(collider, handle, &mut world.rigid_body_set);

        assert_eq!(world.rigid_body_set.len(), 1);
        assert_eq!(world.collider_set.len(), 1);
    }

    #[test]
    fn test_step_simulation() {
        let mut world = PhysicsWorld::default();

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let handle = world.rigid_body_set.insert(rigid_body);

        // Add a collider to give the body mass
        let collider = ColliderBuilder::ball(0.5).build();
        world
            .collider_set
            .insert_with_parent(collider, handle, &mut world.rigid_body_set);

        // Run a few steps to ensure gravity has visibly affected the body
        for _ in 0..10 {
            world.step();
        }

        let body = world.rigid_body_set.get(handle).unwrap();
        assert!(body.translation().y < 10.0);
    }

    #[test]
    fn test_step_with_dt_simulation() {
        let mut world = PhysicsWorld::default();

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .linvel(vector![0.0, 0.0])
            .build();
        let handle = world.rigid_body_set.insert(rigid_body);

        // Add a collider to give the body mass
        let collider = ColliderBuilder::ball(0.5).build();
        world
            .collider_set
            .insert_with_parent(collider, handle, &mut world.rigid_body_set);

        let dt = 0.1;
        world.step_with_dt(dt);

        let body = world.rigid_body_set.get(handle).unwrap();

        let expected_pos_approx = 10.0 + (-9.81 * dt * dt);

        assert!(body.translation().y < 10.0);
        assert!((body.translation().y - expected_pos_approx).abs() < 1.0);
    }

    #[test]
    fn test_reset_world() {
        let mut world = PhysicsWorld::default();

        let rigid_body = RigidBodyBuilder::dynamic().build();
        let handle = world.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(0.5).build();
        world
            .collider_set
            .insert_with_parent(collider, handle, &mut world.rigid_body_set);

        // Create a second rigid body for the joint (joints need two bodies)
        let rigid_body2 = RigidBodyBuilder::dynamic().build();
        let handle2 = world.rigid_body_set.insert(rigid_body2);

        let joint = FixedJointBuilder::new().build();
        world.impulse_joint_set.insert(handle, handle2, joint, true);

        // Note: We inserted 2 rigid bodies (handle and handle2).
        assert_eq!(world.rigid_body_set.len(), 2);
        assert_eq!(world.collider_set.len(), 1);

        world.reset();

        assert_eq!(world.rigid_body_set.len(), 0);
        assert_eq!(world.collider_set.len(), 0);
        assert_eq!(world.impulse_joint_set.len(), 0);
        assert_eq!(world.multibody_joint_set.iter().count(), 0);
    }

    #[test]
    fn test_collision_event_handling() {
        let mut world = PhysicsWorld::default();
        world.gravity = vector![0.0, 0.0];

        let rb1 = RigidBodyBuilder::dynamic()
            .translation(vector![-1.0, 0.0])
            .linvel(vector![2.0, 0.0])
            .build();
        let h1 = world.rigid_body_set.insert(rb1);
        let co1 = ColliderBuilder::ball(0.6)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        world
            .collider_set
            .insert_with_parent(co1, h1, &mut world.rigid_body_set);

        let rb2 = RigidBodyBuilder::dynamic()
            .translation(vector![1.0, 0.0])
            .linvel(vector![-2.0, 0.0])
            .build();
        let h2 = world.rigid_body_set.insert(rb2);
        let co2 = ColliderBuilder::ball(0.6)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        world
            .collider_set
            .insert_with_parent(co2, h2, &mut world.rigid_body_set);

        // FIX: Increased loop count from 10 to 20.
        // Impact happens at t=0.2s (approx step 12). 10 steps was insufficient.
        for _ in 0..20 {
            world.step();
        }

        let event = world.collision_recv.try_recv();
        assert!(event.is_ok(), "Collision event queue was empty");

        if let Ok(CollisionEvent::Started(_, _, _)) = event {
            // Expected behavior
        } else {
            panic!("Expected a Started collision event, got {:?}", event);
        }
    }
}
