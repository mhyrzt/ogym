import json
import matplotlib.pyplot as plt
import numpy as np
import os
from pathlib import Path

# Configuration
RESULTS_FILE = "results/all_results.json"
OUTPUT_IMAGE = "results/benchmark_comparison.png"

def load_data(filepath):
    if not os.path.exists(filepath):
        print(f"Error: Results file '{filepath}' not found.")
        print("Run 'just hyperfine-compare-all' first.")
        exit(1)
    
    with open(filepath, 'r') as f:
        return json.load(f)

def parse_results(data):
    """
    Parses the Hyperfine JSON structure into a dictionary organized by environment.
    Expected command names: "ogym-acrobot", "gym-acrobot", etc.
    """
    envs = {}
    
    for result in data['results']:
        command_name = result['command']
        mean_time = result['mean']
        stddev = result['stddev']
        
        # Extract env name and type (ogym vs gym)
        # Assumes format: "ogym-{env}" or "gym-{env}"
        parts = command_name.split('-')
        if len(parts) < 2:
            continue
            
        tool = parts[0] # 'ogym' or 'gym'
        env_name = "-".join(parts[1:]).title() # 'Acrobot', 'Mountain Car', etc.
        
        if env_name not in envs:
            envs[env_name] = {'ogym': None, 'gym': None}
        
        envs[env_name][tool] = {
            'mean': mean_time,
            'stddev': stddev
        }
        
    return envs

def plot_benchmark(envs):
    # Filter out incomplete benchmarks
    valid_envs = {k: v for k, v in envs.items() if v['ogym'] and v['gym']}
    
    if not valid_envs:
        print("No complete pairs (ogym vs gym) found to plot.")
        return

    labels = list(valid_envs.keys())
    ogym_times = [valid_envs[env]['ogym']['mean'] for env in labels]
    gym_times = [valid_envs[env]['gym']['mean'] for env in labels]
    
    # Calculate speedups
    speedups = [g / o for g, o in zip(gym_times, ogym_times)]

    x = np.arange(len(labels))  # the label locations
    width = 0.35  # the width of the bars

    fig, ax = plt.subplots(figsize=(10, 6))
    rects1 = ax.bar(x - width/2, ogym_times, width, label='Rust (ogym)', color='#dea584')
    rects2 = ax.bar(x + width/2, gym_times, width, label='Python (gymnasium)', color='#6699cc')

    # Add some text for labels, title and custom x-axis tick labels, etc.
    ax.set_ylabel('Execution Time (seconds)')
    ax.set_title('Performance Benchmark: Rust (ogym) vs Python (gymnasium)\n(Lower is better)')
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.legend()
    
    # Add grid
    ax.yaxis.grid(True, linestyle='--', alpha=0.7)

    # Annotate bars with speedup info
    def autolabel_speedup(rects_gym, speedups):
        """Attach a text label above the Python bar displaying the speedup."""
        for rect, speedup in zip(rects_gym, speedups):
            height = rect.get_height()
            ax.annotate(f'{speedup:.1f}x slower',
                        xy=(rect.get_x() + rect.get_width() / 2, height),
                        xytext=(0, 3),  # 3 points vertical offset
                        textcoords="offset points",
                        ha='center', va='bottom', fontweight='bold', color='#333333')

    autolabel_speedup(rects2, speedups)

    # Handle very small times for Rust vs large times for Python (Log scale optional)
    # If the difference is massive, uncomment the line below:
    # ax.set_yscale('log') 

    fig.tight_layout()

    # Ensure results directory exists
    Path(OUTPUT_IMAGE).parent.mkdir(parents=True, exist_ok=True)
    
    plt.savefig(OUTPUT_IMAGE, dpi=300)
    print(f"✅ Plot saved to {OUTPUT_IMAGE}")

def main():
    print("📊 Generating benchmark visualization...")
    data = load_data(RESULTS_FILE)
    parsed_envs = parse_results(data)
    plot_benchmark(parsed_envs)

if __name__ == "__main__":
    main()
