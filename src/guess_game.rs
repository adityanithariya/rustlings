use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

#[allow(dead_code)]
fn validate_guess(guess: &str) -> (bool, Result<i32, &str>) {
    match guess.trim().parse::<i32>() {
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

#[allow(dead_code)]
pub fn guess() {
    println!("Guess the number!");

    let mut guess = String::new();

    print!("Enter your guess(1-10): ");
    io::stdout().flush().expect("Failed to flush stdout!");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let (is_valid, random_number) = validate_guess(&guess);
    let res = if is_valid {
        format!("You guessed: {guess}Guessed Correctly!")
    } else {
        match random_number {
            Ok(res) => format!("You guessed: {guess}Random No: {res} Game Over!"),
            Err(err) => format!("{err}"),
        }
    };
    println!("{res}");
}

#[allow(dead_code)]
fn validate_guess_iterative(guess: &str, random: i32) -> bool {
    match guess.trim().parse::<i32>() {
        Ok(guess) => match guess.cmp(&random) {
            Ordering::Equal => return true,
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
        },
        Err(_err) => println!("Invalid Number"),
    }
    false
}

#[allow(dead_code)]
pub fn guess_iterative() {
    println!("\nGuess the number!");

    let random = rand::thread_rng().gen_range(0..=100);
    let mut is_valid = false;

    while is_valid == false {
        print!("Enter your guess(1-100): ");
        io::stdout().flush().expect("Failed to flush stdout!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        is_valid = validate_guess_iterative(&guess, random);
    }
    println!("Guessed Correctly!");
}
