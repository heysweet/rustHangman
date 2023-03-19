use std::io::{stdin, stdout, Write};

mod game_state;
use game_state::{GameState, HangmanError};

use crate::game_state::RoundOutcome;

/// Informs the user what letters they've guessed so far,
/// and then accepts user input and validates it.
fn prompt_user_input(game_state: &GameState) -> String {
    let mut user_input: String = String::new();
    let guesses = game_state.get_all_guesses();

    println!("");
    if guesses.len() > 0 {
        println!("Guesses: {:?}", guesses);
    }
    let obfuscated_word = game_state.get_obfuscated_word();
    println!("{}", obfuscated_word);
    print!("Enter your guess: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut user_input)
        .expect("Did not get user input");

    user_input
}

fn parse_char(user_input: &String) -> Result<char, HangmanError> {
    let mut lowercased_chars = user_input.to_lowercase();
    lowercased_chars.pop();
    match (
        lowercased_chars.chars().nth(0),
        lowercased_chars.chars().nth(1),
    ) {
        (Some(char), None) => {
            if !char.is_alphabetic() {
                Err(HangmanError::NonAlphabeticInput { user_input: char })
            } else {
                Ok(char)
            }
        }
        _ => Err(HangmanError::ExpectedOneCharacterInput {
            user_input: lowercased_chars.clone(),
        }),
    }
}

fn play_hangman(target_word: &str) {
    let mut game_state: GameState = GameState::new(target_word.to_string());

    loop {
        println!("{}", game_state);
        match game_state.round_outcome {
            RoundOutcome::Win | RoundOutcome::Lose => {
                break;
            }
            _ => (),
        }
        let user_input = prompt_user_input(&game_state);

        let char = match parse_char(&user_input) {
            Ok(c) => c,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        game_state = game_state.guess(char);
    }
}

fn main() {
    play_hangman("bartender");
}
