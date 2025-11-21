import gymnasium as gym
import time
import numpy as np


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


if __name__ == "__main__":
    benchmark_cartpole()
