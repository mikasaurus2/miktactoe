pub mod ai_basic;
pub mod ai_forking;
pub mod ai_optimal;
pub mod ai_random;
pub mod human;

use crate::board::Board;
use crate::common::{CellCoord, Marker};

// Traits can specify lifetime parameters just like structs.
// We need one here because we create a concrete Player object
// with a reference to the name string.
pub trait Player<'a> {
    // Inside traits, Self refers to the implementing type.
    // So, the structs that implement this trait can implement
    // a `new()` function that returns their respective concrete type.
    fn new(name: &'a str, marker: Marker) -> Self;
    fn get_valid_move(&mut self, board: &Board) -> CellCoord;
    fn get_marker(&self) -> Marker;
    fn get_name(&self) -> &'a str;
}
