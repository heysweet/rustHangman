use std::{io::{stdin, stdout, Write}};
use colored::Colorize;

const MAX_FAILURES: u8 = 6; // head, body, 2 arms, 2 legs

enum RoundOutcome {
    /// Discovered a new letter
    Hit,
    /// Found a letter not in the word
    Miss,
    /// Already guessed this letter
    Duplicate,
    /// Successfully found the word
    Win,
    /// Failed to find the word
    Lose
}

/// Displays how much of the word has been correctly guessed so far.
/// All missing letters will be represented with underscores.
fn show_word(guesses: &Vec<char>, target_word: &str) -> (String, i32) {
    let mut result = String::with_capacity(target_word.len() * 2);
    let mut remaining_letters = 0;

    // Technically O(n**2) but we're looking at 26 letters and the target word length so...
    // Inefficieny is fine.
    for char in target_word.chars() {
        if guesses.contains(&char) {
            result.push(char);
        } else {
            result.push('_');
            remaining_letters += 1;
        }
        result.push(' ')
    }
    return (result, remaining_letters);
}

/// Informs the user what letters they've guessed so far,
/// and then accepts user input and validates it.
fn prompt_user_input(guesses: &Vec<char>, target_word: &str) -> String {
    let mut user_input: String = String::new();

    println!("");
    if guesses.len() > 0 {
        println!("Guesses: {:?}", guesses);
    }
    let (word, _) = show_word(&guesses, target_word);
    println!("{}", word);
    print!("Enter your guess: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut user_input).expect("Did not get user input");

    return user_input;
}

fn validate_char(user_input: &String) -> Result<char, &str> {
    // character followed by newline
    if user_input.len() != 2 {
        return Err("Expected 1 character");
    }
    // I think I'd want to unwrap with an error message?
    // Shadowing!
    let char = user_input.chars().next().unwrap();
    let char: char = char.to_lowercase().next().unwrap();

    if !char.is_alphabetic() {
        return Err("Not a valid letter.");
    }

    return Ok(char);
}

/// Determines the outcome of the round.
fn score(guesses: &mut Vec<char>, char: char, target_word: &str, num_wrong_remaining: &mut u8) -> RoundOutcome {
    if guesses.contains(&char) {
        return RoundOutcome::Duplicate;
    }
    
    guesses.push(char);

    if !target_word.contains(char) {
        *num_wrong_remaining -= 1;
        if *num_wrong_remaining <= 0 {
            return RoundOutcome::Lose;
        }
        return RoundOutcome::Miss;
    }
    
    let (_, num_blanks_remaining) = show_word(&guesses, target_word);
    if num_blanks_remaining == 0 {
        return RoundOutcome::Win;
    }
    return RoundOutcome::Hit;
}

fn play_hangman(target_word: &str) {
    let mut num_wrong_remaining = MAX_FAILURES;

    let mut guesses: Vec<char> = Vec::new();

    loop {
        let user_input = prompt_user_input(&guesses, target_word);

        let char = match validate_char(&user_input) {
            Ok(c) => { c }
            Err(message) => { 
                println!("{}", message.red());
                continue;
            }
            
        };
        
        let outcome = score(&mut guesses, char, target_word, &mut num_wrong_remaining);

        match outcome {
            RoundOutcome::Win => {
                println!("{}", "You win!".green());
                break;
            },
            RoundOutcome::Lose => {
                println!("{}", format!("You lose! The word was '{}'.", target_word).red());
                break;
            },
            RoundOutcome::Miss => {
                println!("{}", format!("'{}' in not in the word. You have {} more incorrect guesses.", char, num_wrong_remaining).red());
                continue;
            },
            RoundOutcome::Duplicate => {
                println!("{}", format!("You've already guessed '{}'.", char).red());
                continue;
            },
            RoundOutcome::Hit => {
                continue;
            },
        }
    }
}

fn main() {
    play_hangman("bartender");
}
