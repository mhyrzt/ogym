# InvertedDoublePendulum

## State and action spaces

The fixed 9-value observation is

\\[
[x,\sin\theta_1,\sin\theta_2,\cos\theta_1,\cos\theta_2,
\operatorname{clip}(\dot q_0,-10,10),
\operatorname{clip}(\dot q_1,-10,10),
\operatorname{clip}(\dot q_2,-10,10),
\operatorname{clip}(f_{constraint,0},-10,10)].
\\]

The default action is one cart control in \\([-1,1]\\).

## Dynamics and reward

Let the free tip site position be \\((x_{tip},z_{tip})\\). Then

\\[
p_d=0.01x_{tip}^2+(z_{tip}-2)^2,
\qquad p_v=0.001\dot q_1^2+0.005\dot q_2^2,
\\]

\\[
r=r_{healthy}\mathbf{1}[z_{tip}>1]-p_d-p_v.
\\]

`Info` contains cart x position and velocity.

## Episode end

The task terminates when \\(z_{tip}\le1\\) and truncates at `max_steps`.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `5` | Model and action duration |
| `healthy_reward` | `10` | Upright bonus |
| `reset_noise_scale` | `0.1` | Uniform qpos and Gaussian qvel noise |
| `observation_shape/dtype` | `(9,)`, `f64` | Descriptive metadata; assembly is fixed by code |
| `max_steps` | `1000` | Horizon |

Reference: [Gymnasium InvertedDoublePendulum](https://gymnasium.farama.org/environments/mujoco/inverted_double_pendulum/).
