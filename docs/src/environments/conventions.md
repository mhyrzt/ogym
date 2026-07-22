# Notation and conventions

OGym calls the agent-visible observation a **state** in its Rust API. This documentation writes it as \\(s_t\\), while \\(x_t\\) denotes a backend's latent state when the distinction matters. An interaction is

\\[
(s_{t+1}, r_t, d_t, i_t) = \operatorname{step}(s_t, a_t),
\\]

where \\(a_t\\) is the action, \\(r_t\\) the reward, \\(d_t\\) the [`Terminal`](../architecture/core-abstractions.md#terminal) status, and \\(i_t\\) the environment-specific information value.

- `Terminate` means an environment-defined terminal event occurred.
- `Truncate` means an external horizon was reached.
- `Both` records both events on the same transition.
- `Ongoing` records neither event.

Intervals use \\([l,h]\\) for inclusive space bounds and \\((l,h)\\) when the health test uses strict inequalities. `clip(x,l,h)` limits a scalar to that interval. \\(\lVert x\rVert_2\\) is the Euclidean norm and \\(\lVert x\rVert_2^2=\sum_i x_i^2\\).

For MuJoCo, \\(q\\), \\(\dot q\\), and \\(u\\) denote generalized positions, velocities, and actuator controls. A default-model dimension is not a promise for custom XML: the action length is always `nu`, and observation length is assembled from the loaded model.
