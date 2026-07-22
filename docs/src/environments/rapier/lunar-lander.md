# LunarLander

## State space

The `SVector<f32, 8>` observation is normalized horizontal and vertical displacement from the helipad, scaled horizontal and vertical velocity, angle, scaled angular velocity, and left/right leg-contact flags. Its declared bounds are \\([-2.5,2.5]^2\\), \\([-10,10]^2\\), \\([-2\pi,2\pi]\\), \\([-10,10]\\), and \\([0,1]^2\\).

## Action space

- Discrete: `0` idle, `1` left orientation engine, `2` main engine, `3` right orientation engine.
- Continuous: \\([a_m,a_s]\in[-1,1]^2\\). The main engine activates for \\(a_m>0\\) with power \\(0.5\operatorname{clip}(a_m,0,1)+0.5\\). The side engine activates for \\(|a_s|>0.5\\), with direction `sign(a_s)` and power \\(\operatorname{clip}(|a_s|,0.5,1)\\).

## Dynamics and reward

Engine impulses are applied at offset points, creating both translation and torque; gravity and optional wind/turbulence act before a Rapier step of \\(1/\text{fps}\\). For observation \\(s\\), shaping is

\\[
H(s)=-100\sqrt{s_0^2+s_1^2}-100\sqrt{s_2^2+s_3^2}-100|s_4|+10s_6+10s_7.
\\]

\\[
r_t=H(s_{t+1})-H(s_t)-0.30p_{main}-0.03p_{side}.
\\]

A successful landing overrides reward to \\(+100\\); a crash or leaving the horizontal screen overrides it to \\(-100\\). In the current collision handler, any started collision whose colliders both have parent bodies sets the crash flag; this is broader than hull-only collision and is an implementation difference to keep in mind when comparing with Gymnasium.

## Episode end

Termination occurs when the current crash flag is set, when the lander leaves the horizontal screen, or on a settled two-leg landing with near-zero linear and angular velocity. Truncation occurs at `max_steps`.

## Configuration

| Fields | Defaults | Meaning |
| --- | --- | --- |
| `scale`, `fps`, `max_steps` | `30`, `50`, `1000` | Geometry scale, simulation rate, horizon |
| `main_engine_force`, `side_engine_force` | `13`, `0.6` | Engine impulse scales |
| `side_engine_offset_x/y` | `12`, `14` | Side-thruster application offset |
| `main_engine_y_position` | `4` | Main-thruster offset |
| `leg_offset_x/y` | `20`, `18` | Leg attachment geometry |
| `leg_width/length` | `2`, `8` | Leg collider size |
| `leg_spring_torque` | `40` | Leg-joint motor strength |
| `gravity` | `-10` | Vertical acceleration |
| `continuous` | `false` | Select action representation |
| `wind_strength` | `None` | Optional horizontal wind amplitude |
| `turbulence_strength` | `1.5` | Angular disturbance amplitude |
| `initial_random` | `1000` | Initial random force magnitude |
| `viewport_width/height` | `600`, `400` | World scaling and normalization |

All fields are public and common values have builders. `Info` is `()`.

Reference: [Gymnasium LunarLander](https://gymnasium.farama.org/environments/box2d/lunar_lander/).
