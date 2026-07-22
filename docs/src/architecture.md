# Architecture

OGym separates the environment contract, typed spaces, and simulation backends. Native environments implement their transition equations directly. Rapier environments own a `PhysicsWorld`; feature-gated MuJoCo environments own an `MjEnv` loaded from embedded or custom XML.

```text
Environment
├── EnvSpace<StateSpace, ActionSpace>
├── reset(seed) -> (state, info)
└── step(action) -> Experience
    ├── current and next state
    ├── reward and action
    ├── info
    └── Terminal
```

The crate also defines dynamic and const-generic batch *interfaces*. It does not currently ship a concrete `VecEnv` or native `CartPoleVec` implementation.

- [Core abstractions](architecture/core-abstractions.md) describes environment lifecycle and batch contracts.
- [Spaces](architecture/spaces.md) covers validation, sampling, and mixed action modes.
- [Backends and dynamics](architecture/backends.md) explains native, Rapier, and MuJoCo transitions.
- [Implementing an environment](architecture/implementing-environments.md) is the contributor checklist.
