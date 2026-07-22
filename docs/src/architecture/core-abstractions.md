# Core abstractions

## `Environment`

`Environment` uses associated `Action`, `State`, and `Info` types. `reset(seed)` initializes an episode; `step(action)` produces an `Experience`; `state()` reads the current observation. `is_terminal()` and `is_truncated()` remain separate, while `is_done()` is their disjunction and `to_terminal()` preserves both flags.

Calling `state()` or `step()` before initialization may return `NotInitialized`; stepping an already completed native environment may return `EpisodeDone`; invalid kinds, dimensions, or values produce typed errors.

## `Experience`

`Experience<S, I, A>` contains `curr_state`, `reward: f64`, `action`, `next_state`, `info`, `terminal`, and `step`. The `step` field is environment supplied; consumers should not infer it from vector indices.

## `Terminal`

`Terminal` has `Ongoing`, `Terminate`, `Truncate`, and `Both` variants. `is_terminated()`, `is_truncated()`, and `is_done()` expose the three useful predicates. `from_flags(terminated, truncated)` retains simultaneous termination and truncation.

## Batch traits

`BatchEnvironment` uses runtime-sized `Vec` collections; `StaticBatchEnvironment<N>` uses arrays and a const batch size. Both define reset, batched stepping, state access, completion flags, and selective reset contracts. They are extension interfaces: OGym currently provides no concrete batch wrapper.

See the [generated API documentation](../api/ogym/env/environment/index.html) for exact signatures.
