use clap::{Parser, Subcommand};

use tasks::factorial_task as factorial;
use tasks::fibonacci_task as fibonacci;
use tasks::guessing_game_task as guessing_game_main;

/// rust_lab is a collection of Rust code samples for learning Rust programming concepts.
///
/// You can run rust_lab with various subcommands and options.
/// Try `--help` to see all available flags.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional subcommands
    #[command(subcommand)]
    command: Option<Commands>,

    /// Turn debugging information on
    #[arg(short, long, help = "Enables verbose output for debugging")]
    debug: bool,
}

/// Subcommands available in rust_lab
#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the guessing game
    GuessingGame,
    /// Run factorial calculation
    Factorial {
        /// Number for factorial calculation
        #[arg(
            short,
            long,
            default_value_t = 1,
            help = "Your number to calculate factorial"
        )]
        number: u8,
    },
    /// Run fibonacci calculation
    Fibonacci {
        /// Number for fibonacci calculation
        #[arg(
            short,
            long,
            default_value_t = 1,
            help = "Your number to calculate fibonacci"
        )]
        number: u8,
    },
    /// Run palindrome check
    Palindrome {
        /// String to check for palindrome
        #[arg(
            short,
            long,
            default_value = "racecar",
            help = "Your string to check for palindrome"
        )]
        string: String,
    },
}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Debugging is enabled!");
    }

    match &args.command {
        Some(Commands::GuessingGame) => {
            println!("Running the guessing game...");
            guessing_game_main::guessing();
        }
        Some(Commands::Factorial { number }) => {
            println!(
                "\nFactorial number at position {}: {}",
                number,
                factorial::factorial(*number as u64)
            );
        }
        Some(Commands::Fibonacci { number }) => {
            println!(
                "\nFibonacci number at position {}: {}",
                number,
                fibonacci::fibonacci(*number as u32)
            );
        }
        Some(Commands::Palindrome { string }) => {
            let is_palindrome = string.chars().eq(string.chars().rev());
            if is_palindrome {
                println!("The string '{}' is a palindrome.", string);
            } else {
                println!("The string '{}' is not a palindrome.", string);
            }
        }
        None => {
            println!("Hello, world!");

            println!("No subcommand was used.");
        }
    }
}
