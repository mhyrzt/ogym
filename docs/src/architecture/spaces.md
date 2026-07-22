# Spaces

`Space` associates an `Item` type with four operations: `sample`, `contains`, `shape`, and `bounds`. `EnvSpace<S, A>` pairs the state and action spaces exposed by an environment.

| Type | Item | Meaning |
| --- | --- | --- |
| `Boxed<D>` | `SVector<f64, D>` | Fixed-size continuous box with elementwise bounds |
| `Discrete` | `u32` | Integer values \\(0,\ldots,n-1\\) |
| `MultiDiscrete` | `Vec<u32>` | Product of discrete ranges |
| `Mixed<D>` | `MixedItem<D>` | Either a discrete space or a continuous `Boxed<D>` selected at construction |

Classic Control and LunarLander use `Mixed` so action mode is encoded in both the configured space and the action value. Passing a continuous item to a discrete instance, or the reverse, is an error. MuJoCo environments use dynamically sized `DVector<f64>` actions because custom XML can change `nu`.
