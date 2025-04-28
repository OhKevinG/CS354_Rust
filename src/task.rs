/// A trait representing a unit of work that can be executed.
///
/// All task types (such as Compute, Fibonacci, etc.) must implement this trait.
/// The `run` method defines the execution behavior for the task,
/// returning `Ok(())` if successful or `Err(String)` if an error occurs.
pub trait Task {
    /// Executes the task.
    ///
    /// Returns:
    /// - `Ok(())` on successful completion
    /// - `Err(String)` if an error occurs during execution
    fn run(&self) -> Result<(), String>;
}

/// Represents the different types of tasks that can be executed.
/// Each variant defines its own behavior inside the `run` method.
#[derive(Clone)]
pub enum TaskType {
    
    /// Simple addition of two operators of a generic type.
    Compute { a: T, b: T },

    /// Calculate the nth Fibonacci number.
    Fibonacci { n: u32 },

    /// Divide numerator by denominator.
    Divide { numerator: T, denominator: T },

    /// Multiply two integers together.
    Multiply { a: T, b: T },

    /// Compute factorial of a number.
    Factorial { n: u32 },

    /// Check if a number is prime.
    PrimeCheck { n: u32 },

    /// Perform modular exponentiation (base^exponent mod modulus).
    ModuloExponentiation { base: u64, exponent: u64, modulus: u64 },
}

/// Implements the `Task` trait for `TaskType`.
///
/// Defines how each type of task (Compute, Fibonacci, etc.) is executed
/// by matching on the variant and performing the appropriate computation.
impl Task for TaskType {

    /// Executes the task and returns success or an error message.
    fn run(&self) -> Result<(), String> {
        match self {
            // TODO: Match arms calling appropriate helpers
        }
    }
}