---
id: TASK-14
title: Add root Justfile and portable MuJoCo loader
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 17:54'
updated_date: '2026-07-22 17:59'
labels: []
dependencies: []
modified_files:
  - Justfile
  - benchmark/Justfile
  - README.md
  - benchmark/README.md
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Provide a root development command interface and make benchmark commands select the correct dynamic-library environment variable for the host operating system.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root Justfile exposes common Rust, documentation, and benchmark workflows
- [x] #2 Benchmark commands use LD_LIBRARY_PATH on Linux and DYLD_LIBRARY_PATH on macOS automatically
- [x] #3 Benchmark recipes remain callable directly and through the root Justfile
- [x] #4 Justfiles parse and relevant build checks pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add host-OS detection and loader environment selection to benchmark/Justfile. 2. Add a root Justfile for standard Rust, docs, and delegated benchmark workflows. 3. Validate direct and delegated recipes, Cargo checks, and mdBook.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added just os() detection: Linux uses LD_LIBRARY_PATH and macOS uses DYLD_LIBRARY_PATH; MUJOCO_DIR can override ~/.local/mujoco. Root recipes delegate benchmarks to benchmark/Justfile. Validation passed for both Justfile format/parser checks, Linux loader evaluation, Cargo check, 272 tests, Clippy (warnings only), mdBook, and direct/root benchmark invocation. GitHub Actions inspection found CI failing only rustfmt and Docs failing only because Pages is not enabled; these pre-existing issues are outside this task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a documented root Justfile and portable benchmark MuJoCo loader configuration with Linux/macOS auto-detection and a configurable installation directory.
<!-- SECTION:FINAL_SUMMARY:END -->
