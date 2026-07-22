---
id: TASK-16
title: Wire root MuJoCo development recipes
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 18:05'
updated_date: '2026-07-22 18:06'
labels: []
dependencies: []
modified_files:
  - Justfile
  - build.rs
  - README.md
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add root-level MuJoCo build and test workflows with portable loader configuration, and make the crate build script honor the documented MUJOCO_DIR override.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root Justfile exposes MuJoCo build and test recipes
- [x] #2 Root MuJoCo recipes select LD_LIBRARY_PATH on Linux and DYLD_LIBRARY_PATH on macOS
- [x] #3 build.rs uses MUJOCO_DIR when set and ~/.local/mujoco otherwise
- [x] #4 MuJoCo recipes build and test successfully with the local installation
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add root MuJoCo directory and OS-specific loader variables. 2. Add build-mujoco and test-mujoco recipes using those variables. 3. Update build.rs to honor MUJOCO_DIR consistently with documentation. 4. Validate root Justfile expansion and real MuJoCo build/tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added root MuJoCo directory, loader-path, and combined environment variables with Linux/macOS detection and safe handling of an unset inherited loader path. build.rs now honors MUJOCO_DIR before falling back to HOME/.local/mujoco. Validation passed for Just and rustfmt checks, build-mujoco, and test-mujoco (326 tests).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added functional root MuJoCo build/test workflows and made native library discovery honor the documented configurable installation path.
<!-- SECTION:FINAL_SUMMARY:END -->
