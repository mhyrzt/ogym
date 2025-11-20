# OGym

[![Crate](https://img.shields.io/crates/v/ogym.svg)](https://crates.io/crates/ogym)
[![Documentation](https://docs.rs/ogym/badge.svg)](https://docs.rs/ogym)
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

If you intend to use `ogym` with MuJoCo physics environments, you must install the MuJoCo library separately.

#### **For macOS (Build from Source)**

Since MuJoCo is required as a native library, we recommend cloning and building the latest version directly:

```bash
# 1. Clone the repository
cd ~/Downloads
git clone https://github.com/google-deepmind/mujoco.git mujoco

# 2. Build the project
cd mujoco
mkdir build
cd build
cmake ..

# Compile using all available CPU cores
make -j$(sysctl -n hw.ncpu)

# 3. Install libraries and headers
mkdir -p ~/.local/mujoco/
cp -r ./lib ../include ~/.local/mujoco/
```

> **Note:** Ensure `~/.local/mujoco/lib` is in your library path or `PKG_CONFIG_PATH` depending on your linker configuration.

#### **For Linux / Windows**

Please download the pre-built binaries from the [Official MuJoCo Releases](https://github.com/google-deepmind/mujoco/releases) or build from source using similar CMake steps.

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
- **`env/rapier`**: Physics-based environments powered by the [Rapier](https://rapier.rs/) engine.
- **`env/mujoco`**: High-fidelity physics environments using the MuJoCo engine.

## 🤝 Contributing

We welcome contributions from the community!

- **Bug Reports & Feature Requests**: Please open an issue on GitHub.
- **Pull Requests**: Feel free to submit PRs for bug fixes or new environments. For major architecture changes, please open an issue first to discuss.

## 🛣️ Roadmap

- [ ] GPU acceleration support for vectorized environments.
- [ ] Expansion of MuJoCo asset loading support.
- [ ] Additional "Gym-standard" environments (Atari, etc.).
- [ ] Python bindings (PyO3) for cross-language usage.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
