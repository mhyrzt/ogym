use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::walker2d::{MujocoWalker2dEnv, Walker2dConfig};
use std::time::Instant;

pub fn main() {
    let config = Walker2dConfig::default();
    let mut env = MujocoWalker2dEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 6 for Walker2d based on typical mujoco walker2d env
        let action = DVector::<f64>::from_vec(vec![0.0; 6]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Walker2d (ogym): {:?}", duration);
}
