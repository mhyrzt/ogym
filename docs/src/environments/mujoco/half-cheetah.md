# HalfCheetah

## State and action spaces

The default 17-value observation is `qpos[1..]` followed by all `qvel`; retaining root x produces 18 values. The default action is 6 controls in \([-1,1]\).

## Dynamics, reward, and info

\[
v_x=\frac{x_{t+1}-x_t}{\Delta t},\qquad
r=w_fv_x-w_u\lVert u\rVert_2^2.
\]

`Info` contains `x_position`, `x_velocity`, `reward_forward`, and `reward_ctrl`. Reset adds uniform position noise and Gaussian velocity noise of configured scale.

## Episode end

There is no state termination. The episode truncates at `max_episode_steps`.

## Configuration

| Builder setting | Default | Meaning |
| --- | ---: | --- |
| XML | embedded model | Custom model source |
| `frame_skip` | `5` | Positive number of MuJoCo steps per action |
| forward weight | `1` | Velocity reward coefficient |
| control weight | `0.1` | Squared-action penalty coefficient |
| reset noise | `0.1` | Non-negative initialization-noise scale |
| position exclusion | `true` | Omit root x from observation |
| horizon | `1000` | Truncation limit |

Reference: [Gymnasium HalfCheetah](https://gymnasium.farama.org/environments/mujoco/half_cheetah/).
