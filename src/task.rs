use crate::helpers;

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
    Compute { a: i32, b: i32 },

    /// Calculate the nth Fibonacci number.
    Fibonacci { n: u32 },

    /// Divide numerator by denominator.
    Divide { numerator: i32, denominator: i32 },

    /// Multiply two integers together.
    Multiply { a: i32, b: i32 },

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
            TaskType::Compute { a, b } => {
                let result = helpers::compute::<i32>(*a, *b);
                println!("Computed {} + {} = {}", a, b, result);
                Ok(())
            }

            TaskType::Fibonacci { n } => {
                let result = helpers::fibonacci(*n as u64);
                println!("Fibonacci({}) = {}", n, result);
                Ok(())
            }

            TaskType::Divide { numerator, denominator } => {
                if *denominator == 0 {
                    return Err("Division by zero.".to_string());
                }
                let result = helpers::divide::<i32>(*numerator, *denominator);
                println!("Division: {} / {} = {}", numerator, denominator, result);
                Ok(())
            }

            TaskType::Multiply { a, b } => {
                let result = helpers::multiply::<i32>(*a, *b);
                println!("Multiplication: {} * {} = {}", a, b, result);
                Ok(())
            }

            TaskType::Factorial { n } => {
                let result = helpers::factorial(*n as u64);
                println!("Factorial({}) = {}", n, result);
                Ok(())
            }

            TaskType::PrimeCheck { n } => {
                let result = helpers::prime_check(*n);
                println!("{} is {}", n, if result { "prime" } else { "not prime" });
                Ok(())
            }

            TaskType::ModuloExponentiation { base, exponent, modulus } => {
                if *modulus == 0 {
                    return Err("Modulus cannot be zero.".to_string());
                }
                let result = helpers::mod_exp(*base, *exponent, *modulus);
                println!("ModExp: {}^{} % {} = {}", base, exponent, modulus, result);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_task() {
        let task = TaskType::Compute { a: 3, b: 4 };
        assert_eq!(task.run(), Ok(()));
    }

    #[test]
    fn test_fibonacci_task() {
        let task = TaskType::Fibonacci { n: 7 };
        assert_eq!(task.run(), Ok(())); // Fibonacci(7) = 21
    }

    #[test]
    fn test_divide_task_valid() {
        let task = TaskType::Divide { numerator: 10, denominator: 2 };
        assert_eq!(task.run(), Ok(()));
    }

    #[test]
    fn test_divide_task_by_zero() {
        let task = TaskType::Divide { numerator: 10, denominator: 0 };
        assert_eq!(task.run(), Err("Division by zero.".to_string()));
    }

    #[test]
    fn test_multiply_task() {
        let task = TaskType::Multiply { a: 6, b: 7 };
        assert_eq!(task.run(), Ok(()));
    }

    #[test]
    fn test_factorial_task() {
        let task = TaskType::Factorial { n: 5 };
        assert_eq!(task.run(), Ok(())); // 5! = 120
    }

    #[test]
    fn test_prime_check_task_prime() {
        let task = TaskType::PrimeCheck { n: 7 };
        assert_eq!(task.run(), Ok(()));
    }

    #[test]
    fn test_modulo_exponentiation_task_valid() {
        let task = TaskType::ModuloExponentiation {
            base: 3,
            exponent: 4,
            modulus: 5,
        };
        assert_eq!(task.run(), Ok(())); // 3^4 % 5 = 81 % 5 = 1
    }

    #[test]
    fn test_modulo_exponentiation_task_zero_modulus() {
        let task = TaskType::ModuloExponentiation {
            base: 2,
            exponent: 3,
            modulus: 0,
        };
        assert_eq!(task.run(), Err("Modulus cannot be zero.".to_string()));
    }
}
