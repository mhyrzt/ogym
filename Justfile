# MuJoCo configuration for feature-enabled root workflows.
home_dir := env('HOME')
mujoco_dir := env('MUJOCO_DIR', home_dir / ".local/mujoco")
mujoco_lib_path := mujoco_dir / "lib"
loader_path_var := if os() == "linux" { "LD_LIBRARY_PATH" } else if os() == "macos" { "DYLD_LIBRARY_PATH" } else { error("MuJoCo workflows are unsupported on " + os()) }
inherited_loader_path := env(loader_path_var, '')
mujoco_loader_path := if inherited_loader_path == "" { mujoco_lib_path } else { mujoco_lib_path + ":" + inherited_loader_path }
mujoco_env := "MUJOCO_DIR=" + mujoco_dir + " " + loader_path_var + "=" + mujoco_loader_path

# Default recipe lists available project commands.
default:
    @just --list

# Type-check the default feature set.
check:
    cargo check --locked

# Build the default feature set.
build:
    cargo build --locked

# Build with MuJoCo support and the local native library.
build-mujoco:
    {{ mujoco_env }} cargo build --locked --features mujoco

# Run the default test suite.
test:
    cargo test --locked

# Run tests with MuJoCo support and the local native library.
test-mujoco:
    {{ mujoco_env }} cargo test --locked --features mujoco

# Check Rust formatting without changing files.
fmt-check:
    cargo fmt --all --check

# Format Rust sources.
fmt:
    cargo fmt --all

# Lint all default-feature targets.
clippy:
    cargo clippy --all-targets --locked

# Build the mdBook documentation.
docs:
    mdbook build docs --dest-dir target/site

# Verify the crates.io package contents.
package:
    cargo package --locked

# Remove Cargo build artifacts.
clean:
    cargo clean

# Run the standard local validation suite.
validate: fmt-check check test clippy docs

# List benchmark recipes or run one by name, for example `just benchmark cartpole`.
benchmark *args:
    just -f benchmark/Justfile {{ args }}

# Run every benchmark in one Hyperfine session.
benchmark-all:
    just -f benchmark/Justfile all

# Run benchmarks and generate the comparison plot.
benchmark-full:
    just -f benchmark/Justfile full

# Generate the comparison plot from existing results.
benchmark-plot:
    just -f benchmark/Justfile plot
