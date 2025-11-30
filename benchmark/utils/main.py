import sys
import subprocess
import os


def main():
    if len(sys.argv) != 2:
        print("Usage: python main.py <environment_name>")
        print("Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all")
        sys.exit(1)

    env_name = sys.argv[1]

    # Define the path to the gym_benchmarks directory relative to this script
    gym_benchmarks_dir = os.path.join(os.path.dirname(__file__), "..", "gym_benchmarks")

    if env_name == "acrobot":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "acrobot_gym.py")]
        )
    elif env_name == "ant":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "ant_gym.py")]
        )
    elif env_name == "bipedal_walker":
        # Bipedal walker doesn't have a direct gymnasium equivalent, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "bipedal_walker",
            ]
        )
    elif env_name == "cartpole":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "cartpole_gym.py")]
        )
    elif env_name == "half_cheetah":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "half_cheetah_gym.py")]
        )
    elif env_name == "hopper":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "hopper_gym.py")]
        )
    elif env_name == "humanoid":
        # Humanoid doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "humanoid",
            ]
        )
    elif env_name == "humanoid_standup":
        # HumanoidStandup doesn't have a direct gymnasium equivalent, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "humanoid_standup",
            ]
        )
    elif env_name == "inverted_double_pendulum":
        # InvertedDoublePendulum doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "inverted_double_pendulum",
            ]
        )
    elif env_name == "inverted_pendulum":
        # InvertedPendulum doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "inverted_pendulum",
            ]
        )
    elif env_name == "lunar_lander":
        # Lunar lander doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "lunar_lander",
            ]
        )
    elif env_name == "mountain_car":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "mountain_car_gym.py")]
        )
    elif env_name == "pendulum":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "pendulum_gym.py")]
        )
    elif env_name == "pusher":
        # Pusher doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "pusher",
            ]
        )
    elif env_name == "reacher":
        # Reacher doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "reacher",
            ]
        )
    elif env_name == "swimmer":
        # Swimmer doesn't have a separate file, so we use the general script
        subprocess.run(
            [
                sys.executable,
                os.path.join(gym_benchmarks_dir, "gym_benchmarks.py"),
                "swimmer",
            ]
        )
    elif env_name == "walker2d":
        subprocess.run(
            [sys.executable, os.path.join(gym_benchmarks_dir, "walker2d_gym.py")]
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
        print("Available environments: acrobot, ant, bipedal_walker, cartpole, half_cheetah, hopper, humanoid, humanoid_standup, inverted_double_pendulum, inverted_pendulum, lunar_lander, mountain_car, pendulum, pusher, reacher, swimmer, walker2d, all")
        sys.exit(1)


if __name__ == "__main__":
    main()
