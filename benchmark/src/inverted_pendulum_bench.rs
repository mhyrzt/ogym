use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::inverted_pendulum::{InvertedPendulumConfig, MujocoInvertedPendulumEnv};
use std::time::Instant;

pub fn main() {
    let config = InvertedPendulumConfig::default();
    let mut env = MujocoInvertedPendulumEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::PHYSICS_STEPS {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 1 for InvertedPendulum based on typical mujoco pendulum env
        let action = DVector::<f64>::from_vec(vec![0.0; 1]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("InvertedPendulum (ogym): {:?}", duration);
}
