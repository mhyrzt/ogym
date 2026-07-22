"""Render benchmark comparisons grouped by environment family."""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import scienceplots  # noqa: F401  Registers the SciencePlots styles.


BENCHMARK_DIR = Path(__file__).resolve().parent
RESULTS_FILE = BENCHMARK_DIR / "results" / "all_results.json"
OUTPUT_IMAGE = BENCHMARK_DIR / "results" / "benchmark.png"
IMPLEMENTATIONS = ("ogym", "gym")
COLORS = {"ogym": "#D97757", "gym": "#4C78A8"}
LABELS = {"ogym": "ogym (Rust)", "gym": "Gymnasium (Python)"}
SECONDS_TO_MILLISECONDS = 1_000
ENVIRONMENT_GROUPS = (
    (
        "Box2D / Rapier2D",
        (
            ("bipedal-walker", "BipedalWalker"),
            ("lunar-lander", "LunarLander"),
        ),
    ),
    (
        "MuJoCo",
        (
            ("ant", "Ant"),
            ("half-cheetah", "HalfCheetah"),
            ("hopper", "Hopper"),
            ("humanoid", "Humanoid"),
            ("humanoid-standup", "HumanoidStandup"),
            ("inverted-double-pendulum", "InvDoublePendulum"),
            ("inverted-pendulum", "InvPendulum"),
            ("pusher", "Pusher"),
            ("reacher", "Reacher"),
            ("swimmer", "Swimmer"),
            ("walker2d", "Walker2d"),
        ),
    ),
    (
        "Classic Control",
        (
            ("acrobot", "Acrobot"),
            ("cartpole", "CartPole"),
            ("mountain-car", "MountainCar"),
            ("pendulum", "Pendulum"),
        ),
    ),
    (
        "Toy Text",
        (
            ("blackjack", "Blackjack"),
            ("cliff-walking", "CliffWalking"),
            ("frozen-lake", "FrozenLake"),
            ("taxi", "Taxi"),
        ),
    ),
)


def load_data(filepath: Path) -> dict:
    try:
        with filepath.open(encoding="utf-8") as results_file:
            return json.load(results_file)
    except FileNotFoundError as error:
        raise SystemExit(
            f"Results file not found: {filepath}\nRun `just all` first."
        ) from error


def parse_results(data: dict) -> dict[str, dict[str, dict[str, float]]]:
    """Group Hyperfine results by environment slug and implementation."""
    environments: dict[str, dict[str, dict[str, float]]] = {}
    for result in data["results"]:
        implementation, separator, environment = result["command"].partition("-")
        if not separator or implementation not in IMPLEMENTATIONS:
            continue

        environments.setdefault(environment, {})[implementation] = {
            "mean": result["mean"],
            "stddev": result["stddev"],
        }
    return environments


def available_groups(
    environments: dict[str, dict[str, dict[str, float]]],
) -> tuple[tuple[str, tuple[tuple[str, str], ...]], ...]:
    """Select complete benchmark pairs and report skipped environments."""
    groups = []
    skipped = []
    for group_name, group_environments in ENVIRONMENT_GROUPS:
        available_environments = []
        for slug, label in group_environments:
            available = environments.get(slug, {})
            absent = [
                LABELS[implementation]
                for implementation in IMPLEMENTATIONS
                if implementation not in available
            ]
            if absent:
                skipped.append(f"{group_name}/{label} ({', '.join(absent)})")
            else:
                available_environments.append((slug, label))
        groups.append((group_name, tuple(available_environments)))

    if skipped:
        details = "\n  - ".join(skipped)
        print(f"Skipping incomplete benchmark pairs:\n  - {details}", file=sys.stderr)

    if not any(group_environments for _, group_environments in groups):
        raise SystemExit(
            "No complete benchmark pairs found. Run `just all` to generate results."
        )

    return tuple(groups)


def format_milliseconds(value: float) -> str:
    """Format a compact bar label while retaining useful precision."""
    if value >= 100:
        return f"{value:.0f}"
    if value >= 10:
        return f"{value:.1f}"
    return f"{value:.2f}"


