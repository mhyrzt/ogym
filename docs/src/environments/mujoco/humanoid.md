# Humanoid

## State and action spaces

The default 348-value observation concatenates `qpos[2..]`, all `qvel`, non-world body inertias (`cinert`), non-world body spatial velocities (`cvel`), actuator generalized forces after the six root DOFs, and non-world external contact forces. Configuration can retain root x/y or omit any auxiliary group. The default action is 17 controls in \([-0.4,0.4]\).

## Dynamics, reward, and info

Horizontal velocity is computed from the mass-weighted body center. With simulator control \(u\) and external forces \(f_c\),

\[
r=w_fv_x+r_{healthy}-w_u\lVert u\rVert_2^2-
\operatorname{clip}(w_c\lVert f_c\rVert_2^2,c_{min},c_{max}).
\]

Health is strict root height \(z\in(z_{min},z_{max})\). `Info` exposes xyz position, xy velocity, summed tendon length/velocity, and reward components.

## Episode end

Unhealthy state terminates when configured; the horizon truncates.

## Configuration

`HumanoidConfig` exposes builders for:

| Setting | Default |
| --- | --- |
| XML / frame skip | embedded / `5` |
| forward, control, contact, healthy weights | `1.25`, `0.1`, `5e-7`, `5` |
| contact-cost clamp | `(-∞,10]` |
| healthy z range / terminate unhealthy | `(1,2)` / `true` |
| reset noise | `0.01` uniform qpos/qvel |
| exclude x/y | `true` |
| include `cinert`, `cvel`, `qfrc_actuator`, `cfrc_ext` | all `true` |
| horizon | `1000` |

Reference: [Gymnasium Humanoid](https://gymnasium.farama.org/environments/mujoco/humanoid/).
