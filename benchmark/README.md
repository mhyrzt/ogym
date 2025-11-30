# Benchmark Suite: ogym vs gymnasium

This benchmark suite compares the performance of `ogym` (our custom Rust library) against `gymnasium` (the Python reinforcement learning environment library) using `hyperfine` for precise timing.

## Overview

The benchmark suite includes multiple classic control and physics simulation environments:

### Classic Control Environments
- **Acrobot**: A two-link pendulum with only the second joint actuated
- **CartPole**: A pole balancing on a cart
- **MountainCar**: A car in a valley that needs to reach the top of a hill
- **Pendulum**: A simple pendulum that swings freely

### Physics Simulation Environments
- **Ant**: A 3D four-legged robot navigating terrain (MuJoCo)
- **BipedalWalker**: A 2D two-legged robot walking on procedurally generated terrain (Rapier)
- **HalfCheetah**: A 3D two-legged cheetah-like robot (MuJoCo)
- **Hopper**: A 2D one-legged robot hopping forward (MuJoCo)
- **LunarLander**: A 2D lander trying to land on the moon (Rapier)
- **Walker2d**: A 2D two-legged robot walking forward (MuJoCo)

Each environment runs 10,000 steps (1,000 for physics-based environments) with random actions, measuring the total execution time. Hyperfine automatically determines the number of runs and provides statistical analysis.

## Prerequisites

- Rust toolchain (with `cargo`)
- Python 3.13+ with `pip`
- `just` command runner (install with `brew install just` on macOS or `apt install just` on Ubuntu)
- `hyperfine` benchmarking tool (install with `brew install hyperfine` on macOS or `apt install hyperfine` on Ubuntu)

## Setup

1. **Install Rust dependencies**:
   ```bash
   # Build the entire ogym library and benchmarks
   cargo build --release
   ```

2. **Install hyperfine**:
   ```bash
   # On macOS
   brew install hyperfine

   # On Ubuntu/Debian
   sudo apt install hyperfine

   # On other systems, see: https://github.com/sharkdp/hyperfine
   ```

3. **Install Python dependencies**:
   ```bash
   cd benchmark
   pip install gymnasium numpy matplotlib seaborn pandas
   # Or install via pyproject.toml:
   pip install -e .
   ```

## Running Benchmarks

### Using Just (Recommended)

The `Justfile` provides convenient commands for running benchmarks with hyperfine:

**Run all benchmarks with hyperfine (results saved to files)**:
```bash
# Run comprehensive comparison with hyperfine (saves results to results/ directory)
just hyperfine-compare-all

# Run specific environment comparison
just hyperfine-compare-acrobot
just hyperfine-compare-cartpole
just hyperfine-compare-mountain-car
just hyperfine-compare-pendulum
```

**Run full comparison with visualization**:
```bash
# Clean, build, run benchmarks, and generate visualizations
just full-comparison
```

**Run visualization only**:
```bash
# Generate plots from existing results
just plot-results
```

### Manual Execution with Hyperfine

**For ogym (Rust)**:
```bash
# Run all ogym environments with hyperfine
hyperfine --command-name "ogym-acrobot" 'cargo run --release -- acrobot' \
          --command-name "ogym-cartpole" 'cargo run --release -- cartpole' \
          --command-name "ogym-mountain-car" 'cargo run --release -- mountain_car' \
          --command-name "ogym-pendulum" 'cargo run --release -- pendulum' \
          --export-json results/ogym_results.json
```

**For gymnasium (Python)**:
```bash
# Run all gymnasium environments with hyperfine
hyperfine --command-name "gym-acrobot" 'python acrobot_gym.py' \
          --command-name "gym-cartpole" 'python cartpole_gym.py' \
          --command-name "gym-mountain-car" 'python mountain_car_gym.py' \
          --command-name "gym-pendulum" 'python pendulum_gym.py' \
          --export-json results/gym_results.json
```

## Visualization

The visualization script (`visualize_results.py`) generates:
- `results/benchmark_comparison.png`: Bar chart comparing mean execution times
- `results/speedup_analysis.png`: Speedup ratio analysis (ogym vs gymnasium)
- `results/benchmark_results.csv`: Raw data in CSV format
- Individual environment comparison plots

## File Structure

```
benchmark/
├── src/                        # Rust benchmark source code
│   ├── main.rs                 # Main entry point
│   ├── acrobot_bench.rs        # Acrobot-specific benchmark
│   ├── cartpole_bench.rs       # CartPole-specific benchmark
│   ├── mountain_car_bench.rs   # MountainCar-specific benchmark
│   └── pendulum_bench.rs       # Pendulum-specific benchmark
├── acrobot_gym.py             # Acrobot benchmark for gymnasium
├── cartpole_gym.py            # CartPole benchmark for gymnasium
├── mountain_car_gym.py        # MountainCar benchmark for gymnasium
├── pendulum_gym.py            # Pendulum benchmark for gymnasium
├── gym_benchmarks.py          # Main Python script for gymnasium benchmarks
├── visualize_results.py       # Visualization script
├── results/                   # Benchmark results directory (created automatically)
├── Cargo.toml                 # Rust dependencies
├── pyproject.toml             # Python dependencies
├── Justfile                   # Just command definitions
└── README.md                  # This file
```

## Customizing Benchmarks

You can customize the benchmark parameters by editing the Rust benchmark files in `src/` to:
- Change the number of steps (currently 10,000)
- Modify environment configurations
- Adjust random seed values
- Change action selection strategies

## Contributing

To add a new environment to the benchmark suite:
1. Create a new Rust benchmark file in `src/`
2. Create a corresponding Python benchmark file
3. Add the new environment to both `main.rs` and `gym_benchmarks.py`
4. Add appropriate commands to the `Justfile`