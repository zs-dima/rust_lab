fn main() {
    println!("Factorial Sequence Calculator");

    // Print specific Factorial number
    let n = 5;
    println!("\nFactorial number at position {}: {}", n, factorial(n));

    let large_n = 100;
    match checked_factorial(large_n) {
        Some(result) => println!("Factorial of {} is {}", large_n, result),
        None => println!("Factorial of {} causes overflow!", large_n),
    }
}

// Calculate factorial recursively
//
// # Arguments
// * `n` - The input number
//
// # Returns
// The factorial of the number
fn factorial(n: u64) -> u64 {
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
fn checked_factorial(n: u64) -> Option<u64> {
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
