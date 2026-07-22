# FrozenLake

## State and action spaces

State \(s=r\,n_{col}+c\) is row-major. Actions are `0 = left`, `1 = down`, `2 = right`, and `3 = up`. The map uses `S` (start), `F` (frozen), `H` (hole), and `G` (goal).

## Dynamics and reward

Without slipperiness, the requested direction is applied with boundary clipping. With `is_slippery`, the actual direction is uniformly selected from

\[
\{(a-1)\bmod4,\ a,\ (a+1)\bmod4\},
\qquad P=1/3\text{ each}.
\]

The reward is \(1\) on `G` and \(0\) everywhere else.

## Episode end

Entering `H` or `G` terminates. Reaching `max_episode_steps` truncates and can coincide with termination.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `map` | `SFFF/FHFH/FFFH/HFFG` | Rectangular row-major byte grid |
| `is_slippery` | `true` | Enable the three-direction transition distribution |
| `max_episode_steps` | `100` | Truncation horizon |

The state-space size and start location are derived from `map`; provide a non-empty rectangular map containing `S`. `Info` is `()`.

Reference: [Gymnasium FrozenLake](https://gymnasium.farama.org/environments/toy_text/frozen_lake/).
