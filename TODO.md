# TODO: Lunar Lander Implementation Sync and Refactoring

This document outlines the necessary bug fixes, refactoring, and synchronization tasks to align the Rust implementation of the Lunar Lander environment with the reference Python version.

## Bug Fixes & Logic Discrepancies

- **[BUG] Incorrect State Normalization:**
  - `env.rs`: The state calculation for `vel.x` and `vel.y` uses an incorrect normalization formula. It multiplies by the scaled width/height instead of dividing.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Correct the velocity normalization in the `get_state` function to match the Python implementation: `vel.x / (self.config.get_scaled_width() / 2.0) / self.config.fps` and `vel.y / (self.config.get_scaled_height() / 2.0) / self.config.fps`.

- **[BUG] Incorrect Impulse Calculation in `apply_engine_forces`:**
  - `env.rs`: The impulse calculation for both main and side engines appears to be applying force in the wrong direction (`-ox`, `-oy`). The Python version applies a positive impulse.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Verify and correct the impulse vectors in `apply_engine_forces` to ensure they align with the Python logic. The impulse should likely be `(ox * force, oy * force)`.

- **[BUG] Game Over Condition is Incomplete:**
  - `env.rs`: The `is_game_over` function only checks if the lander has gone off-screen horizontally (`state[0].abs() >= 1.0`). It does not account for the lander crashing into the terrain, which the Python version handles via a `contactListener`.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Implement a proper contact detection mechanism (e.g., using Rapier's collision events) to set a `game_over` flag when the lander's main body collides with the moon terrain.

- **[BUG] Incorrect Leg Collision Groups:**
  - `env.rs`: The collision groups for the legs (`InteractionGroups::new(0x0020.into(), 0x001.into())`) do not match the logic implied by the Python version, where legs should only interact with the terrain, not the lander body.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Adjust the interaction groups to prevent collisions between the legs and the lander's main body.

## Refactoring & Code Quality

- **[REFACTOR] Missing Leg Joints:**
  - `env.rs`: The legs are created as separate dynamic bodies but are not attached to the lander with a `RevoluteJoint`, which is crucial for their spring-like behavior.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Implement `RevoluteJoint`s to connect the legs to the lander body, mimicking the `revoluteJointDef` setup in the Python version. This includes setting motor torque and angle limits.

- **[REFACTOR] Use `Builder` Pattern for `LunarLander`:**
  - `env.rs`: The `LunarLander::new` function takes a `LunarLanderConfig`. This could be more ergonomic by creating a `LunarLanderBuilder`.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Create a `LunarLanderBuilder` that takes the `LunarLanderConfig` and constructs the `LunarLander` environment, improving instantiation clarity.

- **[REFACTOR] Random Number Generation:**
  - `env.rs`: `rand::rng()` is called multiple times. A single `Rng` instance should be created in `reset` and reused.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Store the `Rng` instance in the `LunarLander` struct and initialize it in the `reset` method, passing it down to functions that require random numbers.

- **[REFACTOR] Hardcoded Constants:**
  - `env.rs`: Several constants like `CHUNKS`, `LANDER_POLY`, `WIDTH`, and `HEIGHT` are hardcoded at the top of the file.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Move these constants into the `LunarLanderConfig` or associate them with the `LunarLander` struct where appropriate to make the environment more configurable.

- **[REFACTOR] State Representation:**
  - `env.rs`: The state is represented as a raw `SVector`.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Introduce a `LanderState` struct with named fields (`x`, `y`, `vx`, `vy`, etc.) to improve readability and maintainability of the `get_state` and `calc_reward` functions.

## Synchronization with Python Implementation

- **[SYNC] Configuration Defaults:**
  - `config.rs`: The default values in `LunarLanderConfig` need to be cross-referenced with the `__init__` defaults in the Python version to ensure they match exactly. For example, `side_engine_offset_y` was fixed, but others may differ.
  - **File:** `src/env/rapier/lunar_lander/config.rs`
  - **Action:** Audit all default values in `LunarLanderConfig::default()` and align them with the Python implementation.

- **[SYNC] Wind Implementation:**
  - `env.rs`: The wind logic uses `(0.02 * t).sin() + (PI * 0.01 * t).sin()`. The Python version uses two separate indices, `wind_idx` and `torque_idx`, which are initialized randomly.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Add `wind_idx` and `torque_idx` to the `LunarLander` struct, initialize them randomly in `reset`, and use them to calculate wind and turbulence forces as in the Python version.

- **[SYNC] Observation Space Definition:**
  - `env.rs`: The observation space bounds are hardcoded (`-hs`, `hs`). These should be derived from the configuration to match the Python version's `spaces.Box(low, high)`.
  - **File:** `src/env/rapier/lunar_lander/env.rs`
  - **Action:** Define the observation space bounds dynamically based on the configuration parameters, similar to how the Python version constructs its `observation_space`.
