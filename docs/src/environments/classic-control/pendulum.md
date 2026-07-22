# Pendulum

`Pendulum` applies torque to swing and hold a single link upright.

## State and action spaces

The `SVector<f64, 3>` observation is \\([\cos\theta,\sin\theta,\dot\theta]\\), bounded by \\([-1,1]\\), \\([-1,1]\\), and \\([-v_{max},v_{max}]\\).

- Continuous (default): torque \\(u\in[-\tau_{max},\tau_{max}]\\), with clipping.
- Discrete: `n` evenly spaced torques, \\(u(a)=2a\tau_{max}/(n-1)-\tau_{max}\\).

Reset samples \\(\theta\in[-x_0,x_0]\\) and \\(\dot\theta\in[-y_0,y_0]\\); a non-positive range fixes that component to zero.

## Dynamics and reward

With \\(\bar\theta\\) normalized to \\([-\pi,\pi)\\),

\\[
c=\bar\theta^2+0.1\dot\theta^2+0.001u^2,
\\]

\\[
\dot\theta_{t+1}=\operatorname{clip}\left(\dot\theta_t+\Delta t\left(\frac{3g}{2l}\sin\theta_t+\frac{3u}{ml^2}\right),-v_{max},v_{max}\right),
\\]

\\[
\theta_{t+1}=\theta_t+\Delta t\dot\theta_{t+1},\qquad r_t=-c.
\\]

## Episode end

The task never terminates from state. It truncates at `max_t` steps.

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `n` | `2` | Number of discrete torque levels |
| `g`, `m`, `l` | `10`, `1`, `1` | Gravity, mass, and length |
| `x0`, `y0` | `π`, `1` | Reset angle and velocity ranges |
| `dt` | `0.05` | Timestep |
| `max_v`, `max_tau` | `8`, `2` | Velocity and torque limits |
| `max_t` | `200` | Truncation horizon |
| `continuous` | `true` | Select action mode |
| `f` | `2` | Configured force value; current transition code uses `max_tau` instead |

Reference: [Gymnasium Pendulum](https://gymnasium.farama.org/environments/classic_control/pendulum/).
