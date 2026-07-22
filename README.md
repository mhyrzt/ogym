# OGym

[![Crate](https://img.shields.io/crates/v/ogym.svg)](https://crates.io/crates/ogym)
[![Documentation](https://img.shields.io/badge/docs-mdBook-blue.svg)](https://mhyrzt.github.io/ogym/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A Rust-native reinforcement learning framework inspired by OpenAI's Gym.**

## 📖 Overview

**OGym** is a high-performance reinforcement learning library designed for systems programming. Built entirely in Rust, it offers a flexible, type-safe, and efficient platform for building, training, and interacting with RL environments.

While inspired by the Python-based OpenAI Gym, OGym leverages Rust's ownership model and zero-cost abstractions to ensure thread safety and maximize performance, particularly for parallel environment execution.

## ✨ Features

- 🛡️ **Type-Safety First**: Strongly typed definitions for Actions and Observations eliminate common runtime shape errors.
- 🚀 **High Performance**: Optimized for speed with minimal overhead, supporting compiled environments.
- ⚡ **Native Vectorization**: First-class support for batched environments using parallel processing and matrix operations.
- 🔌 **Modular Backend**: Flexible architecture supporting multiple physics engines (Rapier, MuJoCo) and classic control systems.
- 📦 **Zero-Cost Abstractions**: Idiomatic Rust design patterns that enforce safety without sacrificing speed.

## 📦 Installation

Add `ogym` to your `Cargo.toml`:

```toml
[dependencies]
ogym = "0.1.0"
```

### External Dependencies (MuJoCo)

MuJoCo environments are opt-in so the default crate does not require native MuJoCo libraries. Enable the `mujoco` feature in your `Cargo.toml`:

```toml
[dependencies]
ogym = { version = "0.1.0", features = ["mujoco"] }
```

> **Current limitation:** The MuJoCo feature relies on OGym's repository-local
> patch for `mujoco-rust`, which crates.io packages cannot pass on to consumers.
> Use a source checkout for MuJoCo environments until the patched wrapper is
> published; the default crates.io feature set is unaffected.

You must also install the MuJoCo library separately.

#### MuJoCo 3.9.0

OGym targets the latest official MuJoCo release, **3.9.0**. On Linux x86-64, install the prebuilt archive under the default path expected by OGym:

```sh
mkdir -p "$HOME/.local/mujoco"
curl --fail --location \
  https://github.com/google-deepmind/mujoco/releases/download/3.9.0/mujoco-3.9.0-linux-x86_64.tar.gz \
  | tar -xz --strip-components=1 -C "$HOME/.local/mujoco"
```

Official prebuilt packages are also available for [Linux AArch64](https://github.com/google-deepmind/mujoco/releases/download/3.9.0/mujoco-3.9.0-linux-aarch64.tar.gz), [macOS universal](https://github.com/google-deepmind/mujoco/releases/download/3.9.0/mujoco-3.9.0-macos-universal2.dmg), and [Windows x86-64](https://github.com/google-deepmind/mujoco/releases/download/3.9.0/mujoco-3.9.0-windows-x86_64.zip).

> **Note:** If you install MuJoCo elsewhere, set `MUJOCO_DIR` to its root directory and ensure its library directory is available to the dynamic linker.

## 🚀 Quick Start

Here is a minimal example running a Classic Control environment (CartPole):

```rust
use ogym::{
    env::control::cart_pole::{CartPole, CartPoleConfig},
    env::environment::Environment,
    spaces::Space,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure and initialize the environment
    let config = CartPoleConfig::new();
    let mut env = CartPole::new(config)?;

    env.reset(None)?;

    let mut total_reward: f64 = 0.0;

    // 2. Run the simulation loop
    loop {
        // Sample a random action from the valid action space
        let action = env.space.action.sample()?;

        // Step the environment
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

## 🏗️ Architecture

OGym is built around three core traits that ensure consistency across different physics backends:

| Trait                  | Description                                                                                                       |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **`Space`**            | Defines valid bounds for Actions and Observations. Handles sampling, validation, and shape checking.              |
| **`Environment`**      | The standard interface for single-instance environments. Handles `reset`, `step`, and state management.           |
| **`BatchEnvironment`** | Interface for vectorized environments that manage multiple instances simultaneously for high-throughput training. |

## 🌍 Supported Environments

OGym currently supports the following environment modules:

- **`env/control`**: Classic control benchmarks (e.g., CartPole, Pendulum).
- **`env/toy_text`**: Discrete environments such as FrozenLake, Taxi, Blackjack, and CliffWalking.
- **`env/rapier`**: Physics-based environments powered by the [Rapier](https://rapier.rs/) engine.
- **`env/mujoco`**: High-fidelity physics environments using the MuJoCo engine.

## 🤝 Contributing

We welcome contributions from the community!

- **Bug Reports & Feature Requests**: Please open an issue on GitHub.
- **Pull Requests**: Feel free to submit PRs for bug fixes or new environments. For major architecture changes, please open an issue first to discuss.

Common development workflows are available from the repository root:

```bash
just --list
just validate
just build-mujoco
just test-mujoco
just benchmark cartpole
just benchmark-all
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
