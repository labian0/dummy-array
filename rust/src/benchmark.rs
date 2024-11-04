#[path = "dummy_array.rs"] mod dummy_arrays;
use std::time::{Duration, SystemTime};
use rand::Rng;
use dummy_arrays::{DummyArray, DummyArrayVec};

/// Benchmark structure for the dummy array. <br/>
/// It refers to the average time (ns) taken to perform a given 
/// operation on the dummy array. <br/>
/// The benchmark is performed on a given capacity and repetition. <br/>
pub struct DummyArrayVecBenchmark {
    pub capacity: usize,
    pub repetition: i64,
    pub average_time: Duration,
}

/// Expected behaviors for the dummy array benchmark. <br/>
pub trait Benchmark {
    fn benchmark_initialize(&mut self) -> ();
    fn benchmark_add(&mut self) -> ();
    fn benchmark_remove(&mut self) -> ();
    fn benchmark_exists(&mut self) -> ();
}

impl Benchmark for DummyArrayVecBenchmark {

    /// Calculates the average time taken to initialize the dummy array.
    fn benchmark_initialize(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut _dummy_array: DummyArrayVec;
        let mut start: SystemTime;

        for _ in 0..self.repetition
        {
            start = SystemTime::now();
            _dummy_array = DummyArrayVec::new(self.capacity).unwrap();
            total_time += start.elapsed().unwrap();
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to add a value to the dummy array.
    fn benchmark_add(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();
        let mut value: i64;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut warning: Result<bool, &str>;
        let mut start: SystemTime;

        for _ in 0..self.repetition
        {
            value = rng.gen_range(0..self.capacity as i64);
            start = SystemTime::now();
            warning = dummy_array.add(value);
            total_time += start.elapsed().unwrap();

            if warning.is_err() && warning.unwrap_err() == "The dummy array is full. "
            {
                total_time -= start.elapsed().unwrap();
                dummy_array = DummyArrayVec::new(self.capacity).unwrap();
            }
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to remove a value from the dummy array.
    fn benchmark_remove(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let value: i64 = rng.gen_range(0..self.capacity as i64) as i64;
        let mut start: SystemTime;
        
        self.populate(&mut dummy_array);

        for _ in 0..self.repetition
        {
            start = SystemTime::now();
            dummy_array.remove(value).unwrap_or(false);
            total_time += start.elapsed().unwrap();
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Calculates the average time taken to search for a value in the dummy array.
    fn benchmark_exists(&mut self) -> ()
    {
        let mut dummy_array = DummyArrayVec::new(self.capacity).unwrap();
        let mut total_time = Duration::new(0, 0);
        let mut value: i64;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut start: SystemTime;

        self.populate(&mut dummy_array);

        for _ in 0..self.repetition
        {
            value = rng.gen_range(0..i64::MAX);
            start = SystemTime::now();
            dummy_array.exists(value);
            total_time += start.elapsed().unwrap();
        }
        self.average_time = total_time / self.repetition as u32;
    }    
}

impl DummyArrayVecBenchmark {
    /// Calculates the average time taken to clone the dummy array.
    pub fn benchmark_clone(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut _dummy_array_clone: DummyArrayVec;
        let dummy_array: DummyArrayVec = DummyArrayVec::new(self.capacity).unwrap();
        let mut  start: SystemTime;

        for _ in 0..self.repetition
        {
            start = SystemTime::now();
            _dummy_array_clone = dummy_array.clone();
            total_time += start.elapsed().unwrap();
        }
        self.average_time = total_time / self.repetition as u32;
        
    }

    /// Calculates the average time taken to resize the dummy array.
    pub fn benchmark_resize(&mut self) -> ()
    {
        let mut total_time = Duration::new(0, 0);
        let mut dummy_array: DummyArrayVec = DummyArrayVec::new(0).unwrap();
        let mut  start: SystemTime;

        for x in 0..self.repetition
        {
            start = SystemTime::now();
            dummy_array.add(x).unwrap();
            total_time += start.elapsed().unwrap();  
        }
        self.average_time = total_time / self.repetition as u32;
    }

    /// Populates the dummy array with random values.
    pub fn populate(&mut self, dummy_array: &mut DummyArrayVec) -> ()
    {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut value: i64;

        for _ in 0..self.capacity
        {
            value = rng.gen_range(0..self.capacity as i64) as i64;
            while dummy_array.exists(value)
            {
                value = rng.gen_range(0..self.capacity as i64) as i64;
            }
            dummy_array.add(value).unwrap();
        }
    }
}
