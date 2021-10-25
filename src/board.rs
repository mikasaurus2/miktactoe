use crate::common::*;
use std::collections::HashMap;

pub struct Board {
    cells: [[char; 3]; 3],
    marker_count: u8,
    metadata: BoardMetadata,
}

#[derive(Debug, PartialEq)]
pub enum BoardState {
    Win,
    Tie,
    Playing,
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
            marker_count: 0,
            metadata: BoardMetadata::new(),
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

    pub fn place_marker(&mut self, cell_coord: CellCoord, marker: Marker) {
        match marker {
            Marker::X => self.cells[cell_coord.row][cell_coord.column] = 'X',
            Marker::O => self.cells[cell_coord.row][cell_coord.column] = 'O',
        }
        self.marker_count += 1;
    }

    pub fn validate_move(&self, cell_coord: CellCoord) -> Move {
        // We only check the upper bound, because column and row are usize,
        // which is always >= 0.
        if cell_coord.column <= 2 && cell_coord.row <= 2 {
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

    // check_board_state() takes the last move to better check the win condition.
    // Since we check for a win after every move, we only have to check
    // the row, column, and diagnals that correspond to the most recently
    // marked cell.
    pub fn check_board_state(&self, last_move: CellCoord, marker: Marker) -> BoardState {
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
            return BoardState::Win;
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
            return BoardState::Win;
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
            return BoardState::Win;
        }

        // Checking top right to bottom left diagonal.
        let winning_diag = (0..3)
            .rev()
            .zip(0..3)
            .map(|(row, column)| &self.cells[row][column])
            .all(marker_check);
        if winning_diag {
            return BoardState::Win;
        }

        // No winners this move. Let's check if it's a tie.
        if self.marker_count == 9 {
            return BoardState::Tie;
        }

        BoardState::Playing
    }

    pub fn update_board_metadata(&mut self, last_move: CellCoord, marker: Marker) {
        self.metadata.trigger_cell_reactors(last_move);

        // Update the winnable cells for this marker.
        // row must have 2 of marker and 1 empty
        // column must have 2 of marker and 1 empty
        // diagonal must have 2 of marker and 1 empty

        let mut winning_coord = CellCoord { row: 0, column: 0 };
        let (x_count, empty_count) = self.cells[last_move.row].iter().enumerate().fold(
            (0, 0),
            |mut acc, (column_index, cell_marker)| match cell_marker {
                'X' if marker == Marker::X => {
                    acc.0 += 1;
                    acc
                }
                'O' if marker == Marker::O => {
                    acc.0 += 1;
                    acc
                }
                '_' => {
                    winning_coord.row = last_move.row;
                    winning_coord.column = column_index;
                    acc.1 += 1;
                    acc
                }
                _ => (0, 0),
            },
        );
        println!("row stats: {} {}", x_count, empty_count);
        if x_count == 2 && empty_count == 1 {
            println!("winning move: {:?}", winning_coord);
            self.metadata.add_winning_coord(winning_coord, marker);
        }
    }
}

struct BoardMetadata {
    // winning coords: map of Marker to vector of CellCoord
    winning_coords: HashMap<Marker, Vec<CellCoord>>,
    // 2 d array of vector of lambdas
    cell_reactors: Vec<Vec<Vec<fn(CellCoord)>>>,
}

impl BoardMetadata {
    fn new() -> BoardMetadata {
        BoardMetadata {
            winning_coords: HashMap::new(),
            cell_reactors: vec![vec![Vec::new(); 3]; 3],
        }
    }

    fn add_winning_coord(&mut self, winning_coord: CellCoord, marker: Marker) {
        let coords = self.winning_coords.entry(marker).or_insert(Vec::new());
        coords.push(winning_coord);

        let reactor = |winning_coord| {
            println!("Reacting to marker placement in {:?}!", winning_coord);
        };
        self.cell_reactors[winning_coord.row][winning_coord.column].push(reactor);
    }

    fn trigger_cell_reactors(&mut self, cell_coord: CellCoord) {
        println!("Checking reactors");
        for reactor in self.cell_reactors[cell_coord.row][cell_coord.column].iter() {
            println!("Triggering reactor for {:?}", cell_coord);
            reactor(cell_coord);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note that this test module is an inner module to the board module
    // that is this file. So, to use the Board implementation code, we need
    // to bring that parent module into scope for our test module.
    use super::*;

    #[test]
    fn checks_row_win() {
        let marker = Marker::X;
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, marker);
        board.place_marker(CellCoord { row: 0, column: 1 }, marker);
        board.place_marker(CellCoord { row: 0, column: 2 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 2 }, marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_column_win() {
        let marker = Marker::X;
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, marker);
        board.place_marker(CellCoord { row: 1, column: 0 }, marker);
        board.place_marker(CellCoord { row: 2, column: 0 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 2, column: 0 }, marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_row_not_win() {
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 0 }, Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 0, column: 1 }, Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 1 }, Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 0, column: 2 }, Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 2 }, Marker::O),
            BoardState::Playing
        );
    }

    #[test]
    fn checks_column_not_win() {
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 0 }, Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 1, column: 0 }, Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord { row: 1, column: 0 }, Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 2, column: 0 }, Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord { row: 2, column: 0 }, Marker::O),
            BoardState::Playing
        );
    }

    #[test]
    fn checks_diag_win() {
        let marker = Marker::X;
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 0 }, marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 1, column: 1 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 1, column: 1 }, marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 2, column: 2 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 2, column: 2 }, marker),
            BoardState::Win
        );

        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 2 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 0, column: 2 }, marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 1, column: 1 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 1, column: 1 }, marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord { row: 2, column: 0 }, marker);
        assert_eq!(
            board.check_board_state(CellCoord { row: 2, column: 0 }, marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_tie() {
        let mut board = Board::new();
        board.place_marker(CellCoord { row: 0, column: 0 }, Marker::X);
        board.place_marker(CellCoord { row: 0, column: 1 }, Marker::X);
        board.place_marker(CellCoord { row: 0, column: 2 }, Marker::O);
        board.place_marker(CellCoord { row: 1, column: 0 }, Marker::O);
        board.place_marker(CellCoord { row: 1, column: 1 }, Marker::O);
        board.place_marker(CellCoord { row: 1, column: 2 }, Marker::X);
        board.place_marker(CellCoord { row: 2, column: 0 }, Marker::X);
        board.place_marker(CellCoord { row: 2, column: 1 }, Marker::X);
        board.place_marker(CellCoord { row: 2, column: 2 }, Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord { row: 2, column: 2 }, Marker::O),
            BoardState::Tie
        );
    }

    #[test]
    fn validates_move_to_used_cell() {
        let marker = Marker::X;
        let mut board = Board::new();
        let move_type = board.validate_move(CellCoord { row: 0, column: 0 });
        assert_eq!(move_type, Move::Valid);

        board.place_marker(CellCoord { row: 0, column: 0 }, marker);
        let move_type = board.validate_move(CellCoord { row: 0, column: 0 });
        assert_eq!(move_type, Move::Invalid);
    }

    #[test]
    fn validates_out_of_bounds_move() {
        let board = Board::new();
        let move_type = board.validate_move(CellCoord { row: 3, column: 0 });
        assert_eq!(move_type, Move::Invalid);
        let move_type = board.validate_move(CellCoord { row: 1, column: 3 });
        assert_eq!(move_type, Move::Invalid);
    }
}
