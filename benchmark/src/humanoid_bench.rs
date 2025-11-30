use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::humanoid::{HumanoidConfig, MujocoHumanoidEnv};
use std::time::Instant;

pub fn main() {
    let config = HumanoidConfig::default();
    let mut env = MujocoHumanoidEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 17 for Humanoid based on typical mujoco humanoid env
        let action = DVector::<f64>::from_vec(vec![0.0; 17]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Humanoid (ogym): {:?}", duration);
}