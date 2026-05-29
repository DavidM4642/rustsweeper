use inquire::Text;

use crate::{board::Board, user_input};

pub fn run(board: &mut Board) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        board.draw();

        let input = Text::new("Enter a cell (example: A5 or F A5):").prompt()?; // `?` operator means to propogate errors. in the main function here, we return an error in the program if the Text::new().prompt() fails
        println!("You selected: {input}");

        let results = user_input::parse_input(&input);
        match results {
            Some(parsed_user_input) => {
                let flag = parsed_user_input.flagged;
                let row = parsed_user_input.row;
                let col = parsed_user_input.col;
                if flag {
                    board.toggle_flag(row, col);
                } else {
                    board.reveal_cell(row, col);
                }
            }
            None => {
                println!("erm, that input looks invalid...");
            }
        }
        if board.is_mine_hit() {
            println!("Awwww better luck next time");
            break;
        }
    }
    Ok(())
}

/*
pub struct Sodoku;
pub struct Scabble;

pub trait Game {
    fn run(&self) -> String;
}

impl Game for Sodoku {
    fn run(&self) -> String {
        "sodoku".to_string()
    }
}

impl Game for Scabble {
    fn run(&self) -> String {
        "scabble".to_string()
    }
}

pub struct GameRunner {
    game: Box<dyn Game>,
}

impl GameRunner {
    pub fn new(game: Box<dyn Game>) -> Self {
        Self { game }
    }

    pub fn run(&self) -> String {
        self.game.run()
    }

    pub fn with_game(self, game: Box<dyn Game>) -> Self {
        Self { game }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_any_game() {
        let game_runner = GameRunner::new(Box::new(Sodoku));
        let output = game_runner.run();
        assert_eq!(output, "sodoku");

        let game_runner = game_runner.with_game(Box::new(Scabble));
        let output = game_runner.run();
        assert_eq!(output, "scabble");
    }
}
*/
