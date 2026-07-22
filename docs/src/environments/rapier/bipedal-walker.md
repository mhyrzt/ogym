# BipedalWalker

## State space

The `SVector<f32, 24>` observation contains, in order: hull angle; scaled hull angular, horizontal, and vertical velocity; left hip angle/speed; left knee angle-plus-one/speed; left foot contact; the corresponding five right-leg values; and 10 normalized lidar hit fractions. If `lidar_count` differs from 10, values are copied into the fixed 24-vector up to its capacity and remaining elements are zero.

## Action space

The continuous `SVector<f32, 4>` controls left hip, left knee, right hip, and right knee. Values are clipped to \([-1,1]\) when applied. With `control_speed=false`, sign chooses target direction and magnitude scales maximum motor torque. With `control_speed=true`, the value scales target speed while maximum torque stays fixed. There is no discrete mode.

## Dynamics and reward

Rapier advances one step with \(\Delta t=1/\text{fps}\). Joint motors and contact constraints enter the shared rigid-body equation from the [backend chapter](../../architecture/backends.md#rapier2d). Define

\[
H_t=130x_t/\text{scale}-5|\theta_t|.
\]

The ordinary reward is

\[
r_t=H_t-H_{t-1}-0.00035\,\tau_{motor}\sum_i\operatorname{clip}(|a_i|,0,1).
\]

Hull contact or movement behind \(x=0\) overrides it with \(-100\).

## Episode end

Termination occurs on hull contact, \(x<0\), or reaching the end of the generated track. Truncation occurs at `max_episode_steps`.

## Configuration

| Fields | Defaults | Meaning |
| --- | --- | --- |
| `fps`, `scale` | `50`, `30` | Simulation rate and geometry scale |
| `motors_torque` | `80` | Joint motor authority |
| `speed_hip`, `speed_knee` | `4`, `6` | Motor target-speed scales |
| `lidar_range`, `lidar_count` | `160/scale`, `10` | Terrain ray sensors |
| `initial_random` | `5` | Initial perturbation magnitude |
| `hull_vertices` | five default points | Hull polygon before scaling |
| `leg_down`, `leg_w`, `leg_h` | `-8/scale`, `8/scale`, `34/scale` | Leg geometry |
| `viewport_w/h` | `600`, `400` | Velocity normalization dimensions |
| `terrain_step/length/height` | `14/scale`, `200`, `viewport_h/scale/4` | Track geometry |
| `terrain_grass/startpad` | `10`, `20` | Terrain generation parameters |
| `friction` | `2.5` | Terrain friction |
| `hardcore` | `false` | Generate harder terrain obstacles |
| `control_speed` | `false` | Select motor interpretation |
| `max_episode_steps` | `1600` | Truncation horizon |

All fields are public; builders cover the common mode, rate, horizon, scale, and hull changes. `Info` is `()`.

Reference: [Gymnasium BipedalWalker](https://gymnasium.farama.org/environments/box2d/bipedal_walker/).
