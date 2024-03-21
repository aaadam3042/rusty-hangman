mod utils;
mod ui;
use std::fs;
use rand::Rng;
use crate::utils::clear_terminal;

fn main() {
    println!("Welcome to Rusty Hangman!");
    
    println!("Starting new game!");   
    utils::clear_terminal();

    let word = fs::read_to_string("src/resources/dictionary.txt").expect("Error reading file");
    let lines = word.lines().count();

    let random_number = rand::thread_rng().gen_range(1..=lines);
    let chosen_word = word.lines().nth(random_number).unwrap().to_lowercase();

    let mut guessed_letters: Vec<char> = Vec::new();
    let total_attempts = 8; // Minimum attempts should be 8 - can make less if we skip frames
    let mut attempt = 0;
    let mut guessed_word = "_".repeat(chosen_word.len());

    while attempt < total_attempts {
        println!("{}", guessed_word.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));

        println!("Guesses left: {}", total_attempts - attempt); 
        println!("Current guesses: {:?}", guessed_letters.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        println!("{}", ui::get_frame(attempt, false));
        let guess = utils::get_input("Guess a letter: ".to_string());
        
        
        // Check if guess is a single char
        if guess.len() > 1 || guess.len() == 0 {
            clear_terminal();
            println!("Please enter a single letter");
            continue;
        }
        
        let guess_c = guess.chars().next().unwrap();    
        if !guess_c.is_alphabetic() {
            clear_terminal();
            println!("Please enter a letter");
            continue;
        }

        // Check if the letter has already been guessed
        if guessed_letters.contains(&guess_c) {
            println!("You've already guessed that letter");
            clear_terminal();
            continue;
        }
        
        // Check if the letter is in the word
        if chosen_word.contains(guess_c) {
            println!("Correct!");
            chosen_word.chars().enumerate().for_each(|(i, c)| {
                if c == guess_c {
                    guessed_word.replace_range(i..i+1, &c.to_string());
                }
            });
        } else {
            println!("Incorrect!");
            attempt += 1;
        }
        guessed_letters.push(guess_c);
        
        clear_terminal();
        
        if chosen_word == guessed_word {
            println!("Congratulations! You've guessed the word: {}", chosen_word);  
            println!("{}", ui::get_frame(attempt, true));
            break;
        } else if attempt == total_attempts {
            println!("You lose! The word was: {}.", chosen_word);  
            println!("{}", ui::get_frame(attempt, false));
            break;
        }
    }
    
    // Every time the player guesses a letter add to guessed letters and display
    // Also animation frame should change every time the player guesses a letter with a modification to meet total attempts
}
