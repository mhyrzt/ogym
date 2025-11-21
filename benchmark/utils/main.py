import sys
import subprocess
import os


def main():
    if len(sys.argv) != 2:
        print("Usage: python main.py <environment_name>")
        print("Available environments: acrobot, cartpole, mountain_car, pendulum, all")
        sys.exit(1)

    env_name = sys.argv[1]

    # Define the path to the gym_benchmarks directory relative to this script
    gym_benchmarks_dir = os.path.join(os.path.dirname(__file__), "..", "gym_benchmarks")

    if env_name == "acrobot":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "acrobot_gym.py")]
        )
    elif env_name == "cartpole":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "cartpole_gym.py")]
        )
    elif env_name == "mountain_car":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "mountain_car_gym.py")]
        )
    elif env_name == "pendulum":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "pendulum_gym.py")]
        )
    elif env_name == "all":
        # Update path for gym_benchmarks.py too
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "all",
            ]
        )
    else:
        print(f"Unknown environment: {env_name}")
        print("Available environments: acrobot, cartpole, mountain_car, pendulum, all")
        sys.exit(1)


if __name__ == "__main__":
    main()
