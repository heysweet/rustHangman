mod game_state;
mod user_input;
use game_state::GameState;

use crate::{
    game_state::RoundOutcome,
    user_input::{parse_char, prompt_user_input},
};

fn play_hangman(target_word: &str) {
    fn play(game_state: GameState) -> GameState {
        println!("{}", game_state);
        match game_state.round_outcome {
            Some(RoundOutcome::Win | RoundOutcome::Lose) => game_state,
            _ => {
                let user_input = prompt_user_input();

                match parse_char(&user_input) {
                    Ok(c) => {
                        let game_state = game_state.guess(c);
                        play(game_state)
                    }
                    Err(error) => {
                        println!("{}", error);
                        play(game_state.skip())
                    }
                }
            }
        }
    }

    play(GameState::new(target_word.to_owned()));
}

fn main() {
    play_hangman("bartender");
}
