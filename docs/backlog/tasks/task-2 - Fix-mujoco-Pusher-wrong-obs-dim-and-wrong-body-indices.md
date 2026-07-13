---
id: TASK-2
title: 'Fix mujoco Pusher: wrong obs dim and wrong body indices'
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 20:48'
labels: []
dependencies:
  - TASK-9
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
src/env/mujoco/pusher/env.rs builds a 31-dim observation (qpos 11 + qvel 11 + object_pos 3 + target_pos 3 + fingertip_pos 3) but PusherConfig::observation_shape (pusher/config.rs:21) declares (23,), the real Gymnasium dim. Worse: _get_fingertip_pos/_get_object_pos/_get_target_pos (pusher/env.rs:81-120) read xipos() at hard-coded indices 2/3/4, but the model.xml worldbody order puts tips_arm at 10, object at 11, goal at 12 — indices 2-4 are early arm links, not fingertip/object/goal. Reward (reward_near, reward_dist) is currently computed from the wrong bodies entirely, corrupting the training signal.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Body index lookups for fingertip/object/target are corrected to match model.xml's actual body order (tips_arm/object/goal), verified against the XML rather than hard-coded guesses
- [x] #2 Observation dimension matches PusherConfig::observation_shape (or the declared shape is corrected to match reality, whichever is the intended behavior) and this is covered by a test asserting obs.len()
- [x] #3 A test verifies reward_near/reward_dist are computed from the correct body positions (e.g. by checking known geometry in a small test XML)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added MjEnv::body_com/body_com_vector(name) (xipos, center-of-mass, name-resolved via model().name_to_id) alongside the existing xpos-based body()/body_vector(), since Gymnasium's get_body_com reads xipos not xpos. Fixed _get_object_pos/_get_target_pos/_get_fingertip_pos to resolve 'object'/'goal'/'tips_arm' by name instead of hard-coded xipos indices 3/4/2. Fixed obs to use qpos[..7]/qvel[..7] (arm joints only, matching real Gymnasium -- object/goal slide-joint positions are redundant with their body COMs already in the obs) instead of all 11 qpos/qvel, dropping dim from 31 to the config-declared 23. Also reordered obs to fingertip/object/goal matching Gymnasium's exact layout (was object/goal/fingertip).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed Pusher's body-index bug (was reading arm-link bodies instead of tips_arm/object/goal) via a new name-based xipos lookup on MjEnv, and fixed the observation to be 23-dim (qpos/qvel arm-joints-only + 3 body COMs) matching Gymnasium and the env's own declared config shape. Verified with 3 new tests (body identity vs known XML positions, obs dimension, reward recomputed from the same body positions) plus full suite (271/271 passing).
<!-- SECTION:FINAL_SUMMARY:END -->
