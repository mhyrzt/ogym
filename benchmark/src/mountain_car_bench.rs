use ogym::env::control::mountain_car::{MountainCar, MountainCarConfig};
use ogym::env::environment::Environment;
use ogym::spaces::MixedItem;
use std::time::Instant;

pub fn main() {
    let config = MountainCarConfig::default();
    let mut env = MountainCar::new(config).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..10000 {
        let action = MixedItem::Discrete(1);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("MountainCar (ogym): {:?}", duration);
}
