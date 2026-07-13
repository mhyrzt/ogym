---
id: TASK-3
title: 'Fix mujoco Reacher: wrong obs dim and off-by-one body index'
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 20:51'
labels: []
dependencies:
  - TASK-9
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
src/env/mujoco/reacher/env.rs builds a 12-dim observation (qpos 4 + qvel 4 + target_pos 2 + fingertip_pos 2) vs ReacherConfig::observation_shape declaring (10,) (reacher/config.rs:19); real Gymnasium is 11-dim. _get_target_pos reads xipos()[1] (actually body1, not target) and _get_fingertip_pos reads xipos()[2] — reacher/env.rs:75-99. XML body order is body0(1), body1(2), fingertip(3), target(4), so target position is read from the wrong body, corrupting reward_dist.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Body index lookups for target/fingertip are corrected to match model.xml's actual body order
- [x] #2 Observation dimension matches ReacherConfig::observation_shape (or the declared shape is corrected), covered by a test asserting obs.len()
- [x] #3 A test verifies reward_dist is computed from the correct target/fingertip positions
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reused MjEnv::body_com_vector(name) added for TASK-2. Fixed _get_target_pos/_get_fingertip_pos to resolve 'target'/'fingertip' by name (was xipos indices 1/2, landing on body0/body1 -- the arm links -- for both). Also rewrote _get_obs to match Gymnasium's actual Reacher-v4 layout (cos(theta)[2]+sin(theta)[2]+target-qpos[2]+arm-qvel[2]+fingertip-to-target-vec[3]=11), not just the previous ad-hoc qpos+qvel+raw-positions concat (12-dim). Corrected ReacherConfig::observation_shape from the previously-guessed (10,) to the real (11,).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed Reacher's body-index bug (target/fingertip both resolved to arm-link bodies) via name-based lookup, and rewrote the observation to match Gymnasium's real 11-dim Reacher-v4 layout. Verified with 3 new tests (target/fingertip distinguished via a joint-movement probe rather than assumed absolute coordinates, obs dimension, reward recomputed from the same positions) plus full suite (274/274 passing).
<!-- SECTION:FINAL_SUMMARY:END -->
