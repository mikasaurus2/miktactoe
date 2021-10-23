use crate::common::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, thread, time};

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

pub struct RandomComputer {
    pub name: String,
    pub marker: Marker,
    move_set: Vec<CellCoord>,
    move_index: usize,
}

impl RandomComputer {
    pub fn new(name: String, marker: Marker) -> RandomComputer {
        // To create the RandomComputer's move set, we first use iproduct! macro
        // to make a cartesian product of our row and column ranges. This enumerates
        // all possible cell coordinates. We collect() it to form a vector of these
        // coordinates, and then randomly shuffle it.
        let mut move_set: Vec<CellCoord> = itertools::iproduct!(0..3, 0..3)
            .map(|(row, column)| CellCoord { row, column })
            .collect();

        let mut rng = thread_rng();
        move_set.shuffle(&mut rng);

        RandomComputer {
            name,
            marker,
            move_set,
            move_index: 0,
        }
    }

    pub fn get_move(&mut self) -> CellCoord {
        println!("{}'s turn.", self.name);

        // Use a sleep here so it seems like the computer is thinking a bit.
        thread::sleep(time::Duration::from_secs(1));

        let next_move = self.move_set.get(self.move_index).unwrap_or_else(|| {
            panic!(
                "{} ran out of generated moves. You shouldn't need this many.",
                self.name
            )
        });

        self.move_index += 1;

        // Note that we have to dereference next_move here because the vector's
        // get() method above returns an Option<&CellCoord>. So next_move, if valid,
        // is actually a reference.
        *next_move
    }
}
