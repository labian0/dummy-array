import ctypes

class Library:
    def __init__(self, library_name:str):
        self.lib = ctypes.cdll.LoadLibrary(library_name)
        try:
            self.lib.benchmark_initialize
            self.lib.benchmark_add
            self.lib.benchmark_remove
            self.lib.benchmark_exists
        except AttributeError as e:
            raise Exception("Needed library functions weren't properly exported/don't exist. Double-check the spelling of exported function signatures")
            
    def benchmark_initialize(self, capacity:int, repetitions:int) -> int:
        """get the average execution time (in nanoseconds) for ONE DummyArray initialization of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_initialize(capacity,repetitions)

    def benchmark_add(self, capacity:int, repetitions:int) -> int:
        """get the average execution time (in nanoseconds) for adding <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_add(capacity,repetitions)

    def benchmark_remove(self, capacity:int, repetitions:int) -> int:
        """get the average execution time (in nanoseconds) for removing <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_remove(capacity,repetitions)

    def benchmark_exists(self, capacity:int, repetitions:int) -> int:
        """get the average execution time (in nanoseconds) for checking the existence of <capacity> values in ONE DummyArray of size <capacity>
        averaged over <repetitions> repetitions"""
        return self.lib.benchmark_exists(capacity,repetitions)
