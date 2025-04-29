/// Collection of helper functions for mathematical tasks.
/// 
/// TODO: Implement each helper function as needed.
pub mod helpers {

    // TODO: Implement fibonacci()
    
    // factorial()
    /// Calculate factorial of a number iteratively.
    pub fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            (1..=n).product()
        }
    }

    // adds two operators of a generic type
    pub fn compute<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
        a + b
    }

    // divides two operators of a generic type
    pub fn divide<T: std::ops::Div<Output = T>>(a: T, b: T) -> T{
        a / b
    }

    // multiplies two operators of a generic type
    pub fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T{
        a * b
    }

    // determines if unsigned integer is a prime number 
    pub fn prime_check( n: u32 ) -> bool{
        for i in 2..((n/2)+1) {
            // Uses iterator " 2..((n/2)+1) " which gets all values between 2 and half of n + 1
            if i * (n/i) == n {
                return false;
            }
        }
        true
    }

    // Modulo exponentiation
    pub fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        // Special case: anything mod 1 is 0
        if modulus == 1 {
            return 0;
        }
        // Initialize result as 1
        let mut result = 1;    
        // Ensure base is reduced to avoid unnecessary large numbers
        base %= modulus;
        // Loop while there are still bits in the exponent
        while exp > 0 {
            // If the current bit of exp is 1, multiply result by current base
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            // Shift exp right by 1 bit (divide by 2)
            exp /= 2;
            // Square the base for next iteration
            base = (base * base) % modulus;
        }
        result
    }
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
    // Basic cases
        assert_eq!(mod_exp(2, 3, 5), 3);     // 2^3 = 8, 8 % 5 = 3
        assert_eq!(mod_exp(5, 0, 7), 1);     // 5^0 = 1, 1 % 7 = 1
        assert_eq!(mod_exp(7, 4, 9), 4);     // 7^4 = 2401, 2401 % 9 = 4
        assert_eq!(mod_exp(10, 5, 13), 4);   // 10^5 = 100000, 100000 % 13 = 4
    }
}