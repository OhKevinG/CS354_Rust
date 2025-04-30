use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::task::TaskType;
use crate::task::Task;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use std::io;

mod task;
mod helpers;

/// Entry point for the program. Presents a CLI for configuring and benchmarking task execution.
///
/// Prompts the user to choose between two execution modes:
/// 1. Default (concurrent execution using a mutex-protected queue)
/// 2. Simulated task load (adds delay to simulate real-world task latency)
///
/// The user is then asked to specify:
/// - The number of tasks to generate
/// - The number of threads to use for concurrent execution (validated against CPU count)
///
/// The program generates a set of tasks and benchmarks both serial and concurrent execution.
/// Finally, it compares and prints the time each approach took.
fn main() {
    println!("Choose mode:");
    println!("[1] Default (Mutex-based concurrency)");
    println!("[2] Simulate realistic task load");

    // Loop until the user enters a valid mode (1 or 2)
    let mode = loop {
        let m = prompt_for_u32("Enter choice:");
        if m == 1 || m == 2 {
            break m;
        } else {
            println!("Invalid mode. Please enter 1 or 2.");
        }
    };
    
    // Enable simulated load delay if user selects mode 2.
    let simulate_load = mode == 2;

    // Prompt for how many tasks to generate
    let batch_size = prompt_for_u32("Enter number of tasks to generate:");
    
    // Get number of threads from user input.
    let max_threads = num_cpus::get() as u32;
    let thread_count = loop {
        let count = prompt_for_u32(&format!("Enter number of threads [1-{}]:", max_threads));
        if count > 0 && count <= max_threads {
            break count;
        } else {
            println!("Please enter a number between 1 and {}.", max_threads);
        }
    };

    println!("Generating {} tasks...", batch_size);
    println!("Using {} threads for concurrent execution.", thread_count);

    // Generate a set of tasks
    let tasks = generate_tasks(batch_size);

    // Run the tasks serially and measure the execution time
    println!("\n--- Running tasks serially ---");
    let serial_duration = execute_serially(&tasks, simulate_load);

    // Run the tasks concurrently and measure the execution time
    println!("\n--- Running tasks concurrently ---");
    let concurrent_duration = execute_concurrently(&tasks, thread_count, simulate_load);

    // Compare and summarize both execution durations
    compare_durations(serial_duration, concurrent_duration);
}

/// Prompts the user for a positive integer and validates the input.
///
/// # Arguments
/// * `prompt` - A string prompt to display to the user.
///
/// # Returns
/// A validated, non-zero `u32` entered by the user.
fn prompt_for_u32(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        match input.trim().parse::<u32>() {
            Ok(num) if num > 0 => return num, // Valid number > 0
            Ok(_) => println!("Please enter a number greater than 0."), // Zero or negative
            Err(_) => println!("Invalid number. Try again."), // Not a number
        }
    }
}

/// Generates a list of random tasks from all supported `TaskType` variants.
/// It randomly chooses one of the task variants on each iteration,
/// and generates suitable input values within reasonable ranges for each variant.
///
/// # Arguments
/// * `batch_size` - The total number of tasks to generate.
///
/// # Returns
/// A `Vec<TaskType>` containing `batch_size` randomly generated tasks.
pub fn generate_tasks(batch_size: u32) -> Vec<TaskType> {
    let mut tasks = Vec::new();

    // Use a seeded RNG for deterministic task generation
    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..batch_size {
        let task_type = rng.gen_range(0..7);
        let task = match task_type {
            0 => TaskType::Compute {
                a: rng.gen_range(1..100),
                b: rng.gen_range(1..100),
            },
            1 => TaskType::Fibonacci {
                n: rng.gen_range(1..30),
            },
            2 => TaskType::Divide {
                numerator: rng.gen_range(1..100),
                denominator: rng.gen_range(1..99) + 1, // Avoid division by zero
            },
            3 => TaskType::Multiply {
                a: rng.gen_range(1..100),
                b: rng.gen_range(1..100),
            },
            4 => TaskType::Factorial {
                n: rng.gen_range(0..20),
            },
            5 => TaskType::PrimeCheck {
                n: rng.gen_range(1..100),
            },
            6 => TaskType::ModuloExponentiation {
                base: rng.gen_range(2..20),
                exponent: rng.gen_range(2..10),
                modulus: rng.gen_range(1..50) + 1, // Ensure non-zero modulus
            },
            _ => unreachable!(), // Should never happen given the 0..7 range
        };
        tasks.push(task);
    }
    tasks
}

