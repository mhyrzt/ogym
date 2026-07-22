# Blackjack

## State and action spaces

The `Vec<u32>` state is `[player_sum, dealer_showing, usable_ace]` in `MultiDiscrete([32,11,2])`. Actions are `0 = stick` and `1 = hit`. Cards are drawn uniformly from the infinite-deck table `[A,2,...,9,10,J,Q,K]`, where all face cards count as 10.

## Dynamics and reward

A usable ace contributes 11 when doing so keeps the hand at or below 21. On hit, draw \\(c\sim\text{Deck}\\), append it, and return reward \\(-1\\) if the player busts or \\(0\\) otherwise. On stick, the dealer draws until its sum is at least 17, then

\\[
r=\operatorname{sign}(\operatorname{score}\_{player}-\operatorname{score}\_{dealer}).
\\]

With `sab`, a player natural against a non-natural dealer is an outright \\(+1\\). Otherwise, when `natural` is enabled, a natural that already wins pays \\(+1.5\\).

## Episode end

The episode terminates when a hit busts the player or immediately after stick. It never truncates.

## Configuration

| Field | Default | Meaning |
| --- | --- | --- |
| `natural` | `false` | Pay 1.5 for a natural win when `sab` is off |
| `sab` | `false` | Use Sutton-and-Barto natural precedence; overrides `natural` |

`Info` is `()`. Reference: [Gymnasium Blackjack](https://gymnasium.farama.org/environments/toy_text/blackjack/).
