import matplotlib.pyplot as plt
import numpy as np
from wrapper import go_impl, rust_impl

# Compare initialize

capacities = [100, 500, 1000, 10000,11000,15000, 20000, 30000, 50000]
x = np.linspace(10000, 100000, 10000)

benchmarks = [str(c) for c in capacities]
valuesGo = [go_impl.benchmark_initialize(capacity=c) for c in capacities]
valuesRust = [rust_impl.benchmark_initialize(capacity=c) for c in capacities]

# Create a vertical bar chart
plt.plot(x, valuesGo, label='Go', color='blue')
plt.plot(x, valuesRust, label='Rust', color='orange')

# Add labels and title
plt.xticks(capacities, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Temps (ms) d\'initialisation d\'un DummyArray de capacité n')
plt.legend()

# Show the plot
plt.savefig("visualisation/graphs/tc_initialize.png")

plt.clf()

# Compare add

valuesGo = [go_impl.benchmark_add(capacity=c) for c in capacities]
valuesRust = [rust_impl.benchmark_add(capacity=c) for c in capacities]
plt.plot(capacities, valuesGo, label='Go', color='blue')
plt.plot(capacities, valuesRust, label='Rust', color='orange')

plt.xticks(capacities, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Temps (ms) d\'ajout de 1 valeur pour un DummyArray de capacité n')
plt.legend()

# Show the plot
plt.savefig("visualisation/graphs/tc_add.png")

plt.clf()

# Compare remove

valuesGo = [go_impl.benchmark_remove(capacity=c) for c in capacities]
valuesRust = [rust_impl.benchmark_remove(capacity=c) for c in capacities]
plt.plot(capacities, valuesGo, label='Go', color='blue')
plt.plot(capacities, valuesRust, label='Rust', color='orange')

plt.xticks(capacities, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Temps (ms) de suppression de 1 valeur d\'un DummyArray de capacité n')
plt.legend()

# Show the plot
plt.savefig("visualisation/graphs/tc_remove.png")

plt.clf()

# Compare exists

valuesGo = [go_impl.benchmark_add(capacity=c) for c in capacities]
valuesRust = [rust_impl.benchmark_add(capacity=c) for c in capacities]
plt.plot(capacities, valuesGo, label='Go', color='blue')
plt.plot(capacities, valuesRust, label='Rust', color='orange')

plt.xticks(capacities, benchmarks)
plt.ylabel('Temps (ms)')
plt.title('Temps (ms) de vérification d\'existence de 1 valeur d\'un DummyArray de capacité n')
plt.legend()

# Show the plot
plt.savefig("visualisation/graphs/tc_exists.png")

plt.clf()

# Compare all

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

