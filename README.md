# OGym

[![Crate](https://img.shields.io/crates/v/ogym.svg)](https://crates.io/crates/ogym)
[![Documentation](https://img.shields.io/badge/docs-mdBook-blue.svg)](https://mhyrzt.github.io/ogym/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

OGym is a Rust-native reinforcement learning library inspired by OpenAI Gym.
It provides type-safe environments, explicit action and observation spaces,
and implementations backed by native Rust, Rapier, and MuJoCo.

## Highlights

- **Type-safe APIs:** Actions and observations are checked through strongly
  typed spaces instead of untyped adapters.
- **Native performance:** Environments use compiled Rust implementations with
  minimal runtime overhead.
- **Batch execution:** The `BatchEnvironment` abstraction supports
  high-throughput workloads.
- **Multiple backends:** Classic Control, Toy Text, Rapier2D, and MuJoCo
  environments share a consistent interface.
- **Opt-in native dependencies:** The default feature set does not require a
  MuJoCo installation.

## Benchmarks

![OGym and Gymnasium benchmark comparison](https://raw.githubusercontent.com/mhyrzt/ogym/main/benchmark/results/benchmark.png)

The chart compares OGym with equivalent Gymnasium environments. Values are
mean execution times with standard deviation; lower is better. See the
[benchmark suite](benchmark/README.md) for methodology and reproduction steps.

### Box2D / Rapier2D

| Environment | OGym (Rust) | Gymnasium (Python) | Speedup |
| :---: | :---: | :---: | :---: |
| BipedalWalker | 289.5 ± 33.9 | 1,948.0 ± 296.4 | 6.7× |
| LunarLander | 57.0 ± 8.6 | 635.4 ± 38.8 | 11.2× |

### MuJoCo

| Environment | OGym (Rust) | Gymnasium (Python) | Speedup |
| :---: | :---: | :---: | :---: |
| Ant | 1,550.0 ± 205.3 | 2,582.1 ± 182.9 | 1.7× |
| HalfCheetah | 256.1 ± 49.2 | 1,039.1 ± 113.4 | 4.1× |
| Hopper | 743.4 ± 180.8 | 1,577.1 ± 194.4 | 2.1× |
| Humanoid | 2,293.5 ± 222.8 | 2,901.2 ± 287.8 | 1.3× |
| HumanoidStandup | 3,480.2 ± 362.5 | 5,322.9 ± 800.4 | 1.5× |
| InvertedDoublePendulum | 199.2 ± 21.5 | 1,255.7 ± 111.2 | 6.3× |
| InvertedPendulum | 95.9 ± 33.6 | 1,268.6 ± 163.3 | 13.2× |
| Pusher | 249.6 ± 30.6 | 1,118.2 ± 120.5 | 4.5× |
| Reacher | 97.6 ± 11.4 | 947.0 ± 122.6 | 9.7× |
| Swimmer | 235.6 ± 49.1 | 1,102.6 ± 127.7 | 4.7× |
| Walker2d | 795.0 ± 107.5 | 1,588.4 ± 169.5 | 2.0× |

### Classic Control

| Environment | OGym (Rust) | Gymnasium (Python) | Speedup |
| :---: | :---: | :---: | :---: |
| Acrobot | 63.8 ± 10.4 | 9,553.7 ± 751.4 | 149.7× |
| CartPole | 12.7 ± 3.3 | 2,079.2 ± 210.4 | 163.5× |
| MountainCar | 3.6 ± 0.5 | 2,572.8 ± 415.6 | 705.2× |
| Pendulum | 6.7 ± 0.7 | 10,402.8 ± 1,749.9 | 1,550.4× |

### Toy Text

| Environment | OGym (Rust) | Gymnasium (Python) | Speedup |
| :---: | :---: | :---: | :---: |
| Blackjack | 29.2 ± 3.1 | 8,409.0 ± 855.1 | 288.4× |
| CliffWalking | 2.4 ± 0.8 | 2,267.3 ± 229.5 | 951.1× |
| FrozenLake | 11.1 ± 1.3 | 2,212.0 ± 332.8 | 199.6× |
| Taxi | 2.4 ± 0.9 | 2,377.4 ± 163.8 | 999.2× |

The generated [Markdown report](benchmark/results/all_results.md) and
[raw Hyperfine data](benchmark/results/all_results.json) contain the source
results.

## Supported environments

| Family | Environments | Backend |
| --- | --- | --- |
| Classic Control | Acrobot, CartPole, MountainCar, Pendulum | Native Rust |
| Toy Text | Blackjack, CliffWalking, FrozenLake, Taxi | Native Rust |
| Box2D / Rapier2D | BipedalWalker, LunarLander | Rapier |
| MuJoCo | Ant, HalfCheetah, Hopper, Humanoid, HumanoidStandup, InvertedDoublePendulum, InvertedPendulum, Pusher, Reacher, Swimmer, Walker2d | MuJoCo |

## Installation

Add `ogym` to your `Cargo.toml`:

```toml
[dependencies]
ogym = "0.1.0"
```

### MuJoCo support

MuJoCo environments are optional. Enable the `mujoco` feature and install the
native MuJoCo library separately:

```toml
[dependencies]
ogym = { version = "0.1.0", features = ["mujoco"] }
```

> **Current limitation:** The MuJoCo feature relies on OGym's repository-local
> patch for `mujoco-rust`, which crates.io packages cannot pass on to consumers.
> Use a source checkout for MuJoCo environments until the patched wrapper is
> published; the default crates.io feature set is unaffected.

#### MuJoCo 3.10.0

OGym targets MuJoCo **3.10.0**. On Linux x86-64, install the prebuilt archive
under the default path expected by the project:

```sh
mkdir -p "$HOME/.local/mujoco"
curl --fail --location \
  https://github.com/google-deepmind/mujoco/releases/download/3.10.0/mujoco-3.10.0-linux-x86_64.tar.gz \
  | tar -xz --strip-components=1 -C "$HOME/.local/mujoco"
```

Official prebuilt packages are also available for
[Linux AArch64](https://github.com/google-deepmind/mujoco/releases/download/3.10.0/mujoco-3.10.0-linux-aarch64.tar.gz),
[macOS universal](https://github.com/google-deepmind/mujoco/releases/download/3.10.0/mujoco-3.10.0-macos-universal2.dmg),
and
[Windows x86-64](https://github.com/google-deepmind/mujoco/releases/download/3.10.0/mujoco-3.10.0-windows-x86_64.zip).

If MuJoCo is installed elsewhere, set `MUJOCO_DIR` to its root directory and
make its `lib` directory available to the platform's dynamic linker.

## Quick start

The following example runs a CartPole episode with sampled actions:

```rust
use ogym::{
    env::control::cart_pole::{CartPole, CartPoleConfig},
    env::environment::Environment,
    spaces::Space,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CartPoleConfig::new();
    let mut env = CartPole::new(config)?;

    env.reset(None)?;

    let mut total_reward: f64 = 0.0;
    loop {
        let action = env.space.action.sample()?;
        let experience = env.step(action)?;
        total_reward += experience.reward;

        if experience.terminal.is_done() {
            break;
        }
    }

    println!("Episode finished. Total Reward: {:.2}", total_reward);
    Ok(())
}
```

## Core abstractions

OGym uses three core traits across every backend:

| Trait | Responsibility |
| --- | --- |
| `Space` | Defines, samples, and validates action and observation bounds. |
| `Environment` | Provides reset and step semantics for a single environment. |
| `BatchEnvironment` | Manages multiple environment instances for high-throughput execution. |

For a detailed design overview, see the
[architecture documentation](https://mhyrzt.github.io/ogym/architecture.html).

## Development

Common development workflows are available from the repository root:

```bash
just --list
just validate
just build-mujoco
just test-mujoco
just benchmark cartpole
just benchmark-all
```

See the [contributor guide](docs/src/contributing.md) for repository structure,
testing expectations, and pull request guidance.

## Contributing

Bug reports, feature requests, and pull requests are welcome. Discuss major
architecture changes in an issue before implementation and include benchmark
evidence for performance claims.

## License

OGym is distributed under the [MIT License](LICENSE).
