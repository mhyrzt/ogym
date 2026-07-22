# Toy Text

Toy Text environments are finite Markov decision processes with `u32` discrete actions. Grid states are row-major integers; Blackjack uses a `MultiDiscrete` observation.

| Environment | State space | Actions | Default horizon |
| --- | ---: | ---: | ---: |
| [Blackjack](toy-text/blackjack.md) | `MultiDiscrete([32,11,2])` | 2 | none |
| [CliffWalking](toy-text/cliff-walking.md) | `Discrete(nrow*ncol)` | 4 | 100 |
| [FrozenLake](toy-text/frozen-lake.md) | `Discrete(rows*cols)` | 4 | 100 |
| [Taxi](toy-text/taxi.md) | `Discrete(500)` | 6 | 200 |
