use crate::board::Board;
use crate::common::*;
use std::{thread, time};

pub struct OptimalAI {
    pub name: String,
    pub marker: Marker,
}

impl OptimalAI {
    pub fn new(name: String, marker: Marker) -> OptimalAI {
        OptimalAI {
            name,
            marker,
        }
    }

    pub fn get_valid_move(&mut self, board: &Board) -> CellCoord {
        println!("{}'s turn.", self.name);

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
        // LEFTOFF:
        // if there's only one fork, block it
        // otherwise, make 2 in a row to force other player to defend
        let forking_moves = board.get_forking_move(Marker::opposite(self.marker));

        if forking_moves.len() == 1 {
            println!("blocking forking move");
            return forking_moves[0];
        }
        if forking_moves.len() > 1 {
            println!("more than one forking move for opponent: placing to make opponent defend");
        }

        // play center
        if let Move::Valid = board.validate_move(CellCoord::new(1, 1)) {
            println!("playing center");
            return CellCoord::new(1, 1);
        }

        // play empty corner
        if let Some(cell_coord) = board.get_corner_move() {
            println!("playing corner");
            return cell_coord;
        }

        // play empty edge
        if let Some(cell_coord) = board.get_edge_move() {
            println!("playing edge");
            return cell_coord;
        }

        // Code should not get here
        CellCoord::new(0, 0)
    }
}
