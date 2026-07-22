# Walker2d

## State and action spaces

The default 17-value observation is `qpos[1..]` followed by all velocities clipped to \\([-10,10]\\); retaining root x produces 18 values. The default action is six controls in \\([-1,1]\\).

## Dynamics and reward

\\[
r=w_fv_x+r_{healthy}-w_u\lVert u\rVert_2^2.
\\]

Health requires root height strictly inside `healthy_z_range` and torso angle strictly inside `healthy_angle_range`. `Info` contains x position/velocity, z displacement from reset, and forward/control/survival components.

## Episode end

Unhealthy state terminates when configured; the horizon truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `4` | Model and action duration |
| forward/control weights | `1`, `0.001` | Reward coefficients |
| healthy reward | `1` | Survival reward |
| terminate unhealthy | `true` | Enable health termination |
| healthy z/angle ranges | `(0.8,2)`, `(-1,1)` | Strict health tests |
| reset noise | `0.005` | Uniform qpos/qvel noise |
| exclude root x | `true` | Observation switch |
| horizon | `1000` | Truncation limit |

Reference: [Gymnasium Walker2d](https://gymnasium.farama.org/environments/mujoco/walker2d/).
