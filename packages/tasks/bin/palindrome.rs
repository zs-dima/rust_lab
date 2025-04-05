use tasks::palindrome_task as palindrome;

fn main() {
    println!("Factorial Sequence Calculator");

    // Print specific Factorial number
    let n = "aolfloa";
    println!(
        "\nIs the string a palindrome {}: {}",
        n,
        palindrome::is_palindrome(n)
    );

    println!(
        "\nThe string a palindrome part {}: {}",
        n,
        palindrome::palindrome_slice(n)
    );
}
