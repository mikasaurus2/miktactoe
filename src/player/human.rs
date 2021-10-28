use crate::board::Board;
use crate::common::*;
use std::io;

#[derive(Debug)]
pub struct Human {
    pub name: String,
    pub marker: Marker,
}

impl Human {
    #[allow(dead_code)]
    pub fn get_valid_move(&self, board: &Board) -> CellCoord {
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
            if let Move::Valid = board.validate_move(player_move) {
                return player_move;
            }
        }
    }
}
