use super::Player;
use crate::board::Board;
use crate::common::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};

pub struct ForkingAI<'a> {
    pub name: &'a str,
    pub marker: Marker,
    move_set: Vec<CellCoord>,
}

// ForkingAI will make moves with the following priority:
//   1. make a winning move
//   2. block an opponents winning move
//   3. create a fork if possible
//   3. move randomly
impl<'a> Player<'a> for ForkingAI<'a> {
    fn new(name: &'a str, marker: Marker) -> ForkingAI {
        let mut move_set: Vec<CellCoord> = itertools::iproduct!(0..3, 0..3)
            .map(|(row, column)| CellCoord::new(row, column))
            .collect();

        let mut rng = thread_rng();
        move_set.shuffle(&mut rng);

        ForkingAI {
            name,
            marker,
            move_set,
        }
    }

    fn get_marker(&self) -> Marker {
        self.marker
    }

    fn get_valid_move(&mut self, board: &Board) -> CellCoord {
        //println!("{}'s turn.", self.name);

        // Use a sleep here so it seems like the computer is thinking a bit.
        thread::sleep(time::Duration::from_secs(1));

        // place winning move if there is one
        if let Some(cell_coord) = board.get_winning_move(self.marker) {
            return cell_coord;
        }

        // block opponent's winning move if they have one
        if let Some(cell_coord) = board.get_winning_move(Marker::opposite(self.marker)) {
            return cell_coord;
        }

        // make a fork if possible
        let forking_moves = board.get_forking_move(self.marker);
        if !forking_moves.is_empty() {
            println!("making a fork");
            return forking_moves[0];
        }

        // block opponent's fork
        let opp_forking_moves = board.get_forking_move(Marker::opposite(self.marker));
        if !opp_forking_moves.is_empty() {
            println!("blocking forking move");
            return forking_moves[0];
        }

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
