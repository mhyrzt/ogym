mod acrobot_bench;
mod cartpole_bench;
mod mountain_car_bench;
mod pendulum_bench;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <environment_name>");
        println!("Available environments: acrobot, cartpole, mountain_car, pendulum, all");
        return;
    }

    let env_name = &args[1];

    match env_name.as_str() {
        "acrobot" => {
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
        }
        "cartpole" => {
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
        }
        "mountain_car" => {
            println!("Benchmarking MountainCar (ogym)...");
            mountain_car_bench::main();
        }
        "pendulum" => {
            println!("Benchmarking Pendulum (ogym)...");
            pendulum_bench::main();
        }
        "all" => {
            println!("Benchmarking all environments (ogym)...");
            println!("Benchmarking Acrobot (ogym)...");
            acrobot_bench::main();
            println!("Benchmarking CartPole (ogym)...");
            cartpole_bench::main();
            println!("Benchmarking MountainCar (ogym)...");
            mountain_car_bench::main();
            println!("Benchmarking Pendulum (ogym)...");
            pendulum_bench::main();
        }
        _ => {
            println!("Unknown environment: {}", env_name);
            println!("Available environments: acrobot, cartpole, mountain_car, pendulum, all");
        }
    }
}
