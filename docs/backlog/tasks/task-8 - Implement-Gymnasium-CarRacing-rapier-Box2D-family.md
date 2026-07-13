---
id: TASK-8
title: Implement Gymnasium CarRacing (rapier Box2D family)
status: To Do
assignee: []
created_date: '2026-07-13 12:42'
labels: []
dependencies:
  - TASK-1
priority: low
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Gymnasium's Box2D family is otherwise covered (LunarLander, BipedalWalker exist under src/env/rapier/), but CarRacing-v3 has no rapier module. This is the largest net-new effort of the missing envs: needs procedural track generation, raycasting/sensor obs (or pixel obs, TBD), and car physics tuned to match Gymnasium's behavior. Lowest priority of the identified gaps — do after the Toy Text family and the existing rapier bug fixes (BipedalWalker leg/joint construction, LunarLander seeding) land, since this would reuse the same PhysicsWorld/rapier2d patterns established there.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A design decision is made and recorded for observation type (vector/sensor-based vs. pixel-based) given this is a native Rust env, not a Python/pygame port
- [ ] #2 CarRacing env implements the Environment trait with procedural track generation and car physics
- [ ] #3 Reward and termination conditions match Gymnasium's CarRacing-v3 structure
<!-- AC:END -->
