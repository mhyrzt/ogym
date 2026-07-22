use ogym::env::environment::Environment;
use ogym::env::toy_text::{
    blackjack::{Blackjack, BlackjackConfig},
    cliff_walking::{CliffWalking, CliffWalkingConfig},
    frozen_lake::{FrozenLake, FrozenLakeConfig},
    taxi::{Taxi, TaxiConfig},
};
use std::time::Instant;

fn run<E>(mut env: E, action: u32, name: &str)
where
    E: Environment<Action = u32>,
{
    env.reset(Some(42)).unwrap();

    let start = Instant::now();
    for _ in 0..crate::LIGHTWEIGHT_STEPS {
        let _ = env.step(action);
        if env.is_done().unwrap() {
            env.reset(None).unwrap();
        }
    }

    println!("{name} (ogym): {:?}", start.elapsed());
}

pub fn blackjack() {
    run(
        Blackjack::new(BlackjackConfig::default()).unwrap(),
        1,
        "Blackjack",
    );
}

pub fn cliff_walking() {
    run(
        CliffWalking::new(CliffWalkingConfig::default()).unwrap(),
        1,
        "CliffWalking",
    );
}

pub fn frozen_lake() {
    run(
        FrozenLake::new(FrozenLakeConfig::default()).unwrap(),
        1,
        "FrozenLake",
    );
}

pub fn taxi() {
    run(Taxi::new(TaxiConfig::default()).unwrap(), 0, "Taxi");
}
