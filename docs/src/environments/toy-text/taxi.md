# Taxi

## State and action spaces

The 500 states encode taxi row \(r\), column \(c\), passenger index \(p\in\{R,G,Y,B,\text{taxi}\}\), and destination \(d\in\{R,G,Y,B\}\):

\[
s=4\left(5(5r+c)+p\right)+d.
\]

Actions are `0 = south`, `1 = north`, `2 = east`, `3 = west`, `4 = pickup`, and `5 = dropoff`. Grid boundaries and the fixed interior walls block movement.

## Dynamics and reward

Movement and a legal pickup reward \(-1\). Illegal pickup/dropoff rewards \(-10\). Dropping the carried passenger at the destination rewards \(+20\) and terminates. Dropping at another named location leaves the passenger there with the ordinary \(-1\) reward.

Reset samples taxi position, destination, and a distinct waiting location from a seeded RNG.

## Episode end

Successful delivery terminates. Reaching `max_episode_steps` truncates and may coincide with delivery.

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `max_episode_steps` | `200` | Truncation horizon |

`Info` is `()`. Reference: [Gymnasium Taxi](https://gymnasium.farama.org/environments/toy_text/taxi/).
