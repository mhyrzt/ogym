use nalgebra::DVector;
use ogym::env::environment::Environment;
use ogym::env::mujoco::pusher::{MujocoPusherEnv, PusherConfig};
use std::time::Instant;

pub fn main() {
    let config = PusherConfig::default();
    let mut env = MujocoPusherEnv::new(Some(config)).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::PHYSICS_STEPS {
        // Reduced iterations for physics-based environment
        // Assuming action dimension of 7 for Pusher based on typical mujoco pusher env
        let action = DVector::<f64>::from_vec(vec![0.0; 7]);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Pusher (ogym): {:?}", duration);
}
