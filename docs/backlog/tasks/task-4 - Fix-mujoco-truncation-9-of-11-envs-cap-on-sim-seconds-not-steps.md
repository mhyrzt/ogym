---
id: TASK-4
title: 'Fix mujoco truncation: 9 of 11 envs cap on sim-seconds not steps'
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 21:06'
labels: []
dependencies: []
priority: medium
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
half_cheetah, hopper, humanoid_standup, inverted_double_pendulum, pusher, reacher, swimmer, and walker2d all truncate on env.time() > 1000.0 (simulation seconds), not step count. E.g. hopper's dt = 4*0.002 = 0.008s means ~125,000 steps before truncation — vastly looser than Gymnasium's standard 1000-step cap. Only ant and humanoid correctly use a step-count max_episode_steps (default 1000). Separately, inverted_pendulum has no truncation at all — is_truncated() is hardcoded false (inverted_pendulum/env.rs:130,148-150), so episodes only end via the angle-fallen termination.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 half_cheetah, hopper, humanoid_standup, inverted_double_pendulum, pusher, reacher, swimmer, walker2d all truncate based on a step counter (matching ant/humanoid's pattern) with a configurable max_episode_steps defaulting to 1000
- [x] #2 inverted_pendulum gains step-count-based truncation with a configurable max_episode_steps defaulting to 1000 (matching Gymnasium's InvertedPendulum-v4)
- [x] #3 Each affected env has a test asserting is_truncated() becomes true at exactly max_episode_steps
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Applied ant/humanoid's existing pattern (private steps:usize field, incremented after do_simulation, reset to 0 in reset(), is_truncated()/step()'s truncated compare steps>=config.max_episode_steps) to half_cheetah, hopper, humanoid_standup, pusher, reacher, swimmer, walker2d (all previously env.time()>1000.0 -- unbounded in practice), inverted_double_pendulum (previously env.time() > max_steps*timestep(), which ignored frame_skip and actually truncated ~5x too EARLY at its default frame_skip=5, not too late), and inverted_pendulum (previously no truncation at all). Each env's config gained a max_episode_steps field (or reused inverted_double_pendulum's existing max_steps) defaulting to 1000. Added a per-env test asserting is_truncated() flips exactly at max_episode_steps.

Found but did NOT fix (separate, pre-existing, out of this task's scope): reset() in several mujoco envs calls rng.random_range(-noise_scale..noise_scale), which panics with 'cannot sample empty range' whenever noise_scale is exactly 0.0 (half_cheetah already guards this with an if noise_scale > 0.0 check; hopper/humanoid_standup/pusher/reacher/swimmer/walker2d/inverted_pendulum/inverted_double_pendulum/ant do not). Worth a follow-up task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced sim-time-based (or, for inverted_pendulum, entirely missing) truncation with step-count-based truncation matching ant/humanoid's existing correct pattern, across all 9 affected envs. Verified with 9 new tests (one per env, asserting is_truncated() flips exactly at max_episode_steps) plus full suite (283/283 passing).
<!-- SECTION:FINAL_SUMMARY:END -->
