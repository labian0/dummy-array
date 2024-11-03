import matplotlib.pyplot as plt
import numpy as np
from wrapper import go_impl, rust_impl

benchmarks = ['Initialize', 'Add', 'Remove', 'Exists']
valuesGo = [go_impl.benchmark_initialize(),
            go_impl.benchmark_add(),
            go_impl.benchmark_remove(),
            go_impl.benchmark_exists()]
valuesRust = [rust_impl.benchmark_initialize(),
            rust_impl.benchmark_add(),
            rust_impl.benchmark_remove(),
            rust_impl.benchmark_exists()]

# Define the positions for the bars
y_pos = np.arange(len(benchmarks))

# Create a vertical bar chart
plt.bar(y_pos - 0.2, valuesGo, width=0.4, label='Go', color='blue')
plt.bar(y_pos + 0.2, valuesRust, width=0.4, label='Rust', color='orange')

# Add labels and title
plt.xticks(y_pos, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Comparaison des benchmarks de DummyArray en Go et en Rust')
plt.legend()

# Show the plot
plt.savefig("visualisation/graphs/plot.png")

