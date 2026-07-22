# Classic Control

Classic Control environments integrate compact analytic models in native Rust. All four can select discrete or continuous control at construction through their configuration.

| Environment | Observation | Discrete action | Continuous action | Default horizon |
| --- | --- | --- | --- | ---: |
| [Acrobot](classic-control/acrobot.md) | 6 values | 3 torques | 1 torque in \\([-1,1]\\) | 500 |
| [CartPole](classic-control/cart-pole.md) | 4 values | left/right | 1 force command in \\([-1,1]\\) | 500 |
| [MountainCar](classic-control/mountain-car.md) | 2 values | left/coast/right | 1 force command in \\([-1,1]\\) | 200 (999 in continuous preset) |
| [Pendulum](classic-control/pendulum.md) | 3 values | configurable torque grid | 1 bounded torque | 200 |

Angles are radians, angular velocities are radians per step unit, and each page states its integrator and clipping rules.

`Info` is `()` for Acrobot, CartPole, and Pendulum. MountainCar uses `Option<()>` and currently returns `None`.
