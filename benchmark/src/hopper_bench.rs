use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::hopper::{HopperConfig, MujocoHopperEnv};
use std::time::Instant;

pub fn main() {
    let config = HopperConfig::default();
    let mut env = MujocoHopperEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::PHYSICS_STEPS {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 3 for Hopper based on typical mujoco hopper env
        let action = DVector::<f64>::from_vec(vec![0.0; 3]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Hopper (ogym): {:?}", duration);
}
