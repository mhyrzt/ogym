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
mod walker2d_bench;

use std::env;

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
        "cartpole" => {
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
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
        "all" => {
            println!("Benchmarking all environments (ogym)...");
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
            println!("Benchmarking Ant (ogym)...");
            ant_bench::main();
            println!("Benchmarking BipedalWalker (ogym)...");
            bipedal_walker_bench::main();
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
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
