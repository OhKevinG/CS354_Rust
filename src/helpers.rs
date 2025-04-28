/// Collection of helper functions for mathematical tasks.
/// 
/// TODO: Implement each helper function as needed.
pub mod helpers {
    // TODO: Implement fibonacci()
    // factorial()
    /// Calculate factorial of a number iteratively.
    /// 0! is defined as 1.
    pub fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            (1..=n).product()
        }
    }

    // adds two operators of a generic type
    pub fn Compute<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
        a + b
    }

    // TODO: Implement prime checking
    // TODO: Implement modulo exponentiation
}



/// Unit tests for helper functions.
/// 
/// These tests verify the correctness of each mathematical helper function.
/// 
/// To run tests:
/// ```bash
/// cargo test
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        // TODO: Replace with real fibonacci() once implemented
        // Example: assert_eq!(fibonacci(5), 8);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);     // 0! = 1
        assert_eq!(factorial(1), 1);     // 1! = 1
        assert_eq!(factorial(5), 120);   // 5! = 120
        assert_eq!(factorial(10), 3628800); // 10! = 3628800
    }

    #[test]
    fn test_prime_check() {
        // TODO: Replace with real is_prime() once implemented
        // Example: assert_eq!(is_prime(7), true);
    }

    #[test]
    fn test_modulo_exponentiation() {
        // TODO: Replace with real mod_exp() once implemented
        // Example: assert_eq!(mod_exp(2, 3, 5), 3);
    }
}