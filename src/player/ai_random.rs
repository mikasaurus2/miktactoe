use crate::board::Board;
use crate::common::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};


pub struct RandomAI {
    pub name: String,
    pub marker: Marker,
    move_set: Vec<CellCoord>,
}

impl RandomAI {
    pub fn new(name: String, marker: Marker) -> RandomAI {
        // To create the RandomAI's move set, we first use iproduct! macro
        // to make a cartesian product of our row and column ranges. This enumerates
        // all possible cell coordinates. We collect() it to form a vector of these
        // coordinates, and then randomly shuffle it.
        let mut move_set: Vec<CellCoord> = itertools::iproduct!(0..3, 0..3)
            .map(|(row, column)| CellCoord { row, column })
            .collect();

        let mut rng = thread_rng();
        move_set.shuffle(&mut rng);

        RandomAI {
            name,
            marker,
            move_set,
        }
    }

    // The computer should be smart enough to always make valid moves. Initially,
    // we did move validation at the game level, but we can do that here instead
    // by providing a reference to the board as a method parameter. We can then
    // invoke validate_move().
    pub fn get_valid_move(&mut self, board: &Board) -> CellCoord {
        println!("{}'s turn.", self.name);

        // Use a sleep here so it seems like the computer is thinking a bit.
        thread::sleep(time::Duration::from_secs(1));

        loop {
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
}
