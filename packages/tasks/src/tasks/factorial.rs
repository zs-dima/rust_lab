//! #![doc = include_str!("../README.md")]

pub mod factorial {
    // Calculate factorial recursively
    //
    // # Arguments
    // * `n` - The input number
    //
    // # Returns
    // The factorial of the number
    pub fn factorial(n: u64) -> u64 {
        if n == 0 || n == 1 {
            return 1;
        }

        factorial(n - 1) * n
    }

    /// Calculate factorial with overflow checking
    ///
    /// # Arguments
    /// * `n` - The input number
    ///
    /// # Returns
    /// Some(result) if calculation succeeds, None if overflow occurs
    pub fn checked_factorial(n: u64) -> Option<u64> {
        if n == 0 || n == 1 {
            return Some(1);
        }

        let mut result = 1u64;
        for i in 2..=n {
            // Check for potential overflow before multiplying
            result = result.checked_mul(i)?; // result *= i;        
        }

        Some(result)
    }
}
