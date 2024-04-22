use rand::Rng;
use std::io::{self, Write};

fn validate_guess(guess: &str) -> (bool, Result<i32, &str>) {
    let guess = guess.parse::<i32>();

    match guess {
        Ok(guess) => {
            if guess > 10 || guess < 0 {
                return (false, Err("No. out of range"));
            }

            let random = rand::thread_rng().gen_range(1..=10);

            if guess == random {
                (true, Ok(random))
            } else {
                (false, Ok(random))
            }
        }
        Err(_err) => (false, Err("Invalid Number")),
    }
}

pub fn guess() {
    println!("Guess the number!");

    let mut guess = String::new();

    print!("Enter your guess(1-10): ");
    io::stdout().flush().expect("Failed to flush stdout!");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    guess = guess.trim().to_string();

    let (is_valid, random_number) = validate_guess(&guess);
    let res = if is_valid {
        format!("You guessed: {guess}\nGuessed Correctly!")
    } else {
        match random_number {
            Ok(res) => format!("You guessed: {guess}\nRandom No: {res} Game Over!"),
            Err(err) => format!("{err}"),
        }
    };
    println!("{res}");
}
