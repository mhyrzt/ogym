---
id: TASK-1
title: 'Fix BipedalWalker: legs/joints never constructed, actions are no-op'
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 18:54'
labels: []
dependencies: []
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
rapier BipedalWalker (src/env/rapier/bipedal_walker/env.rs) declares leg_handles, joint_handles, and legs: Vec<LegData> but never creates them anywhere in reset_env/new — only destroy_world() references them for cleanup. Result: joint_handles is always empty, so step()'s action-application loop (env.rs:211-222, zips action with joint_handles) never runs. Actions currently have zero physical effect — this env is a no-op walker. Also missing: lidar raycasting (lidar_fractions stays [1.0;10] forever despite being part of the 24-dim obs and having lidar_range/lidar_count config) and fall detection (game_over is set false on reset, never set true — no hull-ground collision check, so termination is pure step-count truncation).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Legs are constructed in reset_env/new (similar to LunarLander's create_legs pattern) and joint_handles is populated with real ImpulseJointHandles
- [x] #2 step()'s action loop actually drives leg motors — verified by a test asserting joint motor velocity/torque changes in response to a nonzero action
- [x] #3 Lidar raycasting is implemented so lidar_fractions reflects real distances, not a constant [1.0;10]
- [x] #4 game_over is set true on hull-ground contact (fall detection), verified by a test
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Legs: 2x(upper+lower) rigid bodies + 4 revolute joints (hip/knee per leg), order [hip(-1),knee(-1),hip(+1),knee(+1)] matching Gymnasium. Motor fix: f32::signum() returns 1.0 for 0.0 (never 0.0), so a naive port would drive all joints at full speed on a no-op action. Fixed by scaling max_force by |action| (matching real Gymnasium's maxMotorTorque=MOTORS_TORQUE*clip(abs(action),0,1) in the non-control_speed branch) instead of relying on sign(0)==0. Lidar via QueryPipeline::cast_ray with exclude_rigid_body(hull) + terrain-only collision group filter. Fall detection via collision events on hull (game_over) and lower legs (ground_contact), requires active_events(COLLISION_EVENTS) added to hull/leg/terrain colliders (was missing). Also patched contacts_enabled(false) on hip/knee joints so hull-leg and leg-lowerleg pairs don't self-collide, matching Box2D's default collideConnected=false.

Verification: needed a real crate compile, which was blocked by an unrelated pre-existing mujoco-rust/mujoco-rs-sys break (see TASK-9). Fixed via vendor/mujoco-rust local patch + Cargo.toml [patch.crates-io]. cargo test --lib: 33/33 bipedal_walker tests pass; full suite 267/268 (one pre-existing LunarLander test flakes under parallel execution due to TASK-6's unseeded-global-RNG bug, confirmed unrelated by isolated reruns).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented leg/joint construction, motor driving (with correct zero-action handling), lidar raycasting, and hull-ground fall detection for BipedalWalker. Verified with cargo test --lib (33/33 passing) after fixing an unrelated crate-wide build blocker (TASK-9).
<!-- SECTION:FINAL_SUMMARY:END -->
