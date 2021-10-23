use crate::common::*;

pub struct Board {
    cells: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        // The outer bracket represents the row index. The inner bracket represents the column
        // index.
        // ['_'; 3] forms a 1D array: _ _ _ (0 inner index, 1 inner index, 2 inner index)
        //
        // [['_'; 3]; 3] forms a 2D array: [_ _ _] (0 outer index)
        //                                 [_ _ _] (1 outer index)
        //                                 [_ _ _] (2 outer index)
        // When indexing, the leftest bracket will index the outer array, meaning it will index the
        // row. The rightest bracket will index the inner array, meaning it will index the column.
        Board {
            cells: [['_'; 3]; 3],
        }
    }

    pub fn display(&self) {
        println!(
            "\n{} {} {}\n{} {} {}\n{} {} {}\n",
            self.cells[0][0],
            self.cells[0][1],
            self.cells[0][2],
            self.cells[1][0],
            self.cells[1][1],
            self.cells[1][2],
            self.cells[2][0],
            self.cells[2][1],
            self.cells[2][2]
        );
    }

    pub fn place_marker(&mut self, cell_coord: &CellCoord, marker: &Marker) {
        match marker {
            Marker::X => self.cells[cell_coord.row][cell_coord.column] = 'X',
            Marker::O => self.cells[cell_coord.row][cell_coord.column] = 'O',
        }
    }

    pub fn validate_move(&self, cell_coord: &CellCoord) -> Move {
        // We only check the upper bound, because column and row are usize,
        // which is always >= 0.
        if cell_coord.column <= 3 && cell_coord.row <= 3 {
            if let '_' = self.cells[cell_coord.row][cell_coord.column] {
                Move::Valid
            } else {
                println!("Cell already marked. Please try again.");
                Move::Invalid
            }
        } else {
            println!("Out of bounds move. Please try again.");
            Move::Invalid
        }
    }

    // check_win() takes the last move to better check the win condition.
    // Since we check for a win after every move, we only have to check
    // the row, column, and diagnals that correspond to the most recently
    // marked cell.
    pub fn check_win(&self, last_move: &CellCoord, marker: &Marker) -> bool {
        // Checking a row. So we keep the row (y coordinate) consistent.
        // We use an iterator here to iterate each column in the row.
        // We provide a lambda to `all()` to check whether each column
        // matches the last placed marker. This is done by matching on
        // `marker`, and using a match guard to specify an additional
        // condition in a match arm. This means the pattern *and* the
        // match guard must match for the arm to be chosen. We make a lambda
        // for the marker checking function, so we can reuse it for
        // column and diagonal checks.
        let marker_check = |&cell_marker| match marker {
            Marker::X if cell_marker == 'X' => true,
            Marker::O if cell_marker == 'O' => true,
            _ => false,
        };
        let winning_row = self.cells[last_move.row].iter().all(marker_check);
        if winning_row {
            return true;
        }

        // Checking a column. Similar strategy to rows above. However, here
        // we must iterate through each row. While doing that with `iter()`,
        // we use `map()` to transform the iteration from a row to a column
        // index in that row, which gives us the character marker at that
        // coordinate. We use `all()` just like we did above to see if all
        // the characters in the column match the placed marker.
        let winning_column = self
            .cells
            .iter()
            .map(|row| &row[last_move.column])
            .all(marker_check);
        if winning_column {
            return true;
        }

        // Checking top left to bottom right diagonal. Here, we zip together
        // some sequence iterators to form the diagonal coordinates for us to
        // check. We use `map()` to transform the iteration into the cell's
        // value, and use the same `all()` function and lambda as above.
        let winning_diag = (0..3)
            .zip(0..3)
            .map(|(row, column)| &self.cells[row][column])
            .all(marker_check);
        if winning_diag {
            return true;
        }

        // Checking top right to bottom left diagonal.
        let winning_diag = (0..3)
            .rev()
            .zip(0..3)
            .map(|(row, column)| &self.cells[row][column])
            .all(marker_check);
        if winning_diag {
            return true;
        }

        false
    }
}