/// Executes a list of tasks one at a time in serial order,
/// measuring the total time taken to complete all tasks.
///
/// # Arguments
/// * `tasks` - A reference to a vector of `TaskType` values to be executed.
/// * `simulate_load` - If `true`, introduces a brief delay before each task runs.
///
/// # Returns
/// A `Duration` representing the total elapsed time to run all tasks serially.
fn execute_serially(tasks: &Vec<TaskType>, simulate_load: bool) -> Duration {
    let start = Instant::now();
    for task in tasks {
        if simulate_load {
            std::thread::sleep(Duration::from_micros(100));
        }
        if let Err(e) = task.run(simulate_load) {
            eprintln!("Task failed: {}", e);
        }
    }
    start.elapsed()
}

/// Executes a list of tasks concurrently using multiple threads and returns the total duration.
///
/// Tasks are distributed among threads by having each thread pop from a shared task queue
/// protected by a mutex. Threads continue pulling tasks until the queue is empty.
///
/// # Arguments
/// * `tasks` - A slice of `TaskType` elements to be executed.
/// * `thread_count` - Number of threads to spawn for concurrent execution.
/// * `simulate_load` - If `true`, introduces a fixed artificial delay before each task is run.
///
/// # Returns
/// A `Duration` representing the total elapsed time to execute all tasks concurrently.
pub fn execute_concurrently(tasks: &[TaskType], thread_count: u32, simulate_load: bool) -> Duration {
    
    // Wrap the task queue in Arc<Mutex<...>> to allow shared, synchronized access across threads.
    let queue = Arc::new(Mutex::new(VecDeque::from(tasks.to_vec())));
    let mut handles = Vec::new();

    let start_time = Instant::now();

    // Launch the specified number of worker threads
    for _ in 0..thread_count {
        let task_queue = Arc::clone(&queue);

        let handle = thread::spawn(move || {
            loop {
                 // Lock the queue and try to pop the next task
                let maybe_task = {
                    let mut queue_guard = task_queue.lock().unwrap();
                    queue_guard.pop_front()
                };

                match maybe_task {
                    Some(task) => {
                        // Simulate load if enabled (optional delay before running the task)
                        if simulate_load {
                            std::thread::sleep(Duration::from_micros(100));
                        }
                        // Execute the task and log any errors
                        if let Err(e) = task.run(simulate_load) {
                            eprintln!("Error executing task: {}", e);
                        }
                    },
                    None => break, // Exit the loop if the queue is empty
                }
            }
        });
        // Store the handle so we can join it later
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked during execution");
    }

    Instant::now() - start_time
}

/// Compares the durations of serial and concurrent execution and prints a performance summary.
///
/// # Arguments
/// * `serial` - Duration of the serial task execution.
/// * `concurrent` - Duration of the concurrent task execution.
fn compare_durations(serial: Duration, concurrent: Duration) {
    println!("\n=== Execution Time Summary ===");
    println!("Serial execution took:     {:.2?}", serial);
    println!("Concurrent execution took: {:.2?}", concurrent);

    // Compare the durations and report whether concurrency improved or hurt performance
    if serial > concurrent {
        let speedup = serial.as_secs_f64() / concurrent.as_secs_f64();
        println!("Concurrent execution was {:.2}× faster.", speedup);
    } else if concurrent > serial {
        let slowdown = concurrent.as_secs_f64() / serial.as_secs_f64();
        println!("⚠️  Serial execution was {:.2}× faster.", slowdown);
    } else {
        println!("Execution times were equal.");
    }
}
