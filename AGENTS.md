# Repository Guidelines

## Project Structure & Module Organization

OGym is a Rust 2021 library crate. `src/lib.rs` exposes the public API. Shared environment traits and result types live in `src/env/environment/`; implementations are grouped by backend in `src/env/control/`, `src/env/toy_text/`, `src/env/rapier/`, and the feature-gated `src/env/mujoco/`. Space definitions belong in `src/spaces/`. Keep environment-specific configuration, implementation, helpers, and assets together; MuJoCo XML models sit beside their Rust modules.

Documentation sources are in `docs/src/` and build with mdBook. `benchmark/` is a separate Rust/Python comparison suite. `vendor/mujoco-rust/` contains the repository-local compatibility patch and should only change when updating that dependency.

## Build, Test, and Development Commands

- `cargo check --locked`: type-check the default feature set using the lockfile.
- `cargo test --locked`: run the colocated Rust unit tests.
- `cargo fmt --all --check`: verify rustfmt formatting.
- `cargo clippy --all-targets --locked`: lint every default-feature target.
- `cargo package --locked`: verify the crates.io package contents.
- `mdbook build docs --dest-dir target/site`: build contributor documentation locally.
- `cd benchmark && just cartpole`: build and compare one environment; see `just --list` for the full suite. Benchmarks require `uv` and `hyperfine`.

MuJoCo builds require `--features mujoco`, a native MuJoCo installation, and possibly `MUJOCO_DIR` plus the platform linker path.

## Coding Style & Naming Conventions

Use standard rustfmt output (four-space indentation) and keep Clippy clean. Name modules, functions, and variables in `snake_case`; types, traits, and enum variants in `UpperCamelCase`; constants in `SCREAMING_SNAKE_CASE`. Follow the existing environment layout: `config.rs`, `env.rs`, optional helpers or assets, then exports from `mod.rs`. Preserve the crate's type-safe `Environment` and `Space` abstractions instead of adding untyped adapters.

## Testing Guidelines

Place unit tests in the source module under `#[cfg(test)]`, with descriptive `test_*` function names. Cover valid behavior, boundary conditions, invalid actions or shapes, terminal/truncation semantics, and seeded resets where relevant. There is no numeric coverage gate; CI requires the full default test suite to pass. Run focused tests during development, for example `cargo test cart_pole`, then run `cargo test --locked` before submission.

## Commit & Pull Request Guidelines

Recent history uses scoped Conventional Commit subjects such as `fix(toy_text): support Blackjack SAB rules` and `docs: add mdBook documentation`. Keep commits focused and use an imperative summary. Pull requests should explain the behavior change, list validation commands, and link relevant issues. Include benchmark evidence for performance claims and screenshots only for rendered documentation changes. Discuss major architecture changes in an issue first.
