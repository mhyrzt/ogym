use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::ant::{AntConfig, MujocoAntEnv};
use std::time::Instant;

pub fn main() {
    let config = AntConfig::default();
    let mut env = MujocoAntEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 8 for Ant based on typical mujoco ant env
        let action = DVector::<f64>::from_vec(vec![0.0; 8]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Ant (ogym): {:?}", duration);
}
