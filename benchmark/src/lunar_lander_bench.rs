use ogym::env::environment::Environment;
use ogym::env::rapier::lunar_lander::{LunarLander, LunarLanderConfig};
use ogym::spaces::MixedItem;
use std::time::Instant;

pub fn main() {
    let config = LunarLanderConfig::default();
    let mut env = LunarLander::new(config).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..1000 {
        let action = MixedItem::Discrete(0);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("LunarLander (ogym): {:?}", duration);
}
