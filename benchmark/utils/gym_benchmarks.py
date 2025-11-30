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


def benchmark_bipedal_walker():
    # BipedalWalker is not a standard gymnasium environment, so we'll just print a message
    print("BipedalWalker (gymnasium): Skipped - Not available in standard gymnasium")


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


def benchmark_half_cheetah():
    try:
        env = gym.make("HalfCheetah-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"HalfCheetah (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"HalfCheetah (gymnasium): Skipped - {str(e)}")


def benchmark_hopper():
    try:
        env = gym.make("Hopper-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Hopper (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Hopper (gymnasium): Skipped - {str(e)}")


def benchmark_lunar_lander():
    env = gym.make("LunarLander-v2")
    env.reset(seed=42)

    start = time.time()
    for _ in range(1000):  # Reduced iterations to match Rust version for physics environments
        action = env.action_space.sample()  # Random action
        obs, reward, terminated, truncated, info = env.step(action)
        if terminated or truncated:
            env.reset()

    duration = time.time() - start
    print(f"LunarLander (gymnasium): {duration:.4f}s")
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


def benchmark_walker2d():
    try:
        env = gym.make("Walker2d-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Walker2d (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Walker2d (gymnasium): Skipped - {str(e)}")


def benchmark_humanoid():
    try:
        env = gym.make("Humanoid-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Humanoid (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Humanoid (gymnasium): Skipped - {str(e)}")


def benchmark_humanoid_standup():
    # HumanoidStandup is not a standard gymnasium environment, so we'll just print a message
    print("HumanoidStandup (gymnasium): Skipped - Not available in standard gymnasium")


def benchmark_inverted_double_pendulum():
    try:
        env = gym.make("InvertedDoublePendulum-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"InvertedDoublePendulum (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"InvertedDoublePendulum (gymnasium): Skipped - {str(e)}")


def benchmark_inverted_pendulum():
    try:
        env = gym.make("InvertedPendulum-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"InvertedPendulum (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"InvertedPendulum (gymnasium): Skipped - {str(e)}")


def benchmark_pusher():
    try:
        env = gym.make("Pusher-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Pusher (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Pusher (gymnasium): Skipped - {str(e)}")


def benchmark_reacher():
    try:
        env = gym.make("Reacher-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Reacher (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Reacher (gymnasium): Skipped - {str(e)}")


def benchmark_swimmer():
    try:
        env = gym.make("Swimmer-v4", disable_env_checker=True)  # Requires mujoco-py or mujoco
        env.reset(seed=42)

        start = time.time()
        for _ in range(1000):  # Reduced iterations to match Rust version
            action = env.action_space.sample()  # Random action
            obs, reward, terminated, truncated, info = env.step(action)
            if terminated or truncated:
                env.reset()

        duration = time.time() - start
        print(f"Swimmer (gymnasium): {duration:.4f}s")
        env.close()
    except Exception as e:
        print(f"Swimmer (gymnasium): Skipped - {str(e)}")


def main():
    if len(sys.argv) < 2:
        print("Usage: python gym_benchmarks.py <environment_name>")
        print("Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all")
        return

    env_name = sys.argv[1]

    if env_name == "acrobot":
        print("Benchmarking Acrobot (gymnasium)...")
        benchmark_acrobot()
    elif env_name == "ant":
        print("Benchmarking Ant (gymnasium)...")
        benchmark_ant()
    elif env_name == "bipedal_walker":
        print("Benchmarking BipedalWalker (gymnasium)...")
        benchmark_bipedal_walker()
    elif env_name == "cartpole":
        print("Benchmarking CartPole (gymnasium)...")
        benchmark_cartpole()
    elif env_name == "half_cheetah":
        print("Benchmarking HalfCheetah (gymnasium)...")
        benchmark_half_cheetah()
    elif env_name == "hopper":
        print("Benchmarking Hopper (gymnasium)...")
        benchmark_hopper()
    elif env_name == "humanoid":
        print("Benchmarking Humanoid (gymnasium)...")
        benchmark_humanoid()
    elif env_name == "humanoid_standup":
        print("Benchmarking HumanoidStandup (gymnasium)...")
        benchmark_humanoid_standup()
    elif env_name == "inverted_double_pendulum":
        print("Benchmarking InvertedDoublePendulum (gymnasium)...")
        benchmark_inverted_double_pendulum()
    elif env_name == "inverted_pendulum":
        print("Benchmarking InvertedPendulum (gymnasium)...")
        benchmark_inverted_pendulum()
    elif env_name == "lunar_lander":
        print("Benchmarking LunarLander (gymnasium)...")
        benchmark_lunar_lander()
    elif env_name == "mountain_car":
        print("Benchmarking MountainCar (gymnasium)...")
        benchmark_mountain_car()
    elif env_name == "pendulum":
        print("Benchmarking Pendulum (gymnasium)...")
        benchmark_pendulum()
    elif env_name == "pusher":
        print("Benchmarking Pusher (gymnasium)...")
        benchmark_pusher()
    elif env_name == "reacher":
        print("Benchmarking Reacher (gymnasium)...")
        benchmark_reacher()
    elif env_name == "swimmer":
        print("Benchmarking Swimmer (gymnasium)...")
        benchmark_swimmer()
    elif env_name == "walker2d":
        print("Benchmarking Walker2d (gymnasium)...")
        benchmark_walker2d()
    elif env_name == "all":
        print("Benchmarking all environments (gymnasium)...")
        print("Benchmarking Acrobot (gymnasium)...")
        benchmark_acrobot()
        print("Benchmarking Ant (gymnasium)...")
        benchmark_ant()
        print("Benchmarking BipedalWalker (gymnasium)...")
        benchmark_bipedal_walker()
        print("Benchmarking CartPole (gymnasium)...")
        benchmark_cartpole()
        print("Benchmarking HalfCheetah (gymnasium)...")
        benchmark_half_cheetah()
        print("Benchmarking Hopper (gymnasium)...")
        benchmark_hopper()
        print("Benchmarking Humanoid (gymnasium)...")
        benchmark_humanoid()
        print("Benchmarking HumanoidStandup (gymnasium)...")
        benchmark_humanoid_standup()
        print("Benchmarking InvertedDoublePendulum (gymnasium)...")
        benchmark_inverted_double_pendulum()
        print("Benchmarking InvertedPendulum (gymnasium)...")
        benchmark_inverted_pendulum()
        print("Benchmarking LunarLander (gymnasium)...")
        benchmark_lunar_lander()
        print("Benchmarking MountainCar (gymnasium)...")
        benchmark_mountain_car()
        print("Benchmarking Pendulum (gymnasium)...")
        benchmark_pendulum()
        print("Benchmarking Pusher (gymnasium)...")
        benchmark_pusher()
        print("Benchmarking Reacher (gymnasium)...")
        benchmark_reacher()
        print("Benchmarking Swimmer (gymnasium)...")
        benchmark_swimmer()
        print("Benchmarking Walker2d (gymnasium)...")
        benchmark_walker2d()
    else:
        print(f"Unknown environment: {env_name}")
        print("Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all")


if __name__ == "__main__":
    main()
