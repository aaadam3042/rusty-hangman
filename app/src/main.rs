mod utils;
use std::fs;
use rand::Rng;

fn main() {
    println!("Welcome to Rusty Hangman!");
    
    let username = utils::get_input("What is your name: ".to_string());
    println!("Hello, {}!", username);   

    let word = fs::read_to_string("src/resources/dictionary.txt").expect("Error reading file");
    let lines = word.lines().count();

    let random_number = rand::thread_rng().gen_range(1..=lines);
    let chosen_word = word.lines().nth(random_number).unwrap();

    println!("{} - {}", random_number, chosen_word);

    let mut guessed_letters: Vec<char> = Vec::new();
    let total_attempts = 6;
    let mut attempt = 0;

    // Every time the player guesses a letter add to guessed letters and display
    // Also animation frame should change every time the player guesses a letter with a modification to meet total attempts
}
