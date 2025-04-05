//! #![doc = include_str!("../README.md")]
use tasks::factorial_task as factorial;

fn main() {
    println!("Factorial Sequence Calculator");

    // Print specific Factorial number
    let n = 5;
    println!(
        "\nFactorial number at position {}: {}",
        n,
        factorial::factorial(n)
    );

    let large_n = 100;
    match factorial::checked_factorial(large_n) {
        Some(result) => println!("Factorial of {} is {}", large_n, result),
        None => println!("Factorial of {} causes overflow!", large_n),
    }
}
