import matplotlib.pyplot as plt
import numpy as np
from wrapper import go_impl, rust_impl

capacities = [100, 500, 1000, 10000,11000,15000, 20000, 30000, 50000]

benchmarks = [str(c) for c in capacities]
valuesGo = [go_impl.benchmark_initialize(capacity=c) for c in capacities]
valuesRust = [rust_impl.benchmark_initialize(capacity=c) for c in capacities]

# Create a vertical bar chart
plt.plot(capacities, valuesGo, label='Go', color='blue')
plt.plot(capacities, valuesRust, label='Rust', color='orange')

# Add labels and title
plt.xticks(capacities, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Temps (ms) d\'initialisation d\'un DummyArray de capacité n')
plt.legend()

# Show the plot
plt.savefig("tc_initialize.png")
