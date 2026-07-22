use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::half_cheetah::{HalfCheetahConfig, MujocoHalfCheetahEnv};
use std::time::Instant;

pub fn main() {
    let config = HalfCheetahConfig::default();
    let mut env = MujocoHalfCheetahEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::PHYSICS_STEPS {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 6 for HalfCheetah based on typical mujoco half cheetah env
        let action = DVector::<f64>::from_vec(vec![0.0; 6]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("HalfCheetah (ogym): {:?}", duration);
}
