# Acrobot

`Acrobot` is a two-link underactuated pendulum. Only the second joint is actuated.

## State space

The exposed `SVector<f64, 6>` is

\[
s=[\cos\theta_1,\sin\theta_1,\cos\theta_2,\sin\theta_2,\dot\theta_1,\dot\theta_2].
\]

The first four bounds are \([-1,1]\); velocity bounds default to \(\pm4\pi\) and \(\pm9\pi\). Reset samples the latent \([\theta_1,\theta_2,\dot\theta_1,\dot\theta_2]\) uniformly from \([-0.1,0.1]^4\).

## Action space

- Discrete: `0`, `1`, `2` map to torque \(-1,0,+1\).
- Continuous: one value in \([-1,1]\), clipped to the interval.

Uniform noise in \((-n,n)\) is added when `torque_noise_max = n > 0`.

## Dynamics and reward

Let \(m_i,l_1,c_i,I,g\) come from the config and define

\[
d_1=m_1c_1^2+m_2(l_1^2+c_2^2+2l_1c_2\cos\theta_2)+2I,
\quad d_2=m_2(c_2^2+l_1c_2\cos\theta_2)+I.
\]

\[
\phi_2=m_2c_2g\cos(\theta_1+\theta_2-\pi/2),
\]

\[
\phi_1=-m_2l_1c_2\dot\theta_2^2\sin\theta_2-2m_2l_1c_2\dot\theta_1\dot\theta_2\sin\theta_2+(m_1c_1+m_2l_1)g\cos(\theta_1-\pi/2)+\phi_2.
\]

`Book` dynamics use

\[
\ddot\theta_2=\frac{\tau+(d_2/d_1)\phi_1-m_2l_1c_2\dot\theta_1^2\sin\theta_2-\phi_2}{m_2c_2^2+I-d_2^2/d_1},
\]

while `Nips` omits the \(m_2l_1c_2\dot\theta_1^2\sin\theta_2\) term. In both cases \(\ddot\theta_1=-(d_2\ddot\theta_2+\phi_1)/d_1\). RK4 integrates with `dt`; angles wrap to \([-\pi,\pi)\) and velocities are clipped. Reward is \(-1\) until the terminal transition, which receives \(0\).

## Episode end

Termination occurs when

\[
\cos\theta_1+\cos(\theta_1+\theta_2)<-1.
\]

Truncation occurs at `max_t` steps (default 500).

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `g`, `dt` | `9.8`, `0.2` | Gravity and RK4 step |
| `link_length_1/2` | `1`, `1` | Link lengths |
| `link_mass_1/2` | `1`, `1` | Link masses |
| `link_com_pos_1/2` | `0.5`, `0.5` | COM offsets |
| `link_moi` | `1` | Link moment of inertia used for both links |
| `max_vel_1/2` | `4π`, `9π` | Angular-velocity clipping |
| `torque_noise_max` | `0` | Uniform torque-noise magnitude |
| `dynamics_mode` | `Book` | `Book` or `Nips` acceleration equation |
| `continuous` | `false` | Select action representation |
| `max_t` | `500` | Truncation horizon |

The config provides corresponding `with_*` builders plus `use_book_dynamics()` and `use_nips_dynamics()`.

Reference: [Gymnasium Acrobot](https://gymnasium.farama.org/environments/classic_control/acrobot/).
