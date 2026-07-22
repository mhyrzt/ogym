# Ant

## State and action spaces

With defaults, the 105-value observation concatenates `qpos[2..]` (13), all `qvel` (14), and the six external contact-force components for each non-world body (78), clipped to \([-1,1]\). Including root x/y adds two values; disabling contact forces removes 78. The default action has 8 actuator controls in \([-1,1]\).

## Dynamics, reward, and info

For torso displacement over agent timestep \(\Delta t\), \(v_x=(x_{t+1}-x_t)/\Delta t\). With clipped full-model contact-force vector \(f_c\),

\[
r=w_fv_x+r_{healthy}-w_u\lVert u\rVert_2^2-w_c\lVert f_c\rVert_2^2.
\]

Healthy means every simulator-state value is finite and root height \(z\in[z_{min},z_{max}]\). `Info` reports x/y position and velocity, distance from origin, and forward, control, contact, and survival reward components.

## Episode end

When enabled, leaving the healthy range terminates. `max_episode_steps` truncates.

## Configuration

`AntConfig` uses builders because its fields are private to the crate.

| Setting | Default | Meaning |
| --- | ---: | --- |
| XML, `frame_skip` | embedded model, `5` | Model and agent timestep |
| forward/control/contact weights | `1`, `0.5`, `0.0005` | Reward coefficients |
| healthy reward/range | `1`, `[0.2,1.0]` | Survival reward and root-height test |
| terminate unhealthy | `true` | Enable health termination |
| contact force range | `[-1,1]` | Force clipping; fixed by the default config API |
| reset noise | `0.1` | Uniform qpos and Gaussian qvel noise scale |
| exclude root x/y | `true` | Omit the first two qpos entries |
| include contact forces | `true` | Append non-world `cfrc_ext` |
| horizon | `1000` | Truncation limit |

Reference: [Gymnasium Ant](https://gymnasium.farama.org/environments/mujoco/ant/).
