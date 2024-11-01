use std::time::{Duration, Instant};
use crate::dummy_array::DummyArrayVec;
use crate::dummy_array::DummyArray;

/// Benchmark structure for the dummy array. <br/>
/// It refers to the average time (ns) taken to perform a given 
/// operation on the dummy array. <br/>
/// The benchmark is performed on a given capacity and repetition. <br/>
struct DummyArrayVecBenchmark {
    capacity: usize,
    repetition: i64,
    average_time: Duration,
}

/// Expected behaviors for the dummy array benchmark. <br/>
trait Benchmark {
    fn benchmark_initialize(&mut self) -> ();
    fn benchmark_add(&mut self, value: i64) -> ();
    fn benchmark_remove(&mut self, value: i64) -> ();
    fn benchmark_exists(&mut self, value: i64) -> ();
}

impl Benchmark for DummyArrayVecBenchmark {

    /// Calculates the average time taken to initialize the dummy array.
    fn benchmark_initialize(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut _dummy_array: DummyArrayVec;

        for _ in 0..self.repetition
        {
            let start = Instant::now();
            _dummy_array = DummyArrayVec::new(self.capacity).unwrap();
            total_time += start.elapsed();
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to add a value to the dummy array.
    fn benchmark_add(&mut self, value: i64) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();

        for _ in 0..self.repetition
        {
            let warning: Result<bool, &str>;
            let start = Instant::now();
            warning = dummy_array.add(value);
            total_time += start.elapsed();

            if warning.is_err() && warning.unwrap_err() == "The dummy array is full. "
            {
                dummy_array = DummyArrayVec::new(self.capacity).unwrap();
            }
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to remove a value from the dummy array.
    fn benchmark_remove(&mut self, value: i64) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();

        for _ in 0..self.repetition
        {
            dummy_array.add(value).unwrap();
            let start = Instant::now();
            dummy_array.remove(value).unwrap();
            total_time += start.elapsed();
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to search for a value in the dummy array.
    fn benchmark_exists(&mut self, value: i64) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        for _ in 0..self.repetition
        {
            let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();
            dummy_array.add(value).unwrap();
            let start = Instant::now();
            dummy_array.exists(value);
            total_time += start.elapsed();
        }
        self.average_time = total_time / self.repetition as u32;
    }    
}

impl DummyArrayVecBenchmark {
    /// Calculates the average time taken to clone the dummy array.
    fn benchmark_clone(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut _dummy_array_clone: DummyArrayVec;
        let dummy_array: DummyArrayVec = DummyArrayVec::new(self.capacity).unwrap();

        for _ in 0..self.repetition
        {
            let start = Instant::now();
            _dummy_array_clone = dummy_array.clone();
            total_time += start.elapsed();
        }
        self.average_time = total_time / self.repetition as u32;
        
    }

    /// Calculates the average time taken to resize the dummy array.
    fn benchmark_resize(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array: DummyArrayVec = DummyArrayVec::new(0).unwrap();

        for x in 0..self.repetition
        {
            let start = Instant::now();
            dummy_array.add(x);
            total_time += start.elapsed();  
        }
        self.average_time = total_time / self.repetition as u32;
    }
}

pub fn benchmark_initialize(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_initialize();
    benchmark.average_time.as_nanos()
}

pub fn benchmark_add(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_add(5);
    benchmark.average_time.as_nanos()
}

pub fn benchmark_remove(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_remove(2);
    benchmark.average_time.as_nanos()
}

pub fn benchmark_exists(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_exists(5);
    benchmark.average_time.as_nanos()
}

pub fn benchmark_clone(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_clone();
    benchmark.average_time.as_nanos()
}

pub fn benchmark_resize(capacity: usize, repetition: i64) -> u128
{
    let mut benchmark = DummyArrayVecBenchmark {
        capacity,
        repetition,
        average_time: Duration::new(0, 0)
    };
    benchmark.benchmark_resize();
    benchmark.average_time.as_nanos()
}