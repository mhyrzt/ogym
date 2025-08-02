# Lunar Lander Implementation Alignment TODOs

This document outlines the necessary changes to make the Rust implementation of the Lunar Lander environment (`src/env/rapier/lunar_lander/env.rs`) identical to the Python/Box2D version (`src/env/rapier/lunar_lander/lunar_lander.py`).

The items are ordered by priority, from critical bugs to minor inconsistencies.

---

### 1. [CRITICAL] Implement Crash Detection

- **Issue:** The Rust version lacks a crash detection mechanism. The simulation only terminates if the lander flies off-screen or comes to a complete rest on the pad. The Python version terminates with a reward of -100 if the lander's main body collides with the terrain.
- **Required Change:**
  - Implement a mechanism in the Rust `step` function to check for collisions involving the `lander` rigid body. Rapier's `narrow_phase` query or contact events can be used for this.
  - If a collision is detected, the episode should terminate immediately with a reward of -100, just like in the Python version.

### 2. [BUG] Correct Leg Joint Limits

- **Issue:** The revolute joint limits for the right leg (where `i = 1.0`) are swapped in the Rust code. The current code sets them as `[-0.4, -0.9]`.
- **Required Change:**
  - In `create_legs`, change the limits for the right leg to be `[-0.9, -0.4]`, which correctly corresponds to the `[lower, upper]` angle format.

### 3. [BUG] Correct Side Engine Impulse Position

- **Issue:** When calculating the point of application for the side engine impulse, the Rust code incorrectly multiplies the `17.0` term by `self.config.scale` instead of dividing by it.
- **Required Change:**
  - In `apply_engine_forces`, modify the `impulse_pos` calculation for the side engine.
  - Change `... - tip.0 * 17.0 * self.config.scale as f32` to `... - tip.0 * 17.0 / self.config.scale as f32`.

### 4. [BUG] Correct Discrete Side Engine Direction

- **Issue:** In discrete action mode, the side engine force for `action = 3` is incorrectly applied in the negative direction (`-1.0`).
- **Required Change:**
  - In `apply_engine_forces`, modify the `match` statement for discrete actions. When the action is `MixedItem::Discrete(3)`, the `direction` should be `1.0`, not `-1.0`.

### 5. [MAJOR] Use Forces for Wind Effects

- **Issue:** The Rust code applies wind and turbulence as an *impulse* (`apply_impulse`), which is an instantaneous change in velocity. The Python code applies them as a *force* (`ApplyForceToCenter`), which is a continuous push over the physics timestep. This leads to significantly different flight dynamics.
- **Required Change:**
  - In `apply_wind_effects`, replace `apply_impulse` with `apply_force` and `apply_torque_impulse` with `apply_torque`. This will require accessing the rigid body set mutably.

### 6. [MINOR] Replicate Python's Wind Pattern

- **Issue:** The Rust implementation's wind calculation is based on the step counter `t`, which resets to 0 for every episode. The Python version uses two separate indices (`wind_idx`, `torque_idx`) that are initialized to random values at the start of an episode, creating a more varied wind pattern.
- **Required Change:**
  - Add `wind_idx` and `torque_idx` fields to the `LunarLander` struct.
  - In `reset`, initialize these to large random integer values, mimicking the Python implementation.
  - In `apply_wind_effects`, use these indices in the `sin` and `tanh` calculations instead of `self.t`, and increment them on each step.

### 7. [NAMING] Align Configuration Parameters

- **Issue:** Many configuration parameters in Rust have different names than their Python counterparts (e.g., `main_engine_force` vs. `main_engine_power`, `leg_offset_x` vs. `leg_away`).
- **Required Change:**
  - (Optional, for clarity) Rename the fields in the `LunarLanderConfig` struct to match the names used in the Python `__init__` method. This would improve maintainability and make future comparisons easier.
