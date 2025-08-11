# Lunar Lander Rust Implementation - TODOs

## Discrete Action Issues

- [x] **Incorrect Side Engine Direction Mapping**:
  - In `apply_engine_forces`, for discrete actions:
  - Action 1 should fire the LEFT side engine (direction = +1)
  - Action 3 should fire the RIGHT side engine (direction = -1)
  - Currently both are set to direction = -1.0

- [x] **Missing Action 0 Handling**:

  - Action 0 (do nothing) is not explicitly handled
  - Should be a valid discrete action that applies no forces

## Physics and Configuration Issues

- [x] **Incorrect Joint Limits**:
  - In `create_legs`, the joint limits are backwards:
    - For left leg (i = -1): should be `[-0.9, -0.4]`
    - For right leg (i = 1): should be `[0.4, 0.9]`
    - Currently they are swapped

- [x] **Hardcoded Value in Side Engine**:
  - The `17.0` in side engine impulse position calculation should be derived from lander geometry

- [x] **Missing Wind/Turbulence Indices**:
  - Add `wind_idx` and `torque_idx` fields to track wind patterns consistently
  - Initialize them in reset when wind is enabled

- [ ] **Missing Random Seed Handling**: Use the seed parameter in reset to initialize the random number generator

- [x] **Landing Detection**: Improve landing detection to check for zero linear/angular velocity within threshold rather than `!is_moving()`

- [ ] **Physics Step Parameters**: Consider adding parameters to `world.step()` to match Box2D's `world.Step(1.0 / self.fps, 6 * 30, 2 * 30)`