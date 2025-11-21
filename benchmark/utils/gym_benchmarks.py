import gymnasium as gym
import time
import numpy as np
import sys


def benchmark_acrobot():
    env = gym.make("Acrobot-v1")
    env.reset(seed=42)

    start = time.time()
    for _ in range(10000):
        action = env.action_space.sample()  # Random action
        obs, reward, terminated, truncated, info = env.step(action)
        if terminated or truncated:
            env.reset()

    duration = time.time() - start
    print(f"Acrobot (gymnasium): {duration:.4f}s")
    env.close()


def benchmark_cartpole():
    env = gym.make("CartPole-v1")
    env.reset(seed=42)

    start = time.time()
    for _ in range(10000):
        action = env.action_space.sample()  # Random action
        obs, reward, terminated, truncated, info = env.step(action)
        if terminated or truncated:
            env.reset()

    duration = time.time() - start
    print(f"CartPole (gymnasium): {duration:.4f}s")
    env.close()


def benchmark_mountain_car():
    env = gym.make("MountainCar-v0")
    env.reset(seed=42)

    start = time.time()
    for _ in range(10000):
        action = env.action_space.sample()  # Random action
        obs, reward, terminated, truncated, info = env.step(action)
        if terminated or truncated:
            env.reset()

    duration = time.time() - start
    print(f"MountainCar (gymnasium): {duration:.4f}s")
    env.close()


def benchmark_pendulum():
    env = gym.make("Pendulum-v1")
    env.reset(seed=42)

    start = time.time()
    for _ in range(10000):
        action = env.action_space.sample()  # Random action
        obs, reward, terminated, truncated, info = env.step(action)
        if terminated or truncated:
            env.reset()

    duration = time.time() - start
    print(f"Pendulum (gymnasium): {duration:.4f}s")
    env.close()


def main():
    if len(sys.argv) < 2:
        print("Usage: python gym_benchmarks.py <environment_name>")
        print("Available environments: acrobot, cartpole, mountain_car, pendulum, all")
        return

    env_name = sys.argv[1]

    if env_name == "acrobot":
        print("Benchmarking Acrobot (gymnasium)...")
        benchmark_acrobot()
    elif env_name == "cartpole":
        print("Benchmarking CartPole (gymnasium)...")
        benchmark_cartpole()
    elif env_name == "mountain_car":
        print("Benchmarking MountainCar (gymnasium)...")
        benchmark_mountain_car()
    elif env_name == "pendulum":
        print("Benchmarking Pendulum (gymnasium)...")
        benchmark_pendulum()
    elif env_name == "all":
        print("Benchmarking all environments (gymnasium)...")
        print("Benchmarking Acrobot (gymnasium)...")
        benchmark_acrobot()
        print("Benchmarking CartPole (gymnasium)...")
        benchmark_cartpole()
        print("Benchmarking MountainCar (gymnasium)...")
        benchmark_mountain_car()
        print("Benchmarking Pendulum (gymnasium)...")
        benchmark_pendulum()
    else:
        print(f"Unknown environment: {env_name}")
        print("Available environments: acrobot, cartpole, mountain_car, pendulum, all")


if __name__ == "__main__":
    main()
