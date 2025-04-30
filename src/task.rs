use crate::helpers;

/// A trait representing a unit of work that can be executed.
///
/// All task types (such as Compute, Fibonacci, etc.) must implement this trait.
/// The `run` method defines the execution behavior for the task,
/// optionally simulating a realistic workload and returning success or error.
pub trait Task {
    /// Executes the task.
    ///
    /// # Arguments
    /// * `simulate_load` - If true, introduces a brief delay to mimic heavier workloads.
    ///
    /// # Returns
    /// * `Ok(())` on successful task execution.
    /// * `Err(String)` if an error occurred.
    fn run(&self, simulate_load: bool) -> Result<(), String>;
}

/// Enum representing all possible types of tasks supported by the system.
///
/// Each variant contains the data necessary to perform that specific task.
#[derive(Clone)]
pub enum TaskType {
    Compute { a: i32, b: i32 },
    Fibonacci { n: u32 },
    Divide { numerator: i32, denominator: i32 },
    Multiply { a: i32, b: i32 },
    Factorial { n: u32 },
    PrimeCheck { n: u32 },
    ModuloExponentiation { base: u64, exponent: u64, modulus: u64 },
}

/// Implements the Task trait for TaskType.
///
/// Handles dispatching logic to the appropriate helper function depending on the task variant.
impl Task for TaskType {
    fn run(&self, simulate_load: bool) -> Result<(), String> {
        if simulate_load {
        }

        match self {
            TaskType::Compute { a, b } => {
                let _ = helpers::compute::<i32>(*a, *b);
                Ok(())
            }
            TaskType::Fibonacci { n } => {
                let _ = helpers::fibonacci(*n as u64);
                Ok(())
            }
            TaskType::Divide { numerator, denominator } => {
                if *denominator == 0 {
                    return Err("Division by zero.".into());
                }
                let _ = helpers::divide::<i32>(*numerator, *denominator);
                Ok(())
            }
            TaskType::Multiply { a, b } => {
                let _ = helpers::multiply::<i32>(*a, *b);
                Ok(())
            }
            TaskType::Factorial { n } => {
                let _ = helpers::factorial(*n as u64);
                Ok(())
            }
            TaskType::PrimeCheck { n } => {
                let _ = helpers::prime_check(*n);
                Ok(())
            }
            TaskType::ModuloExponentiation { base, exponent, modulus } => {
                if *modulus == 0 {
                    return Err("Modulus cannot be zero.".into());
                }
                let _ = helpers::mod_exp(*base, *exponent, *modulus);
                Ok(())
            }
        }
    }
}