---
id: TASK-11
title: Consolidate and redesign benchmarks
status: Done
assignee: []
created_date: '2026-07-22 14:46'
updated_date: '2026-07-22 16:15'
labels: []
dependencies: []
modified_files:
  - benchmark/gym_benchmarks.py
  - benchmark/Justfile
  - benchmark/README.md
  - benchmark/visualize_results.py
  - benchmark/visualize.py
  - benchmark/pyproject.toml
  - benchmark/uv.lock
  - benchmark/results/benchmark.png
  - .gitignore
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the per-environment Gymnasium benchmark scripts with one argument-driven Python CLI and redesign the benchmark visualization as a clean 2x2 comparison figure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A single Python CLI runs any supported Gymnasium environment benchmark selected by arguments
- [x] #2 Existing benchmark recipes and documentation use the consolidated CLI
- [x] #3 Visualization uses a SciencePlots theme and produces the benchmark image successfully
- [x] #4 Visualization renders the four environment-family panels, plots every available complete ogym/Gymnasium pair, and skips missing or incomplete environments without failing
- [x] #5 Visualization uses milliseconds, labels every bar, removes top/right ticks, adds subplot spacing, and reports geometric-mean group speedup in each populated title
- [x] #6 Visualization uses grouped horizontal bars with readable environment and mean plus-or-minus standard-deviation labels, including the full MuJoCo family
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Replace grouped vertical bars with grouped horizontal bars and horizontal error bars. 2. Move environment names to the y-axis and values to bar ends. 3. Increase figure height and tune subplot spacing for the full MuJoCo group. 4. Run just plot and visually inspect current and complete synthetic layouts.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Consolidated all Gymnasium cases into benchmark/gym_benchmarks.py and removed the obsolete gym_benchmarks and utils directories. The report intentionally focuses its 2x2 layout on the four complete classic-control pairs in the stored Hyperfine data. Validation passed for CLI help, a five-step CartPole run, the Just all-recipe dry run, Python compilation, visualization generation, image inspection, and git diff whitespace checks.

User clarified that subplot grouping must be by environment family, not one classic-control environment per panel.

Reworked the plot after clarification: panels now represent Box2D/Rapier2D, MuJoCo, Classic Control, and Toy Text, with paired bars for every configured environment. Renamed the script to visualize.py and ignored benchmark/results. A complete synthetic 21-environment dataset rendered successfully and was visually inspected; the current stored all_results.json correctly fails validation because it is incomplete. Just plot dry-run, ignore-rule verification, and git diff --check passed.

Adjusted partial-result handling at user request. visualize.py now skips incomplete pairs with a stderr warning, preserves all four family panels, and displays an empty-state message for families without complete pairs.  succeeded against the current 15-entry all_results.json; the generated image was visually inspected and git diff --check passed.

Validation command just plot succeeded against the current 15-entry all_results.json.

Refined the chart presentation: converted means and standard deviations to milliseconds, added compact values above every error bar, disabled top/right major and minor ticks from the SciencePlots style, increased horizontal and vertical subplot spacing, and added geometric-mean speedup titles. Validation command just plot passed against current results; the generated image was visually inspected and git diff --check passed.

Renamed the generated plot output to benchmark/results/benchmark.png and updated the benchmark README. Validation command just plot generated the new filename successfully.

Enhanced bar annotations to show larger mean plus-or-minus standard deviation values in milliseconds. Normal panels use two-line horizontal labels; dense panels rotate compact labels to prevent overlap. Validation command just plot passed and the generated benchmark.png was visually inspected.

Converted all panels to grouped horizontal bars with horizontal standard-deviation error bars. Environment names now use the y-axis and mean plus-or-minus standard-deviation labels sit beyond bar ends. Increased figure height and inter-panel spacing. Validation command just plot passed on current partial results; a complete synthetic 21-environment render confirmed the full 11-row MuJoCo panel remains readable. Both images were visually inspected and git diff --check passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered a readable 2x2 horizontal family benchmark chart with partial-result support, millisecond mean plus-or-minus standard-deviation annotations, geometric-mean speedup titles, clean axes, and layouts verified for both current and full datasets.
<!-- SECTION:FINAL_SUMMARY:END -->
