# Hopper

## State and action spaces

The default 11-value observation is `qpos[1..]` followed by all velocities clipped to \\([-10,10]\\); retaining root x adds one value. The default action is 3 controls in \\([-1,1]\\).

## Dynamics and reward

\\[
r=w_fv_x+r_{healthy}-w_u\lVert u\rVert_2^2.
\\]

Health requires every simulator-state component after the first two positions to lie strictly in `healthy_state_range`, root height in `healthy_z_range`, and torso angle in `healthy_angle_range`. `Info` reports x position/velocity, z displacement from reset, and reward components.

## Episode end

Unhealthy state terminates when `terminate_when_unhealthy` is true. The horizon truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `4` | Model and action duration |
| `forward_reward_weight` | `1` | Velocity reward coefficient |
| `ctrl_cost_weight` | `0.001` | Control penalty coefficient |
| `healthy_reward` | `1` | Reward while healthy |
| `terminate_when_unhealthy` | `true` | Enable health termination |
| `healthy_state_range` | `(-100,100)` | Strict finite-state bounds |
| `healthy_z_range` | `(0.7,∞)` | Strict height range |
| `healthy_angle_range` | `(-0.2,0.2)` | Strict torso-angle range |
| `reset_noise_scale` | `0.005` | Uniform qpos/qvel noise |
| `exclude_current_positions_from_observation` | `true` | Omit root x |
| `max_episode_steps` | `1000` | Horizon |

Reference: [Gymnasium Hopper](https://gymnasium.farama.org/environments/mujoco/hopper/).
