---
id: TASK-15
title: Deduplicate benchmark Justfile recipes
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 18:01'
updated_date: '2026-07-22 18:03'
labels: []
dependencies: []
modified_files:
  - benchmark/Justfile
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Refactor the benchmark task runner so environment metadata and Hyperfine invocation logic are defined once while preserving the existing command interface and result schema.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Every existing environment recipe remains available
- [x] #2 Individual recipes share one comparison implementation
- [x] #3 The all recipe builds its Hyperfine command list from one compact environment table
- [x] #4 Individual and all dry runs preserve command names, environment arguments, result paths, and portable MuJoCo loading
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Replace repeated individual Hyperfine blocks with one private parameterized recipe and thin aliases. 2. Generate the comprehensive Hyperfine argument list from a compact environment table in Bash. 3. Verify recipe parity, dry-run expansion, formatting, and focused execution without touching stored benchmark results.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reduced benchmark/Justfile to 118 lines by routing all 21 public aliases through one private _compare recipe and generating comprehensive Hyperfine arguments from one label/environment table. Applied the portable MuJoCo loader to every Rust invocation because the shared benchmark binary links MuJoCo regardless of selected environment. Validation passed for Just formatting, recipe listing, direct and root dry runs, release build, and a fake-Hyperfine execution that expanded the full argument list without writing results.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Deduplicated individual and comprehensive benchmark recipes while preserving public commands, result names, and portable loader behavior.
<!-- SECTION:FINAL_SUMMARY:END -->
