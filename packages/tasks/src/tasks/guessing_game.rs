pub mod guessing_game {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    pub fn guessing() {
        println!("Guess the number!");

        let secret_number = rand::rng().random_range::<u32, _>(1..=100);

        println!("The secret number is: {}", secret_number);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("Please type a number! {}", err);
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
