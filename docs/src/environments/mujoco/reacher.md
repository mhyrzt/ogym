# Reacher

## State and action spaces

The 10-value observation is

\\[
[\cos\theta_0,\cos\theta_1,\sin\theta_0,\sin\theta_1,
q_{target,x},q_{target,y},\dot\theta_0,\dot\theta_1,
p_{finger,x}-p_{target,x},p_{finger,y}-p_{target,y}].
\\]

The default action is two controls in \\([-1,1]\\).

## Dynamics and reward

\\[
r=-w_d\lVert p_{finger}-p_{target}\rVert_2-w_u\lVert u\rVert_2^2.
\\]

The two terms are returned in `Info`. Reset samples a target uniformly by rejection inside a radius-0.2 disc and adds small arm position/velocity noise.

## Episode end

There is no distance-based termination. The episode truncates at `max_episode_steps`.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `xml_file`, `frame_skip` | embedded, `2` | Model and action duration |
| `reward_dist_weight` | `1` | Distance penalty |
| `reward_control_weight` | `1` | Squared-control penalty |
| observation shape/dtype | `(10,)`, `f64` | Descriptive metadata |
| `max_episode_steps` | `50` | Horizon |

Custom XML must contain `target` and `fingertip` bodies and preserve the assumed joint layout.

Reference: [Gymnasium Reacher](https://gymnasium.farama.org/environments/mujoco/reacher/).
