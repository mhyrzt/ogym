# Pusher

## State and action spaces

The 23-value observation is the seven arm joint positions, seven arm velocities, fingertip COM (3), object COM (3), and goal COM (3). Object/goal slide-joint coordinates are deliberately omitted. The default action is seven controls in \\([-2,2]\\).

## Dynamics and reward

For fingertip \\(p_f\\), object \\(p_o\\), goal \\(p_g\\), and control \\(u\\),

\\[
r=-w_n\lVert p_o-p_f\rVert_2-w_d\lVert p_g-p_o\rVert_2-w_u\lVert u\rVert_2^2.
\\]

The three signed components are returned as `reward_near`, `reward_dist`, and `reward_ctrl`. Reset randomizes the object in the configured task region while the goal remains fixed.

## Episode end

There is no success termination. The task truncates at `max_episode_steps`.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `5` | Model and action duration |
| `reward_near_weight` | `0.5` | Fingertip-to-object penalty |
| `reward_dist_weight` | `1` | Object-to-goal penalty |
| `reward_control_weight` | `0.1` | Squared-control penalty |
| observation shape/dtype | `(23,)`, `f64` | Descriptive metadata |
| `max_episode_steps` | `100` | Horizon |

Custom XML must contain bodies named `object`, `goal`, and `tips_arm` and preserve the assumed seven arm joints.

Reference: [Gymnasium Pusher](https://gymnasium.farama.org/environments/mujoco/pusher/).
