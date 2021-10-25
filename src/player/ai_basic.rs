use crate::board::Board;
use crate::common::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};

pub struct BasicAI {
    pub name: String,
    pub marker: Marker,
    move_set: Vec<CellCoord>,
}

// BasicAI will make moves with the following priority:
//   1. make a winning move
//   2. block an opponents winning move
//   3. move randomly
impl BasicAI {
    pub fn new(name: String, marker: Marker) -> BasicAI {
        let mut move_set: Vec<CellCoord> = itertools::iproduct!(0..3, 0..3)
            .map(|(row, column)| CellCoord { row, column })
            .collect();

        let mut rng = thread_rng();
        move_set.shuffle(&mut rng);

        BasicAI {
            name,
            marker,
            move_set,
        }
    }

    pub fn get_valid_move(&mut self, board: &Board) -> CellCoord {
        println!("{}'s turn.", self.name);

        // Use a sleep here so it seems like the computer is thinking a bit.
        thread::sleep(time::Duration::from_secs(1));

        loop {
            if let Some(cell_coord) = self.find_winning_move(board, &self.marker) {
                break cell_coord;
            }

            let player_move = self.move_set.pop().unwrap_or_else(|| {
                panic!(
                    "{} ran out of generated moves. You shouldn't need this many.",
                    self.name
                )
            });

            if let Move::Valid = board.validate_move(player_move) {
                break player_move;
            }
        }
    }

    fn find_winning_move(&self, _board: &Board, _marker: &Marker) -> Option<CellCoord> {
        // Use a basic implementation for now. Cycle through all the rows,
        // columns, and diagonals to find any winning moves for the specified
        // marker.
        None
    }
}