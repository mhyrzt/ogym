---
id: TASK-7
title: >-
  Implement Gymnasium Toy Text env family (FrozenLake, Taxi, Blackjack,
  CliffWalking)
status: To Do
assignee: []
created_date: '2026-07-13 12:42'
labels: []
dependencies: []
priority: low
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Gymnasium's Toy Text family has no directory/module in this repo at all (checked src/env/ — only control/, rapier/, mujoco/ exist). These are simple discrete-state envs needing no physics engine, just the existing Discrete space (src/spaces/discrete.rs) and a transition table per env, implementing the existing Environment trait (src/env/environment/single.rs). Likely the cheapest missing family to add given the existing Space/Environment abstractions already fit this env style well (see src/env/control for the closest existing pattern of non-physics-engine envs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 FrozenLake, Taxi, Blackjack, and CliffWalking each have a module under src/env/toy_text/ (or similar) implementing Environment with Discrete state/action spaces
- [ ] #2 Each env's transition dynamics and reward match Gymnasium's documented behavior for that env
- [ ] #3 Each env has unit tests covering reset determinism (seeded), step transitions, and terminal conditions
<!-- AC:END -->
