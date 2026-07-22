# InvertedPendulum

## State and action spaces

The 4-value observation is all `qpos` followed by all `qvel`: cart position, pole angle, cart velocity, and pole angular velocity. The default action is one cart control in \([-3,3]\).

## Dynamics and reward

MuJoCo advances the cart-pole for two model steps by default. For observation \(s\),

\[
\text{terminated}=\neg\operatorname{finite}(s)\ \lor\ |s_1|>0.2,
\qquad
r=\begin{cases}0&\text{terminated}\\1&\text{otherwise.}\end{cases}
\]

`Info["reward_survive"]` contains this reward.

## Episode end

Non-finite state or pole angle beyond \(0.2\) radians terminates. The horizon truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `2` | Model and action duration |
| `reset_noise_scale` | `0.01` | Uniform qpos/qvel reset noise |
| observation shape/bounds | `(4,)`, `(-∞,∞)` | Descriptive metadata |
| `default_camera_config` | body 0, distance 2.04 | Rendering metadata |
| `max_episode_steps` | `1000` | Horizon |

Reference: [Gymnasium InvertedPendulum](https://gymnasium.farama.org/environments/mujoco/inverted_pendulum/).
