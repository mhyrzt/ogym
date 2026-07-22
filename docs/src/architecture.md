# Architecture

OGym is a Rust-native reinforcement learning framework inspired by OpenAI Gym.
It provides a flexible, type-safe, and high-performance platform for building
and interacting with reinforcement learning environments.

## Core Concepts

The library is built around three core traits that provide a consistent
interface across its physics backends.

| Trait | Description |
| --- | --- |
| **`Space`** | Defines valid bounds for actions and observations, including sampling, validation, and shape checking. |
| **`Environment`** | Defines the standard interface for a single environment, including reset, step, and state management. |
| **`BatchEnvironment`** | Defines vectorized environments that manage multiple instances for high-throughput training. |

### `Space` Trait

The `Space` trait defines the valid action and observation spaces for an
environment. Its main operations are:

- **`sample()`**: Returns a random sample from the space.
- **`contains(&item)`**: Checks whether an item is a valid member of the space.
- **`shape()`**: Describes the dimensionality of the space.
- **`bounds()`**: Returns the lower and upper bounds of the space.

Key implementations include:

- **`Boxed<D>`**: A continuous N-dimensional space.
- **`Discrete`**: A discrete space with `n` possible values.
- **`Mixed`**: A combination of different space types for complex action
  spaces.

### `Environment` Trait

The `Environment` trait provides the interface for a standard, single-instance
environment:

- **`reset()`**: Resets the environment to an initial state.
- **`step(action)`**: Executes one time step in the environment.
- **`is_terminal()`**: Checks whether the episode ended in a terminal state.
- **`is_truncated()`**: Checks whether the episode ended because of a time
  limit.

### `BatchEnvironment` Trait

The `BatchEnvironment` trait defines the interface for vectorized environments,
which manage multiple environment instances at once:

- **`num_envs()`**: Returns the number of parallel environments.
- **`reset_all()`**: Resets all environments.
- **`step_all(actions)`**: Steps all environments with a batch of actions.

## Module Structure

The project is organized into two main modules:

- `src/spaces` contains space definitions such as `Boxed` and `Discrete`.
- `src/env` contains environment-related logic.

The `env` module is further divided into:

- `env/environment`, which defines the core `Environment` and
  `BatchEnvironment` traits.
- `env/control`, which contains classic control environments such as `CartPole`
  and `Pendulum`.
- `env/rapier` and `env/mujoco`, which contain more complex physics-based
  environments using the Rapier and MuJoCo engines.

## Environment Vectorization

OGym supports two vectorization models for different use cases.

### Generic Wrapper (`VecEnv`)

`VecEnv<E: Environment>` wraps multiple instances of any environment that
implements `Environment`. It implements `BatchEnvironment` by iterating over
the contained environments and invoking their methods in a loop.

This approach can vectorize an environment without requiring a specialized
implementation. Because instances are processed sequentially, however, it does
not provide true data parallelism.

### Native Vectorization (`CartPoleVec`)

Native vectorized environments are designed to operate on batches from the
start. Their internal state is represented by a matrix such as
`nalgebra::DMatrix`, with each row holding one environment's state. Physics and
state updates use matrix and vector operations to process the batch together.

This design can take advantage of CPU SIMD and is suitable for GPU offloading,
but each environment requires its own native vectorized implementation.

## Key Dependencies

- **`nalgebra`** provides the vector and matrix types used for state, actions,
  and physics calculations.
- **`rand`** supports action sampling and environment-state initialization.
- **`thiserror`** provides structured error types.
