# Benchmark results

Mean execution time ± standard deviation in milliseconds. Lower is better.

| Family | Environment | OGym (Rust) | Gymnasium (Python) | Speedup |
| --- | --- | ---: | ---: | ---: |
| Box2D / Rapier2D | BipedalWalker | 289.5 ± 33.9 | 1,948.0 ± 296.4 | 6.7× |
| Box2D / Rapier2D | LunarLander | 57.0 ± 8.6 | 635.4 ± 38.8 | 11.2× |
| MuJoCo | Ant | 1,550.0 ± 205.3 | 2,582.1 ± 182.9 | 1.7× |
| MuJoCo | HalfCheetah | 256.1 ± 49.2 | 1,039.1 ± 113.4 | 4.1× |
| MuJoCo | Hopper | 743.4 ± 180.8 | 1,577.1 ± 194.4 | 2.1× |
| MuJoCo | Humanoid | 2,293.5 ± 222.8 | 2,901.2 ± 287.8 | 1.3× |
| MuJoCo | HumanoidStandup | 3,480.2 ± 362.5 | 5,322.9 ± 800.4 | 1.5× |
| MuJoCo | InvertedDoublePendulum | 199.2 ± 21.5 | 1,255.7 ± 111.2 | 6.3× |
| MuJoCo | InvertedPendulum | 95.9 ± 33.6 | 1,268.6 ± 163.3 | 13.2× |
| MuJoCo | Pusher | 249.6 ± 30.6 | 1,118.2 ± 120.5 | 4.5× |
| MuJoCo | Reacher | 97.6 ± 11.4 | 947.0 ± 122.6 | 9.7× |
| MuJoCo | Swimmer | 235.6 ± 49.1 | 1,102.6 ± 127.7 | 4.7× |
| MuJoCo | Walker2d | 795.0 ± 107.5 | 1,588.4 ± 169.5 | 2.0× |
| Classic Control | Acrobot | 63.8 ± 10.4 | 9,553.7 ± 751.4 | 149.7× |
| Classic Control | CartPole | 12.7 ± 3.3 | 2,079.2 ± 210.4 | 163.5× |
| Classic Control | MountainCar | 3.6 ± 0.5 | 2,572.8 ± 415.6 | 705.2× |
| Classic Control | Pendulum | 6.7 ± 0.7 | 10,402.8 ± 1,749.9 | 1,550.4× |
| Toy Text | Blackjack | 29.2 ± 3.1 | 8,409.0 ± 855.1 | 288.4× |
| Toy Text | CliffWalking | 2.4 ± 0.8 | 2,267.3 ± 229.5 | 951.1× |
| Toy Text | FrozenLake | 11.1 ± 1.3 | 2,212.0 ± 332.8 | 199.6× |
| Toy Text | Taxi | 2.4 ± 0.9 | 2,377.4 ± 163.8 | 999.2× |
