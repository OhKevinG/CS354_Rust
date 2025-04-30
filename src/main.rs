use rand::Rng;
use crate::task::TaskType;
use crate::task::Task;
use std::time::{Instant, Duration};

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

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

    let tasks = generate_tasks(batch_size);

    println!("\n--- Running tasks serially ---");
    let serial_duration = execute_serially(&tasks);

    println!("\n--- Running tasks concurrently ---");
    let concurrent_duration = execute_concurrently(&tasks, thread_count);

    // TODO: Compare execution times and print summary
    // compare_durations(serial_duration, concurrent_duration);

}

/// Generates a vector of random tasks based on the specified batch size.
pub fn generate_tasks(batch_size: u32) -> Vec<TaskType> {
    let mut rng = rand::thread_rng();
    let mut tasks = Vec::new();

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
                denominator: rng.gen_range(1..99) + 1, // avoid zero
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
                modulus: rng.gen_range(1..50) + 1, // avoid zero
            },
            _ => unreachable!(),
        };

        tasks.push(task);
    }

    tasks
}

/// Executes the list of tasks one by one and returns the total elapsed time.
fn execute_serially(tasks: &Vec<TaskType>) -> std::time::Duration {
    let start = Instant::now();
    for task in tasks {
        if let Err(e) = task.run() {
            eprintln!("Task failed: {}", e);
        }
    }
    start.elapsed()
}

/// Executes a list of tasks concurrently using a specified number of threads.
///
/// # Arguments
/// * `tasks` - A reference to the list of tasks to execute.
/// * `thread_count` - The number of worker threads to spawn.
///
/// # Returns
/// * `Duration` - The total time taken to execute all tasks.
pub fn execute_concurrently(tasks: &[TaskType], thread_count: u32) -> Duration {
    let queue = Arc::new(Mutex::new(VecDeque::from(tasks.to_vec())));
    let mut handles = Vec::new();

    let start_time = Instant::now();

    for _ in 0..thread_count {
        let task_queue = Arc::clone(&queue);

        let handle = thread::spawn(move || {
            loop {
                let maybe_task = {
                    let mut queue_guard = task_queue.lock().unwrap();
                    queue_guard.pop_front()
                };

                match maybe_task {
                    Some(task) => {
                        if let Err(e) = task.run() {
                            eprintln!("Error executing task: {}", e);
                        }
                    },
                    None => break, // Queue is empty, exit thread
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked during execution");
    }

    Instant::now() - start_time
}

// TODO: Compare execution times