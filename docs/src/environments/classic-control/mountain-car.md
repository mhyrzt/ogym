# MountainCar

`MountainCar` drives an underpowered car out of a valley by building momentum.

## State and action spaces

The state is `SVector<f64, 2>`: \\([x,v]\\), bounded by `min_x..max_x` and \\([-max_v,max_v]\\). Reset samples \\(x\sim U(-0.6,-0.4)\\) and sets \\(v=0\\).

- Discrete: `0`, `1`, `2` map to direction \\(-1,0,+1\\).
- Continuous: one command \\(a\in[-1,1]\\); out-of-range values are rejected.

## Dynamics and reward

For mapped force command \\(u\\),

\\[
v_{t+1}=\operatorname{clip}(v_t+f u-g\cos(3x_t),-v_{max},v_{max}),
\\]

\\[
x_{t+1}=\operatorname{clip}(x_t+v_{t+1},x_{min},x_{max}).
\\]

At the left boundary, negative velocity is reset to zero. `Constant` reward is always \\(-1\\). `ActionPenalty` is \\(-0.1u^2\\) plus \\(+100\\) on the terminal transition.

## Episode end

Termination requires both \\(x\ge x_{goal}\\) and \\(v\ge v_{goal}\\). Truncation occurs at `max_t` steps.

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `f`, `g` | `0.001`, `0.0025` | Engine and hill-gravity coefficients |
| `min_x`, `max_x` | `-1.2`, `0.6` | Position bounds |
| `max_v` | `0.07` | Speed bound |
| `goal_x`, `goal_v` | `0.5`, `0` | Terminal thresholds |
| `max_t` | `200` | Truncation horizon |
| `continuous` | `false` | Select action mode |
| `reward` | `Constant` | `Constant` or `ActionPenalty` |

`with_continuous_action()` applies the continuous-task preset: `f=0.0015`, `goal_x=0.45`, `max_t=999`, and `ActionPenalty`. `with_discrete_action()` restores the discrete preset. Apply mode selection before later overrides because it replaces those values.

Reference: [Gymnasium MountainCar](https://gymnasium.farama.org/environments/classic_control/mountain_car/).
