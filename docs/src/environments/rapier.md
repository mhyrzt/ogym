# Rapier environments

These tasks use Rapier2D rigid bodies, colliders, revolute joints, collision events, and ray queries. The shared equations and timestep model are described in [Backends and dynamics](../architecture/backends.md#rapier2d).

| Environment | Observation | Action | Horizon |
| --- | ---: | --- | ---: |
| [BipedalWalker](rapier/bipedal-walker.md) | 24 values | 4 continuous motor commands | 1600 |
| [LunarLander](rapier/lunar-lander.md) | 8 values | 4 discrete or 2 continuous controls | 1000 |