def group_title(
    group_name: str,
    group_environments: tuple[tuple[str, str], ...],
    environments: dict[str, dict[str, dict[str, float]]],
) -> str:
    """Describe the geometric-mean speedup for an environment family."""
    speedups = [
        environments[slug]["gym"]["mean"] / environments[slug]["ogym"]["mean"]
        for slug, _ in group_environments
    ]
    average_speedup = math.prod(speedups) ** (1 / len(speedups))

    if average_speedup >= 1:
        comparison = f"ogym {average_speedup:.1f}× faster on average"
    else:
        comparison = f"Gymnasium {1 / average_speedup:.1f}× faster on average"
    return f"{group_name} — {comparison}"


def plot_benchmark(
    environments: dict[str, dict[str, dict[str, float]]],
    output_image: Path = OUTPUT_IMAGE,
) -> None:
    groups = available_groups(environments)

    with plt.style.context(["science", "no-latex"]):
        fig, axes = plt.subplots(2, 2, figsize=(17, 12.5))
        fig.subplots_adjust(
            left=0.09,
            right=0.985,
            bottom=0.08,
            top=0.90,
            wspace=0.28,
            hspace=0.26,
        )
        fig.suptitle(
            "ogym vs Gymnasium",
            x=0.06,
            y=0.97,
            ha="left",
            fontsize=17,
            fontweight="semibold",
        )
        fig.supxlabel("Execution time (milliseconds, lower is better)")
        fig.supylabel("Environment")

        legend_handles = []
        bar_height = 0.38

        for ax, (group_name, group_environments) in zip(axes.flat, groups):
            positions = list(range(len(group_environments)))

            if not group_environments:
                ax.text(
                    0.5,
                    0.5,
                    "No complete benchmark pairs",
                    transform=ax.transAxes,
                    ha="center",
                    va="center",
                    color="0.45",
                )
                ax.set_title(group_name, fontweight="semibold", pad=9)
                ax.set_xticks([])
                ax.set_yticks([])
                ax.tick_params(which="both", top=False, right=False)
                ax.spines[["top", "right", "bottom", "left"]].set_visible(False)
                continue

            maximum_value = 0.0
            for implementation_index, implementation in enumerate(IMPLEMENTATIONS):
                offset = (implementation_index - 0.5) * bar_height
                means = [
                    environments[slug][implementation]["mean"]
                    * SECONDS_TO_MILLISECONDS
                    for slug, _ in group_environments
                ]
                errors = [
                    environments[slug][implementation]["stddev"]
                    * SECONDS_TO_MILLISECONDS
                    for slug, _ in group_environments
                ]
                bar_positions = [position + offset for position in positions]
                bars = ax.barh(
                    bar_positions,
                    means,
                    xerr=errors,
                    height=bar_height,
                    label=LABELS[implementation],
                    color=COLORS[implementation],
                    capsize=2.5,
                    edgecolor="white",
                    linewidth=0.6,
                )
                if len(legend_handles) < len(IMPLEMENTATIONS):
                    legend_handles.append(bars)

                for position, mean, error in zip(bar_positions, means, errors):
                    value_label = (
                        f"{format_milliseconds(mean)} ± "
                        f"{format_milliseconds(error)}"
                    )
                    ax.annotate(
                        value_label,
                        (mean + error, position),
                        xytext=(5, 0),
                        textcoords="offset points",
                        ha="left",
                        va="center",
                        fontsize=8 if len(group_environments) > 5 else 9,
                    )
                maximum_value = max(
                    maximum_value,
                    max(mean + error for mean, error in zip(means, errors)),
                )

            ax.set_title(
                group_title(group_name, group_environments, environments),
                fontweight="semibold",
                pad=12,
            )
            ax.set_yticks(
                positions,
                [label for _, label in group_environments],
            )
            ax.invert_yaxis()
            ax.set_xlim(0, maximum_value * 1.32)
            ax.margins(y=0.04)
            ax.grid(False)
            ax.tick_params(which="both", top=False, right=False)
            ax.xaxis.set_ticks_position("bottom")
            ax.yaxis.set_ticks_position("left")
            ax.spines[["top", "right"]].set_visible(False)

        fig.legend(
            legend_handles,
            [LABELS[implementation] for implementation in IMPLEMENTATIONS],
            loc="upper right",
            ncols=2,
            frameon=False,
            bbox_to_anchor=(0.98, 1.0),
        )

        output_image.parent.mkdir(parents=True, exist_ok=True)
        fig.savefig(output_image, dpi=300, bbox_inches="tight")
        plt.close(fig)

    print(f"Plot saved to {output_image}")


def main() -> None:
    plot_benchmark(parse_results(load_data(RESULTS_FILE)))


if __name__ == "__main__":
    main()
