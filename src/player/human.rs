use crate::board::Board;
use crate::common::*;
use super::Player;
use std::io;

#[derive(Debug)]
pub struct Human<'a> {
    pub name: &'a str,
    pub marker: Marker,
}

impl<'a> Player<'a> for Human<'a> {
    fn new(name: &'a str, marker: Marker) -> Human<'a> {
        Human { name, marker }
    }

    fn get_marker(&self) -> Marker {
        self.marker
    }

    fn get_valid_move(&mut self, board: &Board) -> CellCoord {
        loop {
            let mut input = String::new();
            println!("{}'s turn.", self.name);
            println!("column index (left to right)");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let column: usize = input.trim().parse().expect("Please type a number!");

            let mut input = String::new();
            println!("row index (top to bottom)");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let row: usize = input.trim().parse().expect("Please type a number!");

            let player_move = CellCoord::new(row, column);
            match board.validate_move(player_move) {
                Move::Valid => break player_move,
                Move::AlreadyUsed => {
                    println!("Cell already marked. Please try again.");
                    continue;
                }
                Move::OutOfBounds => {
                    println!("Out of bounds move. Please try again.");
                    continue;
                }
            }
        }
    }
}
