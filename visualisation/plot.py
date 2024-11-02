import matplotlib.pyplot as plt
import numpy as np

# Sample data
benchmarks = ['Benchmark A', 'Benchmark B']
values1 = [75, 85]  # Values for the first benchmark
values2 = [65, 90]  # Values for the second benchmark

# Define the positions for the bars
y_pos = np.arange(len(benchmarks))

# Create a horizontal bar chart
plt.barh(y_pos - 0.2, values1, height=0.4, label='Benchmark 1', color='blue')
plt.barh(y_pos + 0.2, values2, height=0.4, label='Benchmark 2', color='orange')

# Add labels and title
plt.yticks(y_pos, benchmarks)
plt.xlabel('Score')
plt.title('Comparison of Two Benchmarks')
plt.legend()

# Show the plot
plt.savefig("plot.png")
