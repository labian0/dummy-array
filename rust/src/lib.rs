#[path = "benchmark.rs"] mod benchmark;
use std::{ffi::c_ulonglong, time::Duration};
use benchmark::{Benchmark, DummyArrayVecBenchmark};

#[no_mangle]
/// Starts a benchmark focused on dummy-array initialisation, and return 
/// the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C" fn benchmark_initialize(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_initialize();
    benchmark.average_time.as_nanos() as c_ulonglong
}

#[no_mangle]
/// Starts a benchmark focused on dummy-array value adding, and return 
/// the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C" fn benchmark_add(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_add();
    benchmark.average_time.as_nanos() as c_ulonglong
}

#[no_mangle]
/// Starts a benchmark focused on dummy-array value removing, and return 
/// the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C"  fn benchmark_remove(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_remove();
    benchmark.average_time.as_nanos() as c_ulonglong
}

#[no_mangle]
/// Starts a benchmark focused on dummy-array value searching, and return 
/// the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C"  fn benchmark_exists(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_exists();
    benchmark.average_time.as_nanos() as c_ulonglong
}

#[no_mangle]
/// Starts a benchmark focused on dummy-array cloning, and return 
/// the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C"  fn benchmark_clone(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_clone();
    benchmark.average_time.as_nanos() as c_ulonglong
}

#[no_mangle]
/// Starts a benchmark focused on dummy-array resizing (when trying to add an 
/// element over the length), and return the average duration in nanoseconds.
/// #Params
/// - capacity: usize -> dummy-array size.
/// - repetition: i64 -> times the operation is processed (precision of the output).
pub extern "C"  fn benchmark_resize(capacity: usize, repetition: i64) -> c_ulonglong
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_resize();
    benchmark.average_time.as_nanos() as c_ulonglong
}