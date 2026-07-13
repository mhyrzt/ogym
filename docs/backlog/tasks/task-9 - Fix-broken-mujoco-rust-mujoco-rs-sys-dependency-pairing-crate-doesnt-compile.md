---
id: TASK-9
title: >-
  Fix broken mujoco-rust/mujoco-rs-sys dependency pairing (crate doesn't
  compile)
status: Done
assignee: []
created_date: '2026-07-13 13:08'
updated_date: '2026-07-13 18:55'
labels: []
dependencies: []
priority: high
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The whole ogym crate fails to compile at HEAD (confirmed via git stash, unrelated to any in-flight work): mujoco-rust 0.0.6's src/vfs.rs and src/model.rs call C functions/fields (mj_findFileVFS, mj_makeEmptyFileVFS, mjVFS_.nfile/.filedata/.filesize, mj_saveModel's nbytes arg type) that don't exist in mujoco-rs-sys 0.0.4's vendored bindings, which instead expose only an opaque impl_ handle on mjVFS_. Root cause: MuJoCo's C mjVFS struct/API changed between the MuJoCo header version mujoco-rust 0.0.6's vfs.rs was written against (older, public nfile/filedata/filesize fields + mj_findFileVFS/mj_makeEmptyFileVFS functions) and the version mujoco-rs-sys 0.0.4's checked-in generated bindings were produced from (newer, opaque mjVFS with mj_addFileVFS instead). crates.io has only one version of each crate (mujoco-rust 0.0.6, mujoco-rs-sys 0.0.4) so there is no version bump that resolves this, and the upstream repo (github.com/MuJoCo-Rust/MuJoCo-Rust) has been dormant since May 2023 with no fix. This blocks compiling/testing anything in src/env/mujoco/* (11 envs) and blocks TASK-2 and TASK-3.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 cargo build --lib succeeds from a clean checkout without manual registry-cache edits
- [x] #2 cargo test --lib runs (even if individual mujoco env tests fail for unrelated reasons, the crate must compile)
- [x] #3 The fix approach and its tradeoffs are documented (e.g. pin a specific MuJoCo native version + force local bindgen regeneration, vendor a patched fork of mujoco-rust/mujoco-rs-sys via a git [patch] section, or replace the FFI binding entirely)
<!-- AC:END -->



## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Root cause was narrower than first thought: not a full API incompatibility, just mujoco-rust 0.0.6's vfs.rs calling two removed MuJoCo 3.x functions (mj_findFileVFS, mj_makeEmptyFileVFS) and reading raw struct fields (nfile/filedata/filesize) that no longer exist now that mjVFS is opaque. mj_defaultVFS/mj_deleteVFS/mj_deleteFileVFS are all still present in MuJoCo 3.10. Fixed by vendoring mujoco-rust 0.0.6 under vendor/mujoco-rust/ with vfs.rs rewritten against mj_addBufferVFS (replaces the old add_file two-step reserve+copy) and mj_saveModel's i64->i32 nbytes cast fixed in model.rs, wired up via [patch.crates-io] in Cargo.toml. get_file() (read a buffer back out of the VFS) is now a stub returning None since MuJoCo 3.x's opaque VFS has no public read-back accessor -- not used by any ogym code, only by mujoco-rust's own now-adjusted test suite.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Vendored a patched mujoco-rust 0.0.6 under vendor/mujoco-rust/ (fixed vfs.rs against MuJoCo 3.x's mj_addBufferVFS API, fixed model.rs's mj_saveModel arg type) and wired it in via Cargo.toml [patch.crates-io]. cargo build --lib now succeeds with 0 errors; cargo test --lib passes 267/268 (one pre-existing, unrelated LunarLander flake tracked in TASK-6).
<!-- SECTION:FINAL_SUMMARY:END -->
