use std::io::{stdin, stdout, Write};

use crate::game_state::ParseError;

/// Informs the user what letters they've guessed so far,
/// and then accepts user input and validates it.
pub fn prompt_user_input() -> String {
    let mut user_input: String = String::new();

    print!("Enter your guess: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut user_input)
        .expect("Did not get user input");

    user_input
}

pub fn parse_char(user_input: &String) -> Result<char, ParseError> {
    let mut lowercased_chars = user_input.to_lowercase();
    // Remove the trailing "\n"
    lowercased_chars.pop();
    match (
        lowercased_chars.chars().nth(0),
        lowercased_chars.chars().nth(1),
    ) {
        (Some(char), None) => {
            if !char.is_alphabetic() {
                Err(ParseError::NonAlphabeticInput { user_input: char })
            } else {
                Ok(char)
            }
        }
        _ => Err(ParseError::ExpectedOneCharacterInput {
            user_input: lowercased_chars.clone(),
        }),
    }
}
