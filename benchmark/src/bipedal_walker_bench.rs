use nalgebra::SVector;
use ogym::env::environment::Environment;
use ogym::env::rapier::bipedal_walker::{BipedalWalker, BipedalWalkerConfig};
use std::time::Instant;

pub fn main() {
    let config = BipedalWalkerConfig::default();
    let mut env = BipedalWalker::new(config);
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        // Reduced iterations for physics-based environment
        let action = SVector::<f32, 4>::from_vec(vec![0.0, 0.0, 0.0, 0.0]); // 4 continuous values for bipedal walker
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("BipedalWalker (ogym): {:?}", duration);
}
