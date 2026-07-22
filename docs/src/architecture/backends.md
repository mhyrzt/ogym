# Backends and dynamics

## Native Rust

Classic Control integrates explicit ordinary differential or difference equations. Toy Text environments implement finite-state transition functions, with seeded randomness where transitions or card draws are stochastic.

## Rapier2D

`PhysicsWorld` owns Rapier rigid bodies, colliders, joints, collision channels, and the query pipeline. Each step advances with an environment-selected \(\Delta t\). At a high level the constrained rigid-body system is

\[
M(q)\ddot q + C(q,\dot q) + g(q) = \tau + J(q)^T\lambda,
\]

with joint/contact impulses \(\lambda\) resolved by Rapier. Environment pages specify the forces, motor controls, state projection, reward, and episode conditions layered over that solve.

## MuJoCo

The optional `mujoco` feature exposes `MjEnv`, which loads XML, owns MuJoCo model/state objects, validates control length, and advances `frame_skip` internal steps. The agent timestep is

\[
\Delta t_{env} = \text{frame\_skip}\,\Delta t_{model}.
\]

MuJoCo solves multibody dynamics with actuation and constraints, conventionally written

\[
M(q)\dot v + c(q,v) = \tau(u) + J(q)^T f,
\qquad q_{t+1}=\operatorname{integrate}(q_t,v_{t+1}).
\]

`MjEnv` exposes `qpos`, `qvel`, controls, body kinematics, inertias, contact forces, tendon data, and actuator/constraint forces used to construct observations and rewards. Custom XML can change dimensions, ranges, timestep, and body names; environment-specific XML must retain every body/site name that its Rust implementation looks up.

References: [Rapier rigid bodies](https://rapier.rs/docs/user_guides/rust/rigid_bodies), [MuJoCo computation](https://mujoco.readthedocs.io/en/stable/computation/), and [MuJoCo XML reference](https://mujoco.readthedocs.io/en/stable/XMLreference.html).
