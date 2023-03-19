use std::fmt;

use colored::Colorize;

struct WordSoFar {
    pub guesses: Vec<char>,
    pub target_word: String,
}

impl WordSoFar {
    pub fn get_remaining_letters(&self) -> i32 {
        let mut remaining_letters = 0;
        for char in self.target_word.chars() {
            if !self.guesses.contains(&char) {
                remaining_letters += 1;
            }
        }
        remaining_letters
    }

    fn new(target_word: String) -> WordSoFar {
        Self {
            guesses: Vec::new(),
            target_word,
        }
    }

    pub fn add_guess(&self, char: char) -> WordSoFar {
        let mut new_guesses = self.guesses.clone();
        if !new_guesses.contains(&char) {
            new_guesses.push(char);
        }
        WordSoFar {
            guesses: new_guesses,
            target_word: self.target_word.clone(),
        }
    }

    pub fn has_seen(&self, char: &char) -> bool {
        self.guesses.contains(char)
    }
}

impl fmt::Display for WordSoFar {
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

pub struct GameState {
    pub round_outcome: RoundOutcome,
    word_so_far: WordSoFar,
    last_guess: char,
    num_wrong_remaining: u8,
}

const MAX_FAILURES: u8 = 6; // head, body, 2 arms, 2 legs

impl GameState {
    pub fn new(target_word: String) -> GameState {
        Self {
            word_so_far: WordSoFar::new(target_word.clone()),
            round_outcome: RoundOutcome::Hit,
            num_wrong_remaining: MAX_FAILURES,
            last_guess: ' ',
        }
    }

    pub fn get_all_guesses(&self) -> Vec<char> {
        self.word_so_far.guesses.clone()
    }

    pub fn get_obfuscated_word(&self) -> String {
        format!("{}", self.word_so_far)
    }

    fn score(&self, new_word_so_far: &WordSoFar, guess: char) -> RoundOutcome {
        let is_duplicate = self.word_so_far.has_seen(&guess);
        let is_in_word = self.word_so_far.target_word.contains(guess);
        let num_blanks_remaining = new_word_so_far.get_remaining_letters();

        match (
            is_duplicate,
            is_in_word,
            num_blanks_remaining,
            self.num_wrong_remaining,
        ) {
            (_, _, _, 0) => RoundOutcome::Lose,
            (_, _, 0, _) => RoundOutcome::Win,
            (true, _, _, _) => RoundOutcome::Duplicate,
            (_, false, _, _) => RoundOutcome::Miss,
            _ => RoundOutcome::Hit,
        }
    }

    pub fn guess(&self, guess: char) -> GameState {
        let new_word_so_far = self.word_so_far.add_guess(guess);
        let round_outcome = self.score(&new_word_so_far, guess);
        let num_wrong_remaining = self.num_wrong_remaining
            - if round_outcome == RoundOutcome::Miss {
                1
            } else {
                0
            };

        GameState {
            word_so_far: new_word_so_far,
            round_outcome,
            num_wrong_remaining,
            last_guess: guess,
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
        match &self.round_outcome {
            RoundOutcome::Hit => write!(f, ""),
            RoundOutcome::Miss => write!(
                f,
                "{}",
                format!(
                    "'{}' in not in the word. You have {} more incorrect guesses.",
                    self.last_guess, self.num_wrong_remaining
                )
                .red()
            ),
            RoundOutcome::Duplicate => write!(
                f,
                "{}",
                format!("You've already guessed '{}'.", self.last_guess).red()
            ),
            RoundOutcome::Win => write!(
                f,
                "{}",
                format!("You win! The word was '{}'.", self.word_so_far.target_word).green()
            ),
            RoundOutcome::Lose => write!(
                f,
                "{}",
                format!("You lose! The word was '{}'.", self.word_so_far.target_word).red()
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HangmanError {
    ExpectedOneCharacterInput { user_input: String },
    NonAlphabeticInput { user_input: char },
}

impl fmt::Display for HangmanError {
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
