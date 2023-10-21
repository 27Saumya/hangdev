use std::io;
use colored::Colorize;
mod fetch;
mod ui;

fn main() {
    println!("Welcome to Hangdev - The Developers' Hangman!");

    let word = fetch::fetch_random_word().expect(&ui::error_message("Failed to fetch word"));

    let secret_word = word[0].to_lowercase();
    let mut guessed_letters = vec!['_'; secret_word.len()];

    let mut incorrect_attempts = 0;
    let max_attempts = 6;

    while incorrect_attempts < max_attempts {
        ui::print_hangman(incorrect_attempts);
        println!("Word: {}", guessed_letters.iter().collect::<String>().green());
        println!("Incorrect attempts left: {}", (max_attempts - incorrect_attempts).to_string().red());

        println!("Please enter a letter:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if guess.len() != 1 || !guess.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("{}", (ui::error_message("Invalid input. Please enter a single letter")));
            continue;
        }

        let mut correct_guess = false;

        for (i, letter) in secret_word.chars().enumerate() {
            if letter == guess.chars().next().unwrap() {
                guessed_letters[i] = letter;
                correct_guess = true;
            }
        }

        if !correct_guess {
            let incorrect = "Incorrect guess. Try again!".red();
            println!("{}", (incorrect));
            incorrect_attempts += 1;
        }

        if guessed_letters.iter().collect::<String>() == secret_word {
            println!("Congratulations! You guessed the word: {}", secret_word.green());
            break;
        }
    }

    if incorrect_attempts == max_attempts {
        ui::print_hangman(incorrect_attempts);
        println!("You're out of attempts. The word was: {}", secret_word.red());
    }
}
