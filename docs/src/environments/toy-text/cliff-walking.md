# CliffWalking

## State and action spaces

For an `nrow × ncol` grid, state \\(s=r\,n_{col}+c\\) belongs to `Discrete(nrow*ncol)`. Actions are `0 = up`, `1 = right`, `2 = down`, and `3 = left`; boundary moves stay in place. Start is bottom-left and goal is bottom-right. The bottom-row cells between them are the cliff.

## Dynamics and reward

Let \\(\delta(a)\\) be the selected unit grid displacement. The candidate cell is

\\[
(r',c')=\operatorname{clip}_{grid}((r,c)+\delta(a)).
\\]

Entering the cliff returns to start with reward \\(-100\\) and does **not** terminate. Every other move rewards \\(-1\\).

## Episode end

Reaching the goal terminates. The step that reaches `max_episode_steps` truncates, so it may produce `Both` if it also reaches the goal.

## Configuration

| Field | Default | Meaning |
| --- | ---: | --- |
| `nrow`, `ncol` | `4`, `12` | Grid dimensions |
| `max_episode_steps` | `100` | Truncation horizon |

`Info` is `()`. Reference: [Gymnasium CliffWalking](https://gymnasium.farama.org/environments/toy_text/cliff_walking/).
