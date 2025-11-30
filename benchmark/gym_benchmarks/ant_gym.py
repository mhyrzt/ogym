import gymnasium as gym
import time
import numpy as np


def benchmark_ant():
    try:
        env = gym.make("Ant-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Ant (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Ant (gymnasium): Skipped - {str(e)}")


if __name__ == "__main__":
    benchmark_ant()