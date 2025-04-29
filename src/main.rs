mod task;
mod helpers;

use std::io;

fn prompt_for_u32(prompt: &str) -> u32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u32>().expect("Please enter a valid number")
}


struct Inputs<T> {
    // varaibles a and b used for compute, multiply and divide
    a: T,
    b: T,
}
// For Creating Struct and passing variables, use following format:
//   - let x = Pair{a: 10, b: 20};
//   - let result = compute(x.a, x.b);

struct Modulo {
    base: u64,
    exponent: u64, 
    modulus: u64,
}

fn main() {
    let batch_size = prompt_for_u32("Enter number of tasks to generate:");
    let thread_count = prompt_for_u32("Enter number of threads:");

    println!("Generating {} tasks...", batch_size);
    println!("Using {} threads for concurrent execution.", thread_count);

     // TODO: Generate tasks and store in a Vec<TaskType>
    // let tasks = generate_tasks(batch_size);

    // TODO: Execute tasks serially and log time
    // let serial_duration = execute_serially(&tasks);

    // TODO: Execute tasks concurrently using thread_count and log time
    // let concurrent_duration = execute_concurrently(&tasks, thread_count);

    // TODO: Compare execution times and print summary
    // compare_durations(serial_duration, concurrent_duration);

}

// TODO: Generate tasks
    // TODO: Execute tasks serially
    // TODO: Execute tasks concurrently
    // TODO: Compare execution times