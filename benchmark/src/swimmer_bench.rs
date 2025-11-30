use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::swimmer::{SwimmerConfig, MujocoSwimmerEnv};
use std::time::Instant;

pub fn main() {
    let config = SwimmerConfig::default();
    let mut env = MujocoSwimmerEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 2 for Swimmer based on typical mujoco swimmer env
        let action = DVector::<f64>::from_vec(vec![0.0; 2]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Swimmer (ogym): {:?}", duration);
}