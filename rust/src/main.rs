mod lib;
#[path = "unit_tests.rs"] mod unit_tests;
use unit_tests::run;

fn main() 
{
    run(100);
    run_benchmark();
}

fn run_benchmark()
{
    const CAPACITY: usize = 100;
    const REPETITION: i64 = 1000000;

    let benchmark = lib::benchmark_initialize(CAPACITY, REPETITION);
    print!("Initialization benchmark: {} ns\n", benchmark);

    let benchmark = lib::benchmark_add(CAPACITY, REPETITION);
    print!("Add benchmark: {} ns\n", benchmark);

    let benchmark = lib::benchmark_remove(CAPACITY, REPETITION);
    print!("Remove benchmark: {} ns\n", benchmark);

    let benchmark = lib::benchmark_exists(CAPACITY, REPETITION);
    print!("Exists benchmark: {} ns\n", benchmark);

    let benchmark = lib::benchmark_clone(CAPACITY, REPETITION);
    print!("Clone benchmark: {} ns\n", benchmark);

    let benchmark = lib::benchmark_resize(CAPACITY, REPETITION);
    print!("Resize benchmark: {} ns\n", benchmark);

    println!("\nBenchmarks passed ! \n");
}