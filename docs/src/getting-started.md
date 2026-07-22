# Getting Started

## Installation

Add OGym to your `Cargo.toml`:

```toml
[dependencies]
ogym = "0.1.0"
```

## MuJoCo support

MuJoCo environments are opt-in, so the default crate does not require native
MuJoCo libraries. Enable the `mujoco` feature in your `Cargo.toml`:

```toml
[dependencies]
ogym = { version = "0.1.0", features = ["mujoco"] }
```

> **Current limitation:** The MuJoCo feature relies on OGym's repository-local
> patch for `mujoco-rust`, which crates.io packages cannot pass on to consumers.
> Use a source checkout for MuJoCo environments until the patched wrapper is
> published. The default crates.io feature set is unaffected.

You must also install the MuJoCo library separately.

### MuJoCo 3.10.0

OGym targets MuJoCo 3.10.0. On Linux x86-64, install the prebuilt archive under
the default path expected by OGym:

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

> **Note:** If you install MuJoCo elsewhere, set `MUJOCO_DIR` to its root
> directory and ensure its library directory is available to the dynamic
> linker.

## Quick start

The following example runs a Classic Control CartPole environment:

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
