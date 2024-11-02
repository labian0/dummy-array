mod unit_tests;
mod dummy_array;
mod pylib;
mod benchmark;

fn main() 
{
    unit_tests::main(100);
    run_benchmark();
}

fn run_benchmark()
{
    const CAPACITY: usize = 100;
    const REPETITION: i64 = 1000000;

    let benchmark = pylib::benchmark_initialize(CAPACITY, REPETITION);
    print!("Initialization benchmark: {} ns\n", benchmark);

    let benchmark = pylib::benchmark_add(CAPACITY, REPETITION);
    print!("Add benchmark: {} ns\n", benchmark);

    let benchmark = pylib::benchmark_remove(CAPACITY, REPETITION);
    print!("Remove benchmark: {} ns\n", benchmark);

    let benchmark = pylib::benchmark_exists(CAPACITY, REPETITION);
    print!("Exists benchmark: {} ns\n", benchmark);

    let benchmark = pylib::benchmark_clone(CAPACITY, REPETITION);
    print!("Clone benchmark: {} ns\n", benchmark);

    let benchmark = pylib::benchmark_resize(CAPACITY, REPETITION);
    print!("Resize benchmark: {} ns\n", benchmark);

    println!("\nBenchmarks passed ! \n");
}