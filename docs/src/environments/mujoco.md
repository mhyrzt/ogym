# MuJoCo environments

MuJoCo environments require the `mujoco` feature and MuJoCo 3.10.0. Construct them with `new(None)` for defaults or `new(Some(config))`. Actions and observations are `DVector<f64>`; control length is the loaded model's `nu`, and controls use its actuator ranges.

| Environment | Default observation | Default action and range | Horizon |
| --- | ---: | --- | ---: |
| [Ant](mujoco/ant.md) | 105 | 8 × \\([-1,1]\\) | 1000 |
| [HalfCheetah](mujoco/half-cheetah.md) | 17 | 6 × \\([-1,1]\\) | 1000 |
| [Hopper](mujoco/hopper.md) | 11 | 3 × \\([-1,1]\\) | 1000 |
| [Humanoid](mujoco/humanoid.md) | 348 | 17 × \\([-0.4,0.4]\\) | 1000 |
| [HumanoidStandup](mujoco/humanoid-standup.md) | 348 | 17 × \\([-0.4,0.4]\\) | 1000 |
| [InvertedDoublePendulum](mujoco/inverted-double-pendulum.md) | 9 | 1 × \\([-1,1]\\) | 1000 |
| [InvertedPendulum](mujoco/inverted-pendulum.md) | 4 | 1 × \\([-3,3]\\) | 1000 |
| [Pusher](mujoco/pusher.md) | 23 | 7 × \\([-2,2]\\) | 100 |
| [Reacher](mujoco/reacher.md) | 10 | 2 × \\([-1,1]\\) | 50 |
| [Swimmer](mujoco/swimmer.md) | 8 | 2 × \\([-1,1]\\) | 1000 |
| [Walker2d](mujoco/walker2d.md) | 17 | 6 × \\([-1,1]\\) | 1000 |

Each action is held for `frame_skip` MuJoCo steps. Thus \\(\Delta t=\text{frame_skip}\times\text{model timestep}\\), and velocity rewards use displacement divided by this agent timestep. See [backend dynamics](../architecture/backends.md#mujoco).

> Custom XML may change `nq`, `nv`, `nu`, body count, ranges, timestep, and therefore observation/action dimensions. It must preserve body and site names used by task-specific lookups.
