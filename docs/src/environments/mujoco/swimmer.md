# Swimmer

## State and action spaces

The default 8-value observation concatenates `qpos[2..]` with all `qvel`, omitting root x/y. Keeping those positions produces 10 values. The default action is two controls in \([-1,1]\).

## Dynamics and reward

\[
v_x=\frac{x_{t+1}-x_t}{\Delta t},\qquad
r=w_fv_x-w_u\lVert u\rVert_2^2.
\]

`Info` contains x position and the forward/control reward components.

## Episode end

There is no state termination. The horizon truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `4` | Model and action duration |
| `forward_reward_weight` | `1` | Velocity reward coefficient |
| `ctrl_cost_weight` | `0.0001` | Control penalty coefficient |
| `reset_noise_scale` | `0.1` | Uniform qpos/qvel reset noise |
| `exclude_current_positions_from_observation` | `true` | Omit root x/y |
| `max_episode_steps` | `1000` | Horizon |

Reference: [Gymnasium Swimmer](https://gymnasium.farama.org/environments/mujoco/swimmer/).
