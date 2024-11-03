import ctypes

#default capacity and repetitions, can be overwritten in benchmark function calls
CAPACITY = 1000000
REPETITIONS = 10

class Library:
    def __init__(self, library_name:str):
        self.lib = ctypes.cdll.LoadLibrary(library_name)
        try:
            self.lib.benchmark_initialize.restype = ctypes.c_double
            self.lib.benchmark_add.restype = ctypes.c_double
            self.lib.benchmark_remove.restype = ctypes.c_double
            self.lib.benchmark_exists.restype = ctypes.c_double
        except AttributeError as e:
            raise Exception("Needed library functions weren't properly exported/don't exist. Double-check the spelling of exported function signatures")
            
    def benchmark_initialize(self, capacity:int=CAPACITY, repetitions:int=REPETITIONS) -> float:
        """get the average execution time (in milliseconds) for ONE DummyArray initialization of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_initialize(capacity,repetitions)

    def benchmark_add(self, capacity:int=CAPACITY, repetitions:int=REPETITIONS) -> float:
        """get the average execution time (in milliseconds) for adding <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_add(capacity,repetitions)

    def benchmark_remove(self, capacity:int=CAPACITY, repetitions:int=REPETITIONS) -> float:
        """get the average execution time (in milliseconds) for removing <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_remove(capacity,repetitions)

    def benchmark_exists(self, capacity:int=CAPACITY, repetitions:int=REPETITIONS) -> float:
        """get the average execution time (in milliseconds) for checking the existence of <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_exists(capacity,repetitions)

go_impl = Library("./dummyarray_go.so")
rust_impl = Library("./dummyarray_rust.so")


def main():
    #mesurer benchmarks ici

    print(f"temps (ms) d'ajout de valeurs pour un dummy array de capacité 100 000: {go_impl.benchmark_add(capacity=100000, repetitions=REPETITIONS)} (go), {rust_impl.benchmark_add(capacity=100000, repetitions=REPETITIONS)} (rust)")

if __name__ == "__main__":
    main()