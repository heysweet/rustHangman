use core::fmt;
use std::collections::HashSet;

use colored::Colorize;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SolutionState {
    pub guesses: Vec<char>,
    pub target_word: String,
}

impl SolutionState {
    pub fn get_remaining_letters(&self) -> usize {
        let unique_letters: HashSet<char> = self.target_word.chars().collect();
        let guessed_letters = HashSet::from_iter(self.guesses.clone().into_iter());
        let remaining_letters = unique_letters.difference(&guessed_letters);

        let missing_chars: Vec<&char> = remaining_letters.collect();
        missing_chars.len()
    }

    fn new(target_word: String) -> SolutionState {
        SolutionState {
            guesses: Vec::new(),
            target_word,
        }
    }

    pub fn add_guess(&self, char: char) -> SolutionState {
        let mut new_guesses = self.guesses.clone();
        if !new_guesses.contains(&char) {
            new_guesses.push(char);
        }
        SolutionState {
            guesses: new_guesses,
            target_word: self.target_word.clone(),
        }
    }

    pub fn has_seen(&self, char: &char) -> bool {
        self.guesses.contains(char)
    }
}

impl fmt::Display for SolutionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(&self.target_word.len() * 2);
        for char in self.target_word.chars() {
            result.push(if self.guesses.contains(&char) {
                char
            } else {
                '_'
            });
            result.push(' ');
        }
        write!(f, "{}", result)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GameState {
    pub round_outcome: Option<RoundOutcome>,
    solution_state: SolutionState,
    last_guess: char,
    num_wrong_remaining: u8,
}

const MAX_FAILURES: u8 = 6; // head, body, 2 arms, 2 legs

impl GameState {
    pub fn new(target_word: String) -> GameState {
        GameState {
            solution_state: SolutionState::new(target_word.clone()),
            round_outcome: None,
            num_wrong_remaining: MAX_FAILURES,
            last_guess: ' ',
        }
    }

    pub fn skip(&self) -> GameState {
        GameState { round_outcome: None, solution_state: self.solution_state.clone(), last_guess: self.last_guess, num_wrong_remaining: self.num_wrong_remaining }
    }

    pub fn get_all_guesses(&self) -> Vec<char> {
        self.solution_state.guesses.clone()
    }

    pub fn get_obfuscated_word(&self) -> String {
        format!("{}", self.solution_state)
    }

    fn score(&self, new_solution_state: &SolutionState, guess: char) -> RoundOutcome {
        let is_duplicate = self.solution_state.has_seen(&guess);
        let is_in_word = self.solution_state.target_word.contains(guess);
        let num_blanks_remaining = new_solution_state.get_remaining_letters();

        match (
            is_duplicate,
            is_in_word,
            num_blanks_remaining,
            self.num_wrong_remaining,
        ) {
            (false, false, _, 1) => RoundOutcome::Lose,
            (_, _, 0, _) => RoundOutcome::Win,
            (true, _, _, _) => RoundOutcome::Duplicate,
            (_, false, _, _) => RoundOutcome::Miss,
            _ => RoundOutcome::Hit,
        }
    }

    pub fn guess(&self, guess: char) -> GameState {
        let new_solution_state = self.solution_state.add_guess(guess);
        let round_outcome = self.score(&new_solution_state, guess);
        let num_wrong_remaining = self.num_wrong_remaining
            - if round_outcome == RoundOutcome::Miss {
                1
            } else {
                0
            };

        GameState {
            solution_state: new_solution_state,
            round_outcome: Some(round_outcome),
            num_wrong_remaining,
            last_guess: guess,
        }
    }

    /// Informs the user what letters they've guessed so far,
    /// and then accepts user input and validates it.
    fn display_current_state(&self) -> String {
        let guesses = self.get_all_guesses();
        let obfuscated_word = self.get_obfuscated_word();

        format!("\n\nGuesses: {:?}\n{}", guesses, obfuscated_word).to_string()
    }

    fn display_outcome(&self) -> String {
        match &self.round_outcome {
            Some(RoundOutcome::Hit) => "".to_string(),
            Some(RoundOutcome::Miss) => {
                format!(
                    "{}",
                    format!(
                        "'{}' in not in the word. You have {} more incorrect guesses.",
                        self.last_guess, self.num_wrong_remaining
                    )
                    .red()
                )
            }
            Some(RoundOutcome::Duplicate) => {
                format!(
                    "{}",
                    format!("You've already guessed '{}'.", self.last_guess).red()
                )
            }
            Some(RoundOutcome::Win) => {
                format!(
                    "{}",
                    format!(
                        "You win! The word was '{}'.",
                        self.solution_state.target_word
                    )
                    .green()
                )
            }
            Some(RoundOutcome::Lose) => {
                format!(
                    "{}",
                    format!(
                        "You lose! The word was '{}'.",
                        self.solution_state.target_word
                    )
                    .red()
                )
            }
            None => "".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RoundOutcome {
    /// Discovered a new letter
    Hit,
    /// Found a letter not in the word
    Miss,
    /// Already guessed this letter
    Duplicate,
    /// Successfully found the word
    Win,
    /// Failed to find the word
    Lose,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.display_outcome(),
            self.display_current_state()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ParseError {
    ExpectedOneCharacterInput { user_input: String },
    NonAlphabeticInput { user_input: char },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ExpectedOneCharacterInput { user_input } => {
                let message = format!("'{}' is not one character.", user_input).red();
                write!(f, "{}", message)
            }
            Self::NonAlphabeticInput { user_input } => {
                let message = format!("'{}' is non alphabetic.", user_input).red();
                write!(f, "{}", message)
            }
        }
    }
}
