use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::inverted_double_pendulum::{
    InvertedDoublePendulumConfig, MujocoInvertedDoublePendulumEnv,
};
use std::time::Instant;

pub fn main() {
    let config = InvertedDoublePendulumConfig::default();
    let mut env = MujocoInvertedDoublePendulumEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::PHYSICS_STEPS {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 1 for InvertedDoublePendulum based on typical mujoco pendulum env
        let action = DVector::<f64>::from_vec(vec![0.0; 1]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("InvertedDoublePendulum (ogym): {:?}", duration);
}
