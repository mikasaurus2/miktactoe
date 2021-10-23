use std::io;
use crate::common::*;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub marker: Marker,
}

impl Player {
    pub fn get_move(&self) -> CellCoord {
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

        CellCoord { column, row }
    }
}
