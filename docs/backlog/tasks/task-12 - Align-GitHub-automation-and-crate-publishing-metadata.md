---
id: TASK-12
title: Align GitHub automation and crate publishing metadata
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 15:07'
updated_date: '2026-07-22 15:32'
labels:
  - ci
  - release
  - packaging
dependencies: []
references:
  - 'https://doc.rust-lang.org/cargo/reference/manifest.html'
  - 'https://doc.rust-lang.org/cargo/reference/publishing.html'
modified_files:
  - Cargo.toml
  - Cargo.lock
  - README.md
  - build.rs
  - src/env/mod.rs
  - .github/workflows/ci.yml
  - .github/workflows/docs.yml
  - .github/workflows/publish.yml
  - .github/workflows/release.yml
  - .github/RELEASE_NOTE.md
priority: medium
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Adapt the imported GitHub workflows and release notes from xrat to OGym, and improve Cargo.toml for reliable crates.io packaging and documentation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CI validates formatting, linting, tests, and crate packaging for OGym
- [x] #2 Documentation automation matches OGym's Rust API documentation setup
- [x] #3 Tag releases publish the OGym crate once and create an OGym GitHub release without xrat binaries or Docker jobs
- [x] #4 Cargo.toml contains valid, useful crates.io metadata and a publishable package file set
- [x] #5 Relevant local validation passes or remaining environmental blockers are documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit imported workflows and existing publishing setup. 2. Verify current Cargo/crates.io and GitHub Actions guidance. 3. Make MuJoCo an explicit opt-in feature so the default crates.io package can verify independently of the broken published mujoco-rust combination. 4. Correct Cargo package metadata/file inclusion and document the feature. 5. Replace xrat CI, docs, and release automation with OGym-specific jobs and a single publish path. 6. Validate formatting, linting, tests, workflow syntax, and cargo package contents.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Adapted imported xrat automation to OGym. The published mujoco-rust 0.0.6 dependency fails package verification against mujoco-rs-sys 0.0.4, so MuJoCo is now opt-in and the default package is independent of that native stack. Removed the duplicate publish workflow to ensure one tag-driven crates.io upload.

Validation: cargo test --locked passed 269 tests; cargo package --allow-dirty --locked verified the generated 85.7 KiB crate; cargo doc --no-deps --locked passed; cargo clippy --all-targets --locked passed with 34 existing warnings; actionlint and YAML parsing passed; no stale xrat/master/mdBook/Docker references remain in .github. cargo fmt --all --check remains blocked by pre-existing formatting differences in unrelated user-edited environment files, which were intentionally preserved.

Validation: cargo check passed; default cargo test passed 269 tests; cargo test --features mujoco passed 323 tests against official MuJoCo 3.9.0; cargo package verified a 85.2 KiB crate; cargo doc passed; default Clippy completed with 0 errors and 34 existing warnings; all workflow YAML parsed successfully. cargo fmt --all --check still reports formatting differences in pre-existing unrelated source edits, so the new CI formatting gate will remain red until those edits are formatted. The legacy mujoco-rs-sys build script also attempts to rewrite its read-only registry source during a fresh Clippy build; default crates.io packaging is unaffected because MuJoCo is opt-in.

README now explicitly warns that the repository-local mujoco-rust patch is not propagated through crates.io; MuJoCo environments currently require a source checkout, while the default published feature set remains verifiable.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Adapted imported GitHub automation for OGym, consolidated tag publishing into one release workflow, added Rust API docs deployment and MSRV validation, improved crates.io manifest metadata/package contents, trimmed unused runtime dependencies, and documented/tested official MuJoCo 3.9.0 installation.
<!-- SECTION:FINAL_SUMMARY:END -->
