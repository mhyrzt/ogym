---
id: TASK-10
title: Fix mujoco reset() panic when reset_noise_scale is 0.0
status: Done
assignee: []
created_date: '2026-07-13 21:06'
updated_date: '2026-07-13 22:47'
labels: []
dependencies: []
priority: low
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Several mujoco envs' reset() calls rng.random_range(-noise_scale..noise_scale) to jitter initial qpos/qvel, which panics with 'cannot sample empty range' whenever noise_scale is exactly 0.0 (an empty f64 range, since low==high). Found while writing a TASK-4 test for inverted_pendulum's truncation, which had to route around this by not testing with reset_noise_scale=0.0. half_cheetah already guards this correctly (checks noise_scale > 0.0 before sampling, falls back to the unperturbed init state otherwise). ant, hopper, humanoid_standup, pusher, reacher, swimmer, walker2d, inverted_pendulum, and inverted_double_pendulum do not have this guard and will panic if a user sets with_reset_noise_scale(0.0) (a legitimate config, e.g. for deterministic testing).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 reset() with reset_noise_scale set to exactly 0.0 does not panic, for every affected env
- [x] #2 Each affected env has a test constructing a config with reset_noise_scale=0.0 and calling reset() successfully
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Correction to original diagnosis: pusher and reacher were listed as affected but aren't -- their reset() noise uses hard-coded literal constants (0.01, 0.1), not config.reset_noise_scale at all (neither PusherConfig nor ReacherConfig even has that field), so they can never hit the empty-range panic. Actually affected and fixed: ant, hopper, humanoid_standup, swimmer, walker2d, inverted_pendulum, inverted_double_pendulum -- each gained an if noise_high > noise_low guard around the qpos/qvel noise loops (ant's qvel loop uses rng.random::<f64>() rather than random_range, so it was never at risk and didn't need guarding).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Guarded the reset_noise_scale-driven qpos/qvel noise sampling in ant, hopper, humanoid_standup, swimmer, walker2d, inverted_pendulum, and inverted_double_pendulum against empty-range panics when reset_noise_scale=0.0 (pusher/reacher turned out not to be affected -- their noise is hard-coded, unrelated to config). Verified with 7 new tests plus full suite (296/296 passing).
<!-- SECTION:FINAL_SUMMARY:END -->
