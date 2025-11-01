# OGym Architecture

This document outlines the architecture and design principles of the `ogym` reinforcement learning library.

## Overview

`ogym` is a Rust-native reinforcement learning framework inspired by OpenAI's Gym. It provides a set of tools for building and interacting with RL environments. The core focus is on providing a flexible, type-safe, and high-performance platform for RL research and development.

## Core Concepts

The library is built around a few key traits that define the main components of the RL workflow.

### 1. `Space` Trait

The `Space` trait defines the valid action and observation spaces for an environment.

- **`sample()`**: Returns a random sample from the space.
- **`contains(&item)`**: Checks if a given item is a valid member of the space.
- **`shape()`**: Describes the dimensionality of the space.
- **`bounds()`**: Returns the lower and upper bounds of the space.

Key implementations include:

- **`Boxed<D>`**: A continuous N-dimensional space.
- **`Discrete`**: A discrete space with `n` possible values.
- **`Mixed`**: A space that can represent a combination of different space types, useful for complex action spaces.

### 2. `Environment` Trait

This is the trait for a standard, single-instance environment.

- **`reset()`**: Resets the environment to an initial state.
- **`step(action)`**: Executes one time step in the environment.
- **`is_terminal()`**: Checks if the episode has ended due to a terminal state.
- **`is_truncated()`**: Checks if the episode has ended due to a time limit.

### 3. `BatchEnvironment` Trait

This trait defines the interface for vectorized environments, which manage multiple environment instances at once for parallel processing.

- **`num_envs()`**: Returns the number of parallel environments.
- **`reset_all()`**: Resets all environments.
- **`step_all(actions)`**: Steps all environments simultaneously with a batch of actions.

## Module Structure

The project is organized into two main modules:

- `src/spaces`: Contains all space-related definitions (`Boxed`, `Discrete`, etc.).
- `src/env`: Contains the environment-related logic.

The `env` module is further subdivided:

- `env/environment`: Defines the core `Environment` and `BatchEnvironment` traits.
- `env/control`: Contains classic control environments like `CartPole` and `Pendulum`.
- `env/rapier` & `env/mujoco`: (In-progress) Intended for more complex physics-based environments using the Rapier and MuJoCo physics engines.

## Environment Vectorization

`ogym` supports two models for environment vectorization to cater to different needs.

### 1. Generic Wrapper (`VecEnv`)

- **Design**: A generic struct `VecEnv<E: Environment>` that wraps any number of single `Environment` instances.
- **Implementation**: It implements `BatchEnvironment` by iterating over the contained environments and calling their methods in a loop.
- **Pros**: It can vectorize *any* environment that implements the `Environment` trait without modification.
- **Cons**: Performance is limited because it processes environments sequentially. There is no true data parallelism.

### 2. Native Vectorization (`CartPoleVec`)

- **Design**: An environment that is vectorized from the ground up.
- **Implementation**: The internal state is represented by a matrix (e.g., `nalgebra::DMatrix`), where each row is one environment's state. The physics and state-update logic are written using matrix and vector operations that process all environments at once.
- **Pros**: **Massively parallel and fast**. This design leverages SIMD on CPUs and is the only efficient way to offload computation to a GPU.
- **Cons**: It is not generic. A native vectorized implementation must be written for each specific environment.

## Key Dependencies

- **`nalgebra`**: The cornerstone for all numerical operations. Its generic vector and matrix types (`SVector`, `DMatrix`) are used extensively for states, actions, and internal physics calculations, providing both performance and compile-time safety.
- **`rand`**: Used for sampling actions and initializing environment states.
- **`thiserror`**: Used for creating clear, structured error types.
