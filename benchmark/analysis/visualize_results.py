import json
import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd
import numpy as np
import os
from pathlib import Path


def load_json_results(file_path):
    """Load JSON results from hyperfine output."""
    if not os.path.exists(file_path):
        print(f"Warning: {file_path} does not exist.")
        return None

    with open(file_path, "r") as f:
        return json.load(f)


def extract_timing_data(json_data, library_name):
    """Extract timing data from hyperfine JSON results."""
    if json_data is None:
        return []

    results = []
    for cmd_data in json_data.get("results", []):
        command = cmd_data["command"]
        mean_time = cmd_data["mean"]
        median_time = cmd_data["median"]
        std_dev = cmd_data["stddev"]
        env_name = command.split("-")[1].replace('"', "")  # Extract environment name

        results.append(
            {
                "environment": env_name.title(),
                "library": library_name,
                "mean_time": mean_time,
                "median_time": median_time,
                "std_dev": std_dev,
                "command": command,
            }
        )

    return results


def plot_comparison(results_df):
    """Create a bar plot comparing ogym vs gymnasium performance."""
    plt.figure(figsize=(12, 8))

    # Set style
    sns.set_style("whitegrid")

    # Create the plot
    ax = sns.barplot(
        data=results_df,
        x="environment",
        y="mean_time",
        hue="library",
        palette=["#1f77b4", "#ff7f0e"],  # Blue for ogym, Orange for gym
    )

    # Customize the plot
    plt.title(
        "Environment Benchmark: ogym vs gymnasium", fontsize=16, fontweight="bold"
    )
    plt.xlabel("Environment", fontsize=12)
    plt.ylabel("Mean Execution Time (seconds)", fontsize=12)

    # Add value labels on bars
    for container in ax.containers:
        ax.bar_label(container, fmt="%.3f", fontsize=10)

    # Rotate x-axis labels for better readability
    plt.xticks(rotation=45, ha="right")

    # Adjust layout to prevent label cutoff
    plt.tight_layout()

    # Save the plot
    plt.savefig("results/benchmark_comparison.png", dpi=300, bbox_inches="tight")
    plt.show()


def plot_speedup_analysis(results_df):
    """Create a speedup analysis plot."""
    # Pivot the data to have ogym and gym times in separate columns
    pivot_df = results_df.pivot(
        index="environment", columns="library", values="mean_time"
    )
    pivot_df["speedup"] = pivot_df["ogym"] / pivot_df["gymnasium"]

    plt.figure(figsize=(10, 6))

    # Create horizontal bar plot for speedup
    ax = sns.barplot(
        x="speedup", y=pivot_df.index, data=pivot_df.reset_index(), palette="viridis"
    )

    # Add a vertical line at 1.0 (indicating equal performance)
    ax.axvline(x=1.0, color="red", linestyle="--", label="Equal Performance")

    # Customize the plot
    plt.title(
        "Speedup Analysis: ogym vs gymnasium (ogym/gym ratio)",
        fontsize=14,
        fontweight="bold",
    )
    plt.xlabel("Speedup Ratio (ogym / gymnasium)", fontsize=12)
    plt.ylabel("Environment", fontsize=12)

    # Add value labels on bars
    for i, v in enumerate(pivot_df["speedup"]):
        ax.text(
            v + 0.01 if v >= 1 else v - 0.05,
            i,
            f"{v:.2f}x",
            va="center",
            ha="left" if v >= 1 else "right",
            fontsize=10,
            fontweight="bold",
            color="black" if 0.8 < v < 1.2 else "white",
        )

    plt.tight_layout()
    plt.savefig("results/speedup_analysis.png", dpi=300, bbox_inches="tight")
    plt.show()

    return pivot_df


def plot_combined_results():
    """Load and plot all results combined."""
    # Load JSON results
    ogym_results = load_json_results("results/ogym_results.json")
    gym_results = load_json_results("results/gym_results.json")

    # Extract data
    ogym_data = extract_timing_data(ogym_results, "ogym")
    gym_data = extract_timing_data(gym_results, "gymnasium")

    # Combine into a single DataFrame
    all_data = ogym_data + gym_data
    df = pd.DataFrame(all_data)

    # Create output directory if it doesn't exist
    os.makedirs("results", exist_ok=True)

    # Plot comparisons
    plot_comparison(df)
    speedup_df = plot_speedup_analysis(df)

    # Print summary statistics
    print("\n=== Performance Summary ===")
    for env in df["environment"].unique():
        env_data = df[df["environment"] == env]
        ogym_time = env_data[env_data["library"] == "ogym"]["mean_time"].iloc[0]
        gym_time = env_data[env_data["library"] == "gymnasium"]["mean_time"].iloc[0]
        speedup = ogym_time / gym_time
        print(
            f"{env}: ogym={ogym_time:.4f}s, gym={gym_time:.4f}s, speedup={speedup:.2f}x"
        )

    # Save the combined dataframe as CSV
    df.to_csv("results/benchmark_results.csv", index=False)

    return df


def plot_individual_environment(env_name):
    """Plot results for a specific environment."""
    comparison_files = [
        "results/acrobot_comparison.json",
        "results/cartpole_comparison.json",
        "results/mountain_car_comparison.json",
        "results/pendulum_comparison.json",
    ]

    for file_path in comparison_files:
        if not os.path.exists(file_path):
            continue

        json_data = load_json_results(file_path)
        if json_data is None:
            continue

        # Get environment name from file
        env = file_path.split("/")[-1].replace("_comparison.json", "").replace("_", "-")
        env_title = env.replace("-", " ").title()

        if env_title == env_name.title():
            results = extract_timing_data(json_data, "combined")

            if results:
                df = pd.DataFrame(results)

                plt.figure(figsize=(8, 6))
                ax = sns.barplot(
                    data=df, x="library", y="mean_time", palette=["#1f77b4", "#ff7f0e"]
                )

                plt.title(
                    f"{env_title} Benchmark Comparison", fontsize=14, fontweight="bold"
                )
                plt.ylabel("Mean Execution Time (seconds)", fontsize=12)
                plt.xlabel("Library", fontsize=12)

                # Add value labels on bars
                for container in ax.containers:
                    ax.bar_label(container, fmt="%.3f", fontsize=10)

                plt.tight_layout()
                plt.savefig(f"results/{env}_detailed.png", dpi=300, bbox_inches="tight")
                plt.show()

                return df


def main():
    """Main function to generate all plots."""
    print("Generating benchmark visualization...")

    # Create results directory
    os.makedirs("results", exist_ok=True)

    # Generate combined results plot
    combined_df = plot_combined_results()

    print("\nVisualizations saved to the 'results' directory:")
    print("- benchmark_comparison.png: Overall comparison bar chart")
    print("- speedup_analysis.png: Speedup ratio analysis")
    print("- benchmark_results.csv: Raw data in CSV format")

    # Generate individual environment plots if comparison files exist
    envs = ["acrobot", "cartpole", "mountain-car", "pendulum"]
    for env in envs:
        env_title = env.replace("-", " ").title()
        plot_individual_environment(env_title)


if __name__ == "__main__":
    main()
