use std::io::{self, Write};
use std::cmp::Ordering::*;

fn main() {
    let target_number = 10;
    start_guessing_game(target_number);
}

/// Starts the interactive number guessing game.
fn start_guessing_game(target: u32) {
    println!("🎯 Welcome to the Number Guessing Game!");

    loop {
        print!("👉 Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        if let Err(error) = io::stdin().read_line(&mut user_input) {
            eprintln!("❌ Error reading input: {error}");
            continue;
        }

        let parsed_guess = match user_input.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                println!("⚠️ That wasn't a valid number. Try again.");
                continue;
            }
        };

        match parsed_guess.cmp(&target) {
            Equal => {
                println!("✅ Congratulations! You guessed it: {}", parsed_guess);
                break;
            }
            Less => println!("🔻 Too low."),
            Greater => println!("🔺 Too high."),
        }
    }
}
