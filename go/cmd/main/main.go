package main

import (
	"C"

	"github.com/labian0/dummy-array/go/internal/bench"
)

func main() {}

//export benchmark_initialize
func benchmark_initialize(capacity, repetitions int) C.double {
	return C.double(float64(bench.BenchmarkInitialize(uint(capacity), uint(repetitions))) / 1000000)
}

//export benchmark_add
func benchmark_add(capacity, repetitions int) C.double {
	return C.double(float64(bench.BenchmarkAdd(uint(capacity), uint(repetitions))) / 1000000)
}

//export benchmark_remove
func benchmark_remove(capacity, repetitions int) C.double {
	return C.double(float64(bench.BenchmarkRemove(uint(capacity), uint(repetitions))) / 1000000)
}

//export benchmark_exists
func benchmark_exists(capacity, repetitions int) C.double {
	return C.double(float64(bench.BenchmarkExists(uint(capacity), uint(repetitions))) / 1000000)
}
