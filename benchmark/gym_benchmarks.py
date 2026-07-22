"""Run Gymnasium environment benchmarks from one command-line entry point."""

from __future__ import annotations

import argparse
import time
from dataclasses import dataclass

import gymnasium as gym


@dataclass(frozen=True)
class BenchmarkSpec:
    gym_id: str
    steps: int


LIGHTWEIGHT_STEPS = 100_000
PHYSICS_STEPS = 5_000

BENCHMARKS = {
    # Classic Control
    "acrobot": BenchmarkSpec("Acrobot-v1", LIGHTWEIGHT_STEPS),
    "cartpole": BenchmarkSpec("CartPole-v1", LIGHTWEIGHT_STEPS),
    "mountain_car": BenchmarkSpec("MountainCar-v0", LIGHTWEIGHT_STEPS),
    "pendulum": BenchmarkSpec("Pendulum-v1", LIGHTWEIGHT_STEPS),
    # Toy Text
    "blackjack": BenchmarkSpec("Blackjack-v1", LIGHTWEIGHT_STEPS),
    "cliff_walking": BenchmarkSpec("CliffWalking-v1", LIGHTWEIGHT_STEPS),
    "frozen_lake": BenchmarkSpec("FrozenLake-v1", LIGHTWEIGHT_STEPS),
    "taxi": BenchmarkSpec("Taxi-v3", LIGHTWEIGHT_STEPS),
    # Box2D / Rapier2D
    "bipedal_walker": BenchmarkSpec("BipedalWalker-v3", PHYSICS_STEPS),
    "lunar_lander": BenchmarkSpec("LunarLander-v3", PHYSICS_STEPS),
    # MuJoCo
    "ant": BenchmarkSpec("Ant-v5", PHYSICS_STEPS),
    "half_cheetah": BenchmarkSpec("HalfCheetah-v5", PHYSICS_STEPS),
    "hopper": BenchmarkSpec("Hopper-v5", PHYSICS_STEPS),
    "humanoid": BenchmarkSpec("Humanoid-v5", PHYSICS_STEPS),
    "humanoid_standup": BenchmarkSpec("HumanoidStandup-v5", PHYSICS_STEPS),
    "inverted_double_pendulum": BenchmarkSpec(
        "InvertedDoublePendulum-v5", PHYSICS_STEPS
    ),
    "inverted_pendulum": BenchmarkSpec("InvertedPendulum-v5", PHYSICS_STEPS),
    "pusher": BenchmarkSpec("Pusher-v5", PHYSICS_STEPS),
    "reacher": BenchmarkSpec("Reacher-v5", PHYSICS_STEPS),
    "swimmer": BenchmarkSpec("Swimmer-v5", PHYSICS_STEPS),
    "walker2d": BenchmarkSpec("Walker2d-v5", PHYSICS_STEPS),
}


def run_benchmark(name: str, *, steps: int | None = None, seed: int = 42) -> None:
    spec = BENCHMARKS[name]
    step_count = steps or spec.steps
    print(f"Benchmarking {spec.gym_id} for {step_count:,} steps...")

    env = gym.make(spec.gym_id)
    try:
        env.action_space.seed(seed)
        env.reset(seed=seed)

        start = time.perf_counter()
        for _ in range(step_count):
            _, _, terminated, truncated, _ = env.step(env.action_space.sample())
            if terminated or truncated:
                env.reset()
        duration = time.perf_counter() - start
    finally:
        env.close()

    print(f"{spec.gym_id} (gymnasium): {duration:.4f}s")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Benchmark one or all Gymnasium environments used by ogym."
    )
    parser.add_argument(
        "environment",
        choices=[*BENCHMARKS, "all"],
        help="environment to benchmark",
    )
    parser.add_argument(
        "--steps",
        type=int,
        help="override the default step count (applies to every environment)",
    )
    parser.add_argument("--seed", type=int, default=42, help="random seed (default: 42)")
    args = parser.parse_args()
    if args.steps is not None and args.steps < 1:
        parser.error("--steps must be greater than zero")
    return args


def main() -> None:
    args = parse_args()
    names = BENCHMARKS if args.environment == "all" else (args.environment,)
    for name in names:
        run_benchmark(name, steps=args.steps, seed=args.seed)


if __name__ == "__main__":
    main()
