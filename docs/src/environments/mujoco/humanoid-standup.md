# HumanoidStandup

## State and action spaces

The default 348-value observation has the same groups and switches as [Humanoid](humanoid.md): `qpos[2..]`, `qvel`, non-world `cinert`, non-world `cvel`, actuator generalized forces after the root DOFs, and non-world external forces. The default action is 17 controls in \\([-0.4,0.4]\\).

## Dynamics, reward, and info

Let \\(z\\) be root height, \\(h\\) the MuJoCo model timestep, \\(u\\) the simulator controls, and \\(f_c\\) external contact forces:

\\[
r=w_h\frac{z}{h}-w_u\lVert u\rVert_2^2-
\operatorname{clip}(w_i\lVert f_c\rVert_2^2,i_{min},i_{max})+1.
\\]

`Info` includes x/y position, distance from origin, summed tendon length/velocity, and upright/control/impact reward components.

## Episode end

There is no state termination. The horizon truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `5` | Model and action duration |
| `uph_cost_weight` | `1` | Upright reward weight |
| `ctrl_cost_weight` | `0.1` | Control penalty |
| `impact_cost_weight/range` | `5e-7`, `(-∞,10]` | Impact penalty and clamp |
| `reset_noise_scale` | `0.01` | Uniform qpos/qvel noise |
| exclude x/y | `true` | Observation root-position switch |
| include `cinert`, `cvel`, `qfrc_actuator`, `cfrc_ext` | all `true` | Auxiliary observation groups |
| `max_episode_steps` | `1000` | Horizon |

Reference: [Gymnasium HumanoidStandup](https://gymnasium.farama.org/environments/mujoco/humanoid_standup/).
