# Environments

OGym provides 21 environments behind one typed [`Environment`](architecture/core-abstractions.md#environment) contract. The `state` returned by an environment is the agent-visible observation; it is not necessarily the backend's complete physical state.

| Family | Environments | Actions | Backend |
| --- | --- | --- | --- |
| [Classic Control](environments/classic-control.md) | Acrobot, CartPole, MountainCar, Pendulum | Discrete and continuous | Native Rust |
| [Toy Text](environments/toy-text.md) | Blackjack, CliffWalking, FrozenLake, Taxi | Discrete | Native Rust |
| [Rapier](environments/rapier.md) | BipedalWalker, LunarLander | Continuous; LunarLander also supports discrete | Rapier2D |
| [MuJoCo](environments/mujoco.md) | Ant, HalfCheetah, Hopper, Humanoid, HumanoidStandup, InvertedDoublePendulum, InvertedPendulum, Pusher, Reacher, Swimmer, Walker2d | Continuous | MuJoCo 3.10.0 |

Every environment page specifies the observation and action spaces, transition or backend dynamics, reward, termination and truncation conditions, configuration, and returned information. See [notation and conventions](environments/conventions.md) before comparing equations across families.
