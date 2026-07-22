---
id: TASK-17
title: Decouple non-MuJoCo benchmarks from MuJoCo
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 18:11'
updated_date: '2026-07-22 18:36'
labels: []
dependencies: []
modified_files:
  - benchmark/Cargo.toml
  - benchmark/Justfile
  - benchmark/src/main.rs
  - benchmark/README.md
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build and run non-MuJoCo benchmark environments without enabling or loading the native MuJoCo dependency, while retaining separate feature-enabled binaries for the MuJoCo family.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Blackjack and every other non-MuJoCo environment run without MuJoCo in the loader path
- [x] #2 Only the 11 MuJoCo environments compile with the benchmark mujoco feature and receive the native loader path
- [x] #3 Individual benchmark recipes select the correct binary automatically
- [x] #4 The comprehensive recipe combines default and MuJoCo binaries under the existing result names
- [x] #5 Both benchmark feature configurations compile and representative commands execute successfully
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Make MuJoCo an optional benchmark-crate feature and cfg-gate MuJoCo modules/dispatch. 2. Build default and MuJoCo benchmark binaries into separate target directories. 3. Tag environment metadata by backend so individual and comprehensive recipes choose the correct binary and loader configuration. 4. Reproduce the original no-loader failure, then verify non-MuJoCo execution without MuJoCo and representative MuJoCo execution with it.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reproduced the reported failure by running the old feature-enabled benchmark binary with LD_LIBRARY_PATH removed: Blackjack failed loading libmujoco.so.3.10.0. Root cause was benchmark/Cargo.toml enabling ogym/mujoco unconditionally, which linked every dispatch path to MuJoCo before main(). Added an optional benchmark mujoco feature, cfg-gated the 11 MuJoCo modules and dispatch arms, built default and MuJoCo binaries in separate target directories, and tagged Just environment metadata by backend. Verified ldd shows no MuJoCo dependency for the default binary, all 10 non-MuJoCo environments execute with LD_LIBRARY_PATH removed, Ant executes through the MuJoCo binary, and the comprehensive recipe generates correct commands via a fake Hyperfine probe.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Separated benchmark binaries by backend so only MuJoCo environments link and load the native library; all other environments now run independently of MuJoCo.
<!-- SECTION:FINAL_SUMMARY:END -->
