---
id: TASK-13
title: Adapt GitHub automation and Cargo package metadata
status: In Progress
assignee:
  - '@codex'
created_date: '2026-07-22 15:14'
labels:
  - maintenance
  - ci
  - release
dependencies: []
references:
  - 'https://doc.rust-lang.org/cargo/reference/manifest.html'
  - 'https://doc.rust-lang.org/cargo/reference/publishing.html'
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace imported GitHub repository configuration with project-specific automation and improve Cargo.toml metadata for a clean, documented crates.io package.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GitHub workflows and repository templates reference ogym and run commands appropriate for this repository
- [ ] #2 Cargo.toml contains complete, valid package metadata aligned with current Cargo and crates.io guidance
- [ ] #3 Cargo package verification and relevant repository checks pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit repository structure, imported .github files, Cargo manifests, and current git state. 2. Confirm package metadata requirements in official Cargo/crates.io documentation. 3. Update GitHub automation/templates and Cargo.toml without disturbing unrelated work. 4. Run formatting, tests, workflow syntax/static checks, and cargo package verification. 5. Record results and finalize the task.
<!-- SECTION:PLAN:END -->
