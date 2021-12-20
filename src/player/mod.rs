pub mod ai_basic;
pub mod ai_forking;
pub mod ai_optimal;
pub mod ai_random;
pub mod human;

use crate::board::Board;
use crate::common::{CellCoord, Marker};

pub trait Player {
    fn get_valid_move(&mut self, board: &Board) -> CellCoord;
    fn get_marker(&self) -> Marker;
}
