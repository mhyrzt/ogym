# Introduction

OGym is a Rust-native reinforcement learning framework inspired by OpenAI Gym.
It provides a flexible, type-safe, and efficient platform for building,
training, and interacting with reinforcement learning environments.

Built entirely in Rust, OGym uses ownership and zero-cost abstractions to
provide thread safety and strong performance, particularly for parallel
environment execution.

## Performance

![OGym and Gymnasium benchmark comparison](https://raw.githubusercontent.com/mhyrzt/ogym/main/benchmark/results/benchmark.png)

## Features

- **Type safety first:** Strongly typed actions and observations eliminate
  common runtime shape errors.
- **High performance:** Compiled environments run with minimal overhead.
- **Native vectorization:** Batched environments support parallel processing
  and matrix operations.
- **Modular backends:** The architecture supports classic control systems and
  multiple physics engines, including Rapier and MuJoCo.
- **Zero-cost abstractions:** Idiomatic Rust patterns provide safety without
  sacrificing performance.
