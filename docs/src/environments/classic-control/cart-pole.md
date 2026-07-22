# CartPole

`CartPole` balances a pole hinged to a cart moving on a horizontal track.

## State and action spaces

The `SVector<f64, 4>` observation is \\([x,\dot x,\theta,\dot\theta]\\). Bounds are \\(x\in[-2x_{max},2x_{max}]\\), \\(\theta\in[-2\theta_{max},2\theta_{max}]\\), with velocity components represented by \\(\pm\texttt{f64::MAX}\\).

- Discrete: `0` applies \\(-f\\), `1` applies \\(+f\\).
- Continuous: \\(a\in[-1,1]\\) applies \\(F=af\\); out-of-range values are rejected.

Reset samples every state component uniformly from \\([-0.05,0.05]\\).

## Dynamics and reward

Let \\(m=m_c+m_p\\), \\(p=m_pl\\), and

\\[
T=\frac{F+p\dot\theta^2\sin\theta}{m}.
\\]

Then

\\[
\ddot\theta=\frac{g\sin\theta-\cos\theta\,T}{l(4/3-m_p\cos^2\theta/m)},
\qquad
\ddot x=T-\frac{p\ddot\theta\cos\theta}{m}.
\\]

Euler updates positions before velocities; semi-implicit Euler updates velocities first. Both use timestep `tau`. Every accepted step rewards \\(+1\\), including a terminal step.

## Episode end

Termination occurs when \\(|x|>x_{max}\\) or \\(|\theta|>\theta_{max}\\). Truncation occurs at `t_max` steps.

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `mc`, `mp` | `1.0`, `0.1` | Cart and pole mass |
| `l` | `0.5` | Half pole length used by the equations |
| `f`, `g` | `10`, `9.8` | Force magnitude and gravity |
| `tau` | `0.02` | Integration timestep |
| `x_max` | `2.4` | Cart termination threshold |
| `theta_max` | `12°` | Pole-angle threshold |
| `t_max` | `500` | Truncation horizon |
| `integrator` | `Euler` | `Euler` or `SemiImplicitEuler` |
| `continuous` | `false` | Select action mode |

Builders configure each physical parameter, thresholds, integrator, and action mode.

Reference: [Gymnasium CartPole](https://gymnasium.farama.org/environments/classic_control/cart_pole/).
