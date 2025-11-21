use crate::env::rapier::world::PhysicsWorld as GeneralPhysicsWorld;
use rapier2d::prelude::*;

pub trait BipedalWalkerPhysicsExt {
    fn clear_collisions(&self);
}

impl BipedalWalkerPhysicsExt for GeneralPhysicsWorld {
    fn clear_collisions(&self) {
        while self.collision_recv.try_recv().is_ok() {}
    }
}

#[derive(Debug, Clone)]
pub struct LegData {
    pub handle: RigidBodyHandle,
    pub ground_contact: bool,
}

impl LegData {
    pub fn new(handle: RigidBodyHandle) -> Self {
        Self {
            handle,
            ground_contact: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rapier2d::crossbeam::channel::{unbounded, Receiver, Sender};

    struct MockPhysicsWorld {
        pub collision_recv: Receiver<CollisionEvent>,
        #[allow(dead_code)] // Kept to prevent sender from dropping immediately
        pub collision_send: Sender<CollisionEvent>,
    }

    impl MockPhysicsWorld {
        fn new() -> Self {
            let (collision_send, collision_recv) = unbounded();
            Self {
                collision_recv,
                collision_send,
            }
        }
    }

    impl BipedalWalkerPhysicsExt for MockPhysicsWorld {
        fn clear_collisions(&self) {
            while self.collision_recv.try_recv().is_ok() {}
        }
    }

    #[test]
    fn test_leg_data_new() {
        let handle = RigidBodyHandle::from_raw_parts(1, 0);
        let leg_data = LegData::new(handle);

        assert_eq!(leg_data.handle, handle);
        assert!(!leg_data.ground_contact);
    }

    #[test]
    fn test_leg_data_debug_clone() {
        let handle = RigidBodyHandle::from_raw_parts(5, 0);
        let leg_data = LegData::new(handle);
        let cloned = leg_data.clone();

        assert_eq!(format!("{:?}", leg_data), format!("{:?}", cloned));
        assert_eq!(cloned.handle, handle);
        assert_eq!(cloned.ground_contact, leg_data.ground_contact);
    }

    #[test]
    fn test_leg_data_modification() {
        let handle = RigidBodyHandle::from_raw_parts(10, 0);
        let mut leg_data = LegData::new(handle);

        assert!(!leg_data.ground_contact);
        leg_data.ground_contact = true;
        assert!(leg_data.ground_contact);
    }

    #[test]
    fn test_clear_collisions_empty_queue() {
        let world = MockPhysicsWorld::new();

        world.clear_collisions();
        assert!(world.collision_recv.is_empty());
    }

    #[test]
    fn test_clear_collisions_drains_queue() {
        let world = MockPhysicsWorld::new();

        let c1 = CollisionEvent::Started(
            ColliderHandle::from_raw_parts(0, 0),
            ColliderHandle::from_raw_parts(0, 1),
            CollisionEventFlags::SENSOR,
        );
        let c2 = CollisionEvent::Stopped(
            ColliderHandle::from_raw_parts(0, 0),
            ColliderHandle::from_raw_parts(0, 1),
            CollisionEventFlags::SENSOR,
        );

        world.collision_send.send(c1).unwrap();
        world.collision_send.send(c2).unwrap();

        assert!(!world.collision_recv.is_empty());
        assert_eq!(world.collision_recv.len(), 2);

        world.clear_collisions();

        assert!(world.collision_recv.is_empty());
        assert_eq!(world.collision_recv.len(), 0);
    }
}
