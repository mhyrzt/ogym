use nalgebra::SVector;
use ogym::env::control::pendulum::{Pendulum, PendulumConfig};
use ogym::env::environment::Environment;
use ogym::spaces::MixedItem;
use std::time::Instant;

pub fn main() {
    let config = PendulumConfig::default();
    let mut env = Pendulum::new(config).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..10000 {
        let action = MixedItem::Continuous(SVector::from_element(0.0));
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("Pendulum (ogym): {:?}", duration);
}
