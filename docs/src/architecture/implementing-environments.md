# Implementing an environment

Place a new environment under the appropriate backend with `config.rs`, `env.rs`, and `mod.rs`; keep model assets and backend helpers beside it.

1. Define a configuration with documented defaults and meaningful builder methods where fields are private.
2. Define exact `State`, `Action`, and `Info` types and an `EnvSpace` when the dimensions are static.
3. Make seeded resets reproducible, clear the step counter, and initialize every backend state used by `state()`.
4. Validate action kind, dimension, and bounds before applying it.
5. Return an `Experience` containing the pre-step and post-step observations and a correct `Terminal` value.
6. Keep task termination separate from the configured time limit.
7. Test valid transitions, invalid actions, boundaries, seeded resets, rewards, and simultaneous terminal/truncation behavior.
8. Export the environment and config from `mod.rs` and add a specification page using the common environment template.

For MuJoCo, gate exports behind the `mujoco` feature, embed the default XML, derive dimensions from the loaded model, and document required body/site names for custom XML.
