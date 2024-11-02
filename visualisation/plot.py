import matplotlib.pyplot as plt
import numpy as np
import wrapper

CAPACITY = 1000000
REPETITIONS = 10

rust_impl:wrapper.Library = wrapper.Library("dummyarray_rust.so")
go_impl = wrapper.Library("./dummyarray_go.so")

# Sample data
benchmarks = ['Initialize', 'Add', 'Remove', 'Exists']
valuesGo = [go_impl.benchmark_initialize(CAPACITY,REPETITIONS),
            go_impl.benchmark_add(CAPACITY,REPETITIONS),
            go_impl.benchmark_remove(CAPACITY,REPETITIONS),
            go_impl.benchmark_exists(CAPACITY,REPETITIONS)]  # Values for the first benchmark
valuesRust = [rust_impl.benchmark_initialize(CAPACITY,REPETITIONS),
            rust_impl.benchmark_add(CAPACITY,REPETITIONS),
            rust_impl.benchmark_remove(CAPACITY,REPETITIONS),
            rust_impl.benchmark_exists(CAPACITY,REPETITIONS)]  # Values for the second benchmark

# Define the positions for the bars
y_pos = np.arange(len(benchmarks))

# Create a horizontal bar chart
plt.bar(y_pos - 0.2, valuesGo, width=0.4, label='Go', color='blue')
plt.bar(y_pos + 0.2, valuesRust, width=0.4, label='Rust', color='orange')

# Add labels and title
plt.xticks(y_pos, benchmarks)
plt.ylabel('Temps (secondes)')
plt.title('Comparaison des benchmarks de DummyArray en Go et en Rust')
plt.legend()

# Show the plot
plt.savefig("plot.png")

