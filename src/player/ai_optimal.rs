use crate::board::{Board, SetType};
use crate::common::*;
use std::{thread, time};

pub struct OptimalAI {
    pub name: String,
    pub marker: Marker,
}

impl OptimalAI {
    pub fn new(name: String, marker: Marker) -> OptimalAI {
        OptimalAI { name, marker }
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
            println!("blocking a winning move");
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
        if opp_forking_moves.len() == 1 {
            println!("blocking forking move");
            return opp_forking_moves[0];
        }

        // force opponent to defend
        if let Some(cell_coord) = self.force_defending_move(&board, &opp_forking_moves) {
            println!("forcing opponent defend");
            return cell_coord;
        }

        // play center
        if let Move::Valid = board.validate_move(CellCoord::new(1, 1)) {
            println!("playing center");
            return CellCoord::new(1, 1);
        }

        // According to wikipedia, the computer should play the opposite corner here
        // if its opponent is in a corner. I'm not sure what that means though, and
        // the current algorithm seems optimal already. Not implementing for now.

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

        panic!("Unexpected path in OptimalAI::get_valid_move() logic");
    }

    fn force_defending_move(
        &self,
        board: &Board,
        forking_moves: &Vec<CellCoord>,
    ) -> Option<CellCoord> {
        // Try to place 2 in a row to force opp to defend
        // without providing them a forking move.
        let single_marker_sets = board.get_single_marker_sets(self.marker);

        for (coord, set_type) in single_marker_sets {
            //println!("Considering {:?}", set_type);
            let empties: Vec<CellCoord> = self.get_empties(coord, set_type);
            let both_empties_are_forks = empties.iter().all(|&empty| {
                if let Some(_) = forking_moves
                    .iter()
                    .find(|&&forking_move| forking_move == empty)
                {
                    return true;
                } else {
                    return false;
                }
            });

            if both_empties_are_forks {
                //println!("Both empties are forks for {:?}", set_type);
                continue;
            }

            for &empty in empties.iter() {
                if let Some(&coord) = forking_moves.iter().find(|&&coord| coord == empty) {
                    return Some(coord);
                }
            }
            return Some(empties[0]);
        }
        None
    }

    fn get_empties(&self, placed: CellCoord, set_type: SetType) -> Vec<CellCoord> {
        let mut result = Vec::new();
        match set_type {
            SetType::Row(row) => {
                for col in 0..3 {
                    let coord = CellCoord::new(row, col);
                    if placed != coord {
                        result.push(coord);
                    }
                }
            }
            SetType::Column(col) => {
                for row in 0..3 {
                    let coord = CellCoord::new(row, col);
                    if placed != coord {
                        result.push(coord);
                    }
                }
            }
            SetType::Diag1 => {
                for index in 0..3 {
                    let coord = CellCoord::new(index, index);
                    if placed != coord {
                        result.push(coord);
                    }
                }
            }
            SetType::Diag2 => {
                for index in 0..3 {
                    let coord = CellCoord::new(index, 2 - index);
                    if placed != coord {
                        result.push(coord);
                    }
                }
            }
        }
        result
    }
}
