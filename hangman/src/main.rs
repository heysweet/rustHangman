use std::{io::{stdin, stdout, Write}};
use colored::Colorize;

const MAX_FAILURES: u8 = 6; // head, body, 2 arms, 2 legs

enum RoundOutcome {
    Hit,
    Miss,
    Duplicate,
    Win,
    Lose
}

fn show_word(guesses: &Vec<char>, target_word: &str) -> (String, i32) {
    let mut result = String::with_capacity(target_word.len() * 2);
    let mut remaining_letters = 0;

    // Technical O(n**2) but we're looking at 26 letters and the target word length so...
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

fn prompt(guesses: &Vec<char>, target_word: &str, user_input: &mut String) {
    println!("");
    if guesses.len() > 0 {
        println!("Guesses: {:?}", guesses);
    }
    let (word, _) = show_word(&guesses, target_word);
    println!("{}", word);
    print!("Enter your guess: ");
    stdout().flush().unwrap();

    user_input.clear();
    stdin().read_line(user_input).expect("Did not get user input");
}

fn validate_char(user_input: &mut String) -> Result<char, &str> {
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

fn main() {
    let target_word = "bartender";
    let mut num_wrong_remaining = MAX_FAILURES;

    let mut guesses: Vec<char> = Vec::new();

    let mut user_input: String = String::new();

    loop {
        prompt(&guesses, target_word, &mut user_input);

        let char = match validate_char(&mut user_input) {
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
