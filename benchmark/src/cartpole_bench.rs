use ogym::env::control::cart_pole::{CartPole, CartPoleConfig};
use ogym::env::environment::Environment;
use ogym::spaces::MixedItem;
use std::time::Instant;

pub fn main() {
    let config = CartPoleConfig::default();
    let mut env = CartPole::new(config).unwrap();
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::LIGHTWEIGHT_STEPS {
        let action = MixedItem::Discrete(0);
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }
    let duration = start.elapsed();
    println!("CartPole (ogym): {:?}", duration);
}
