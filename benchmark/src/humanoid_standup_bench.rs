use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::humanoid_standup::{HumanoidStandupConfig, MujocoHumanoidStandupEnv};
use std::time::Instant;

pub fn main() {
    let config = HumanoidStandupConfig::default();
    let mut env = MujocoHumanoidStandupEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 17 for HumanoidStandup based on typical mujoco humanoid env
        let action = DVector::<f64>::from_vec(vec![0.0; 17]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("HumanoidStandup (ogym): {:?}", duration);
}