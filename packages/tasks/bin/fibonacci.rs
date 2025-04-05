use tasks::fibonacci_task as fibonacci;

fn main() {
    println!("Fibonacci Sequence Calculator");

    // Print first 10 Fibonacci numbers (iterative approach)
    println!("\nFirst 10 Fibonacci numbers:");
    fibonacci::print_fibonacci_sequence(10);

    // Print specific Fibonacci number
    let n = 10;
    println!(
        "\nFibonacci number at position {}: {}",
        n,
        fibonacci::fibonacci(n)
    );
}
