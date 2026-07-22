mod acrobot_bench;
mod ant_bench;
mod bipedal_walker_bench;
mod cartpole_bench;
mod half_cheetah_bench;
mod hopper_bench;
mod humanoid_bench;
mod humanoid_standup_bench;
mod inverted_double_pendulum_bench;
mod inverted_pendulum_bench;
mod lunar_lander_bench;
mod mountain_car_bench;
mod pendulum_bench;
mod pusher_bench;
mod reacher_bench;
mod swimmer_bench;
mod toy_text_bench;
mod walker2d_bench;

use std::env;

pub(crate) const LIGHTWEIGHT_STEPS: usize = 100_000;
pub(crate) const PHYSICS_STEPS: usize = 5_000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <environment_name>");
        println!(
            "Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all"
        );
        return;
    }

    let env_name = &args[1];

    match env_name.as_str() {
        "acrobot" => {
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
        }
        "ant" => {
            println!("Benchmarking Ant (ogym)...");
            ant_bench::main();
        }
        "bipedal_walker" => {
            println!("Benchmarking BipedalWalker (ogym)...");
            bipedal_walker_bench::main();
        }
        "blackjack" => {
            println!("Benchmarking Blackjack (ogym)...");
            toy_text_bench::blackjack();
        }
        "cartpole" => {
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
        }
        "cliff_walking" => {
            println!("Benchmarking CliffWalking (ogym)...");
            toy_text_bench::cliff_walking();
        }
        "frozen_lake" => {
            println!("Benchmarking FrozenLake (ogym)...");
            toy_text_bench::frozen_lake();
        }
        "half_cheetah" => {
            println!("Benchmarking HalfCheetah (ogym)...");
            half_cheetah_bench::main();
        }
        "hopper" => {
            println!("Benchmarking Hopper (ogym)...");
            hopper_bench::main();
        }
        "lunar_lander" => {
            println!("Benchmarking LunarLander (ogym)...");
            lunar_lander_bench::main();
        }
        "mountain_car" => {
            println!("Benchmarking MountainCar (ogym)...");
            mountain_car_bench::main();
        }
        "pendulum" => {
            println!("Benchmarking Pendulum (ogym)...");
            pendulum_bench::main();
        }
        "walker2d" => {
            println!("Benchmarking Walker2d (ogym)...");
            walker2d_bench::main();
        }
        "humanoid" => {
            println!("Benchmarking Humanoid (ogym)...");
            humanoid_bench::main();
        }
        "humanoid_standup" => {
            println!("Benchmarking HumanoidStandup (ogym)...");
            humanoid_standup_bench::main();
        }
        "inverted_double_pendulum" => {
            println!("Benchmarking InvertedDoublePendulum (ogym)...");
            inverted_double_pendulum_bench::main();
        }
        "inverted_pendulum" => {
            println!("Benchmarking InvertedPendulum (ogym)...");
            inverted_pendulum_bench::main();
        }
        "pusher" => {
            println!("Benchmarking Pusher (ogym)...");
            pusher_bench::main();
        }
        "reacher" => {
            println!("Benchmarking Reacher (ogym)...");
            reacher_bench::main();
        }
        "swimmer" => {
            println!("Benchmarking Swimmer (ogym)...");
            swimmer_bench::main();
        }
        "taxi" => {
            println!("Benchmarking Taxi (ogym)...");
            toy_text_bench::taxi();
        }
        "all" => {
            println!("Benchmarking all environments (ogym)...");
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
            println!("Benchmarking Ant (ogym)...");
            ant_bench::main();
            println!("Benchmarking BipedalWalker (ogym)...");
            bipedal_walker_bench::main();
            println!("Benchmarking Blackjack (ogym)...");
            toy_text_bench::blackjack();
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
            println!("Benchmarking CliffWalking (ogym)...");
            toy_text_bench::cliff_walking();
            println!("Benchmarking FrozenLake (ogym)...");
            toy_text_bench::frozen_lake();
            println!("Benchmarking HalfCheetah (ogym)...");
            half_cheetah_bench::main();
            println!("Benchmarking Hopper (ogym)...");
            hopper_bench::main();
            println!("Benchmarking Humanoid (ogym)...");
            humanoid_bench::main();
            println!("Benchmarking HumanoidStandup (ogym)...");
            humanoid_standup_bench::main();
            println!("Benchmarking InvertedDoublePendulum (ogym)...");
            inverted_double_pendulum_bench::main();
            println!("Benchmarking InvertedPendulum (ogym)...");
            inverted_pendulum_bench::main();
            println!("Benchmarking LunarLander (ogym)...");
            lunar_lander_bench::main();
            println!("Benchmarking MountainCar (ogym)...");
            mountain_car_bench::main();
            println!("Benchmarking Pendulum (ogym)...");
            pendulum_bench::main();
            println!("Benchmarking Pusher (ogym)...");
            pusher_bench::main();
            println!("Benchmarking Reacher (ogym)...");
            reacher_bench::main();
            println!("Benchmarking Swimmer (ogym)...");
            swimmer_bench::main();
            println!("Benchmarking Taxi (ogym)...");
            toy_text_bench::taxi();
            println!("Benchmarking Walker2d (ogym)...");
            walker2d_bench::main();
        }
        _ => {
            println!("Unknown environment: {}", env_name);
            println!(
                "Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all"
            );
        }
    }
}
