# Architecture

OGym is built around three core traits that provide a consistent interface
across its physics backends.

| Trait | Description |
| --- | --- |
| **`Space`** | Defines valid bounds for actions and observations, including sampling, validation, and shape checking. |
| **`Environment`** | Defines the standard interface for a single environment, including reset, step, and state management. |
| **`BatchEnvironment`** | Defines vectorized environments that manage multiple instances for high-throughput training. |
