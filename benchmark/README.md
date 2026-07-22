# Benchmark suite: ogym vs Gymnasium

This suite compares equivalent ogym and Gymnasium environments with
[Hyperfine](https://github.com/sharkdp/hyperfine). Classic-control environments
and Toy Text environments run 100,000 steps; Box2D/Rapier2D and MuJoCo
environments run 5,000 steps. The larger workloads reduce process-startup noise
while keeping the complete suite practical.

## Requirements

- Rust and Cargo
- Python 3.11+
- [uv](https://docs.astral.sh/uv/)
- [just](https://github.com/casey/just)
- [Hyperfine](https://github.com/sharkdp/hyperfine)

From this directory, install the locked Python environment and build the Rust
benchmark:

```bash
uv sync
just build
```

The benchmark recipes automatically use `LD_LIBRARY_PATH` on Linux and
`DYLD_LIBRARY_PATH` on macOS. MuJoCo defaults to `~/.local/mujoco`; set
`MUJOCO_DIR` to override that location.

Non-MuJoCo recipes build a default binary with no MuJoCo dependency. The 11
MuJoCo recipes use a separate feature-enabled binary and native loader path, so
Classic Control, Toy Text, and Rapier environments run without MuJoCo installed.

## Run benchmarks

List all recipes with `just`. Run an individual comparison by environment name:

```bash
just acrobot
just cartpole
just mountain-car
just pendulum
```

Run the complete suite or the full benchmark-and-plot workflow with:

```bash
just all
just full
```

From the repository root, prefix individual recipes with `benchmark`, or use
the root aliases for complete workflows:

```bash
just benchmark cartpole
just benchmark-all
just benchmark-full
```

The Gymnasium side is one argument-driven script. It accepts an environment,
an optional step override, and a seed:

```bash
uv run gym_benchmarks.py cartpole
uv run gym_benchmarks.py pendulum --steps 5000 --seed 7
uv run gym_benchmarks.py all
```

Use `--help` to see every supported environment.

## Visualization

`just plot` reads `results/all_results.json` and writes
`results/benchmark.png`. The SciencePlots-styled 2×2 figure groups
the comparisons by environment family: Box2D/Rapier2D and MuJoCo on the top
row, then Classic Control and Toy Text on the bottom row. Every subplot shows
available paired ogym and Gymnasium timings as grouped horizontal bars with
Hyperfine's standard deviation for each environment in that family. Values are
displayed in milliseconds, and each bar is labeled with its mean and standard
deviation. Each panel title reports the geometric-mean speedup. Missing or
incomplete pairs are skipped.

## Layout

```text
benchmark/
├── src/                              # Rust benchmark implementations
├── gym_benchmarks.py                  # All Gymnasium benchmarks and CLI
├── results/                          # Hyperfine data and generated plot
├── Justfile                          # Benchmark recipes
├── pyproject.toml                    # Python dependencies
└── visualize.py                      # 2×2 family comparison figure
```

To add an environment, add its Rust implementation and CLI dispatch in
`src/main.rs`, add a `BenchmarkSpec` to `gym_benchmarks.py`, and
add matching Hyperfine commands to the `Justfile`.
