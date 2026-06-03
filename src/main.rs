mod board;
mod cell;
mod game;
mod user_input;

use std::process::ExitCode;

use crate::board::Board;

// creates the cell with nothing in it.
// impl is what we use to tell rust what the struct will do.

// Creating the actual board//grid

fn main() -> ExitCode {
    let mut board = Board::new(9, 9, 9);

    if let Err(error) = game::run(&mut board) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

// struct is the list the thing is made of
// impl/fn is the instructions of what the thing does
// Board::new(9, 9, 10) is actually making the thing
