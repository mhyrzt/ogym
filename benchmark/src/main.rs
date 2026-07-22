mod acrobot_bench;
#[cfg(feature = "mujoco")]
mod ant_bench;
mod bipedal_walker_bench;
mod cartpole_bench;
#[cfg(feature = "mujoco")]
mod half_cheetah_bench;
#[cfg(feature = "mujoco")]
mod hopper_bench;
#[cfg(feature = "mujoco")]
mod humanoid_bench;
#[cfg(feature = "mujoco")]
mod humanoid_standup_bench;
#[cfg(feature = "mujoco")]
mod inverted_double_pendulum_bench;
#[cfg(feature = "mujoco")]
mod inverted_pendulum_bench;
mod lunar_lander_bench;
mod mountain_car_bench;
mod pendulum_bench;
#[cfg(feature = "mujoco")]
mod pusher_bench;
#[cfg(feature = "mujoco")]
mod reacher_bench;
#[cfg(feature = "mujoco")]
mod swimmer_bench;
mod toy_text_bench;
#[cfg(feature = "mujoco")]
mod walker2d_bench;

use std::env;

pub(crate) const LIGHTWEIGHT_STEPS: usize = 100_000;
pub(crate) const PHYSICS_STEPS: usize = 5_000;

fn available_environments() -> &'static str {
    #[cfg(feature = "mujoco")]
    {
        "acrobot, ant, bipedal_walker, blackjack, cartpole, cliff_walking, frozen_lake, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, taxi, walker2d, all"
    }
    #[cfg(not(feature = "mujoco"))]
    {
        "acrobot, bipedal_walker, blackjack, cartpole, cliff_walking, frozen_lake, lunar_lander, mountain_car, pendulum, taxi, all"
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <environment_name>");
        println!("Available environments: {}", available_environments());
        return;
    }

    let env_name = &args[1];

    match env_name.as_str() {
        "acrobot" => {
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
        }
        #[cfg(feature = "mujoco")]
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
        #[cfg(feature = "mujoco")]
        "half_cheetah" => {
            println!("Benchmarking HalfCheetah (ogym)...");
            half_cheetah_bench::main();
        }
        #[cfg(feature = "mujoco")]
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
        #[cfg(feature = "mujoco")]
        "walker2d" => {
            println!("Benchmarking Walker2d (ogym)...");
            walker2d_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "humanoid" => {
            println!("Benchmarking Humanoid (ogym)...");
            humanoid_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "humanoid_standup" => {
            println!("Benchmarking HumanoidStandup (ogym)...");
            humanoid_standup_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "inverted_double_pendulum" => {
            println!("Benchmarking InvertedDoublePendulum (ogym)...");
            inverted_double_pendulum_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "inverted_pendulum" => {
            println!("Benchmarking InvertedPendulum (ogym)...");
            inverted_pendulum_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "pusher" => {
            println!("Benchmarking Pusher (ogym)...");
            pusher_bench::main();
        }
        #[cfg(feature = "mujoco")]
        "reacher" => {
            println!("Benchmarking Reacher (ogym)...");
            reacher_bench::main();
        }
        #[cfg(feature = "mujoco")]
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
            println!("Benchmarking LunarLander (ogym)...");
            lunar_lander_bench::main();
            println!("Benchmarking MountainCar (ogym)...");
            mountain_car_bench::main();
            println!("Benchmarking Pendulum (ogym)...");
            pendulum_bench::main();
            println!("Benchmarking Taxi (ogym)...");
            toy_text_bench::taxi();
            #[cfg(feature = "mujoco")]
            {
                println!("Benchmarking Ant (ogym)...");
                ant_bench::main();
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
                println!("Benchmarking Pusher (ogym)...");
                pusher_bench::main();
                println!("Benchmarking Reacher (ogym)...");
                reacher_bench::main();
                println!("Benchmarking Swimmer (ogym)...");
                swimmer_bench::main();
                println!("Benchmarking Walker2d (ogym)...");
                walker2d_bench::main();
            }
        }
        _ => {
            println!("Unknown environment: {}", env_name);
            println!("Available environments: {}", available_environments());
        }
    }
}
