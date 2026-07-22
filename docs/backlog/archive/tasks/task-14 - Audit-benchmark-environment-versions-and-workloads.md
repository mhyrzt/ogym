---
id: TASK-14
title: Audit benchmark environment versions and workloads
status: In Progress
assignee:
  - '@codex'
created_date: '2026-07-22 16:16'
labels:
  - benchmark
dependencies: []
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ensure every Python Gymnasium benchmark uses the latest supported environment version and that Python and Rust benchmarks execute equivalent, sufficiently large workloads. Reorganize benchmark dispatch definitions by environment family for maintainability.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every Gymnasium benchmark ID is verified against current official Gymnasium environment documentation and updated where necessary
- [ ] #2 Python and Rust benchmarks use matching step counts for every environment
- [ ] #3 Default workloads are large enough to reduce process-startup noise while remaining practical for the complete suite
- [ ] #4 Python and Rust benchmark definitions are organized clearly by environment family
- [ ] #5 Relevant benchmark CLI, build, and smoke checks pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory environment IDs and step defaults in Python, Rust, and Just recipes. 2. Verify current Gymnasium IDs against official documentation. 3. Choose per-family or per-environment workloads based on environment cost and align both implementations. 4. Reorganize definitions by family and update benchmark documentation. 5. Run parity, CLI, build, and smoke validations.
<!-- SECTION:PLAN:END -->
