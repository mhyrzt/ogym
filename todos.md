# Lunar Lander Implementation Alignment TODOs

This document outlines the necessary changes to align the Rust implementation of the Lunar Lander environment (`src/env/rapier/lunar_lander/env.rs`) with the Python/Box2D version (`src/env/rapier/lunar_lander/lunar_lander.py`).

The items are ordered by their impact on simulation correctness and behavioral parity.

## 1. [CRITICAL] Implement "Out of Bounds" Termination

- **Issue:** The Rust `step` function does not terminate the episode when the lander flies off-screen (`abs(state[0]) >= 1.0`). The Python version terminates with a reward of -100 in this case. The `is_out_of_screen()` method exists in Rust but is not used to determine termination within the main step loop.
- **Required Change:**
  - In the `step` function, check if `self.is_out_of_screen()` is true.
  - If it is, set `terminated` to `true` and the `reward` to `-100.0`, mirroring the Python logic.

## 2. [MAJOR] Correct Wind & Turbulence Physics

- **Issue:** The Rust code applies wind and turbulence as an *impulse* (`apply_impulse`), which is an instantaneous change in velocity. The Python code applies them as a continuous *force* (`ApplyForceToCenter` and `ApplyTorque`). This causes a significant divergence in flight dynamics.
- **Required Change:**
  - In `apply_wind_effects`, replace `apply_impulse` with `apply_force` and `apply_torque_impulse` with `apply_torque`. This will correctly simulate a continuous push from the wind.

## 3. [MAJOR] Replicate Python's Wind Pattern

- **Issue:** The Rust implementation's wind calculation is deterministic, based on the step counter `t` which resets every episode. The Python version uses `wind_idx` and `torque_idx` which are initialized to large random values on reset, creating a more varied and unpredictable wind pattern across episodes.
- **Required Change:**
  - Add `wind_idx` and `torque_idx` fields to the `LunarLander` struct.
  - In the `reset` function, initialize these fields to large random integer values.
  - In `apply_wind_effects`, use these indices for the `sin` and `tanh` calculations instead of `self.t`, and increment them each step.

## 4. [BUG] Correct Right Leg Joint Limits

- **Issue:** The revolute joint limits for the right leg are swapped in the Rust code. The current code sets them as `[-0.4, -0.9]`, which is an invalid range for `[lower, upper]`.
- **Required Change:**
  - In `create_legs`, for the leg where `i = 1.0`, change the joint `limits` to `[-0.9, -0.4]` to match the Python version and correct the joint's range of motion.

## 5. [BUG] Correct Discrete Side Engine Direction

- **Issue:** In discrete action mode, the side engine force for `action = 3` is incorrectly applied in the negative direction (`-1.0`). In Python, action `1` is left and `3` is right.
- **Required Change:**
  - In `apply_engine_forces`, modify the `match` statement for discrete actions. When the action is `MixedItem::Discrete(3)`, the `direction` variable should be `1.0`, not `-1.0`.

## 6. [MINOR] Use Force for Initial Push

- **Issue:** The Rust code uses `apply_impulse` to give the lander its initial random push. The Python code uses `ApplyForceToCenter`. To better match the physics, a force should be used.
- **Required Change:**
  - In `create_lander`, replace the call to `body.apply_impulse(...)` with `body.apply_force(...)` or an equivalent that applies a continuous force for one timestep.

## 7. [MAINTAINABILITY] Align Configuration Parameter Names

- **Issue:** Many configuration parameters in the Rust `LunarLanderConfig` struct have different names than their Python counterparts (e.g., `main_engine_force` vs. `main_engine_power`, `leg_offset_x` vs. `leg_away`). This makes direct comparison and maintenance difficult.
- **Required Change:**
  - Rename the fields in the `LunarLanderConfig` struct and associated variables to match the names used in the Python `__init__` method. This is optional but highly recommended for clarity and future development.
