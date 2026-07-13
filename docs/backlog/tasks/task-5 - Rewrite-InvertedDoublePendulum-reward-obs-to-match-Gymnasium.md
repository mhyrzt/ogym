---
id: TASK-5
title: Rewrite InvertedDoublePendulum reward/obs to match Gymnasium
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 22:36'
labels: []
dependencies: []
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
src/env/mujoco/inverted_double_pendulum/env.rs's reward (env.rs:38-73,119-161) is a flat 10.0-or-0.0 value based only on whether raw joint angles exceed +/-1.5 rad, with no distance/velocity shaping. Real Gymnasium's InvertedDoublePendulum-v4 reward is 10 - dist_penalty - vel_penalty computed from a tip site position and velocities. Observation is also only 6-dim (qpos++qvel) while the env's own InvertedDoublePendulumConfig::observation_shape declares (9,) (config.rs:20) — neither matches Gymnasium's actual 9-dim obs (sin/cos of angles, tip x/y, constrained velocities).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Reward formula matches Gymnasium's 10 - dist_penalty - vel_penalty structure computed from tip position/velocity
- [x] #2 Observation vector is 9-dim and matches Gymnasium's InvertedDoublePendulum-v4 layout (or config's declared shape is corrected to match the actual implementation, whichever is intended), verified by a test
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Rewrote _get_obs to Gymnasium's exact 9-dim layout: cart x[1] + sin(theta1,theta2)[2] + cos(theta1,theta2)[2] + clamped qvel[3] + clamped last element of qfrc_constraint (hinge2's constraint force)[1]. Rewrote reward to healthy_reward(10.0) - dist_penalty - vel_penalty where dist_penalty=0.01*tip_x^2+(tip_z-2)^2 and vel_penalty=1e-3*theta1_vel^2+5e-3*theta2_vel^2, using the model's existing 'tip' site (site_xpos()[0]) instead of raw joint angles. Termination changed from an arbitrary joint-angle threshold (|angle|>1.5) to Gymnasium's actual tip_height<=1.0 condition. Added MjEnv::qfrc_constraint()/site_xpos() usage (both already existed on MjEnv, just unused before).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rewrote InvertedDoublePendulum's observation and reward/termination to match Gymnasium's InvertedDoublePendulum-v4 exactly (sin/cos obs layout + constraint force, tip-position-based dist/vel reward penalty, tip-height termination), replacing the previous flat pass/fail reward and raw-joint-angle obs/termination. Verified with 4 tests (obs layout, reward formula, termination condition, existing truncation test) plus full suite (286/286 passing).
<!-- SECTION:FINAL_SUMMARY:END -->
