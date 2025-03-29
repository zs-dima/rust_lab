fn main() {
    println!("Fibonacci Sequence Calculator");

    // Print first 10 Fibonacci numbers (iterative approach)
    println!("\nFirst 10 Fibonacci numbers:");
    print_fibonacci_sequence(10);

    // Print specific Fibonacci number
    let n = 10;
    println!("\nFibonacci number at position {}: {}", n, fibonacci(n));
}

// Calculate the nth Fibonacci number iteratively
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// Print Fibonacci sequence up to n numbers
fn print_fibonacci_sequence(n: usize) {
    if n == 0 {
        return;
    }

    let mut a = 0;
    let mut b = 1;

    println!("F(0) = {}", a);

    if n == 1 {
        return;
    }

    println!("F(1) = {}", b);

    for i in 2..n {
        let temp = a + b;
        println!("F({}) = {}", i, temp);
        a = b;
        b = temp;
    }
}
