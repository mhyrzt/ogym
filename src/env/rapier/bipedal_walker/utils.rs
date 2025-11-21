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
