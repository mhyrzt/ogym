---
id: TASK-13
title: Migrate README content into mdBook
status: Done
assignee:
  - '@codex'
created_date: '2026-07-22 15:43'
updated_date: '2026-07-22 15:44'
labels:
  - documentation
dependencies: []
modified_files:
  - docs/src/SUMMARY.md
  - docs/src/introduction.md
  - docs/src/getting-started.md
  - docs/src/architecture.md
  - docs/src/environments.md
  - docs/src/contributing.md
  - docs/src/roadmap.md
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Move the project guidance currently concentrated in README.md into the mdBook chapter structure while keeping README.md usable as the crate and repository landing page.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README overview and feature content is represented in the mdBook introduction
- [x] #2 Installation, MuJoCo setup, and quick-start guidance is represented in Getting Started
- [x] #3 Architecture, environments, contributing, roadmap, and license guidance is organized into appropriate book chapters
- [x] #4 The mdBook builds successfully without broken local links
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Split README sections across the existing mdBook chapters and add focused chapters where needed. 2. Adjust repository-relative links for the generated site. 3. Build and validate the book, then retain README.md as the crate/repository landing page.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Migrated the README's substantive guidance into dedicated mdBook chapters. Kept README.md unchanged because Cargo uses it as the published crate landing page. Validation passed: mdbook build, actionlint, and git diff --check.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Organized README content across Introduction, Getting Started, Architecture, Environments, Contributing, and Roadmap chapters; verified the mdBook and docs workflow build cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
