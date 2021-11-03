use crate::common::*;
use std::collections::HashMap;

pub struct Board {
    cells: [[CellState; 3]; 3],
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
            cells: [[CellState::Empty; 3]; 3],
            marker_count: 0,
            metadata: BoardMetadata::new(),
        }
    }

    pub fn display(&self) {
        let get_cell_char = |&cell_state| match cell_state {
            CellState::X => 'X',
            CellState::O => 'O',
            CellState::Empty => '_',
        };
        println!(
            "\n{} {} {}\n{} {} {}\n{} {} {}\n",
            get_cell_char(&self.cells[0][0]),
            get_cell_char(&self.cells[0][1]),
            get_cell_char(&self.cells[0][2]),
            get_cell_char(&self.cells[1][0]),
            get_cell_char(&self.cells[1][1]),
            get_cell_char(&self.cells[1][2]),
            get_cell_char(&self.cells[2][0]),
            get_cell_char(&self.cells[2][1]),
            get_cell_char(&self.cells[2][2])
        );
    }

    pub fn place_marker(&mut self, cell_coord: CellCoord, marker: Marker) {
        match marker {
            Marker::X => self.cells[cell_coord.row][cell_coord.column] = CellState::X,
            Marker::O => self.cells[cell_coord.row][cell_coord.column] = CellState::O,
        }
        self.marker_count += 1;
        self.update_board_metadata(cell_coord, marker);
    }

    pub fn validate_move(&self, cell_coord: CellCoord) -> Move {
        // We only check the upper bound, because column and row are usize,
        // which is always >= 0.
        if cell_coord.column <= 2 && cell_coord.row <= 2 {
            if let CellState::Empty = self.cells[cell_coord.row][cell_coord.column] {
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
            Marker::X if cell_marker == CellState::X => true,
            Marker::O if cell_marker == CellState::O => true,
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

    fn update_board_metadata(&mut self, last_move: CellCoord, marker: Marker) {
        for cell_flag in self.metadata.cell_flags[last_move.get_index()].iter_mut() {
            match cell_flag {
                CellFlags::WinningMove(Marker::X) if marker == Marker::O => {
                    // O blocked X's winning move
                    // remove cell coord from winning coords
                    println!(
                        "{:?} blocked {:?}'s winning move. Removing {:?} from winning coords",
                        marker,
                        Marker::X,
                        last_move,
                    );
                    if let Some(winning_coords) = self.metadata.winning_coords.get_mut(&Marker::X) {
                        winning_coords.retain(|&cell_coord| {
                            if cell_coord == last_move {
                                return false;
                            }
                            return true;
                        });
                    };
                    *cell_flag = CellFlags::None;
                }
                CellFlags::WinningMove(Marker::O) if marker == Marker::X => {
                    // X blocked O's winning move
                    // remove cell coord from winning coords
                    println!(
                        "{:?} blocked {:?}'s winning move. Removing {:?} from winning coords",
                        marker,
                        Marker::O,
                        last_move,
                    );
                    if let Some(winning_coords) = self.metadata.winning_coords.get_mut(&Marker::O) {
                        winning_coords.retain(|&cell_coord| {
                            if cell_coord == last_move {
                                return false;
                            }
                            return true;
                        });
                    };
                    *cell_flag = CellFlags::None;
                }
                _ => (),
            }
        }

        // clean up cell flags
        self.metadata.cell_flags[last_move.get_index()].retain(|&cell_flag| match cell_flag {
            CellFlags::None => false,
            _ => true,
        });

        // To see if there is a winning move, we count the cell states in the vector of
        // 3 cells. If there are 2 cells with the specified marker, and 1 empty cell, the
        // empty cell is a winning move.
        let find_winning_move =
            |cells: Vec<(CellState, CellCoord)>, marker: Marker| -> Option<CellCoord> {
                let (cell_states, cell_coords): (Vec<_>, Vec<_>) = cells.into_iter().unzip();

                let marker_count = cell_states
                    .iter()
                    .filter(|cell_state| match cell_state {
                        CellState::X if marker == Marker::X => true,
                        CellState::O if marker == Marker::O => true,
                        _ => false,
                    })
                    .count();

                if marker_count == 2 {
                    if let Some(empty_cell_index) =
                        cell_states.iter().position(|&cell_state| match cell_state {
                            CellState::Empty => true,
                            _ => false,
                        })
                    {
                        return Some(cell_coords[empty_cell_index]);
                    }
                };
                None
            };

        let get_move_row = |last_move: CellCoord| -> Vec<(CellState, CellCoord)> {
            self.cells[last_move.row]
                .iter()
                .enumerate()
                .map(|(column_index, &cell_state)| {
                    (cell_state, CellCoord::new(last_move.row, column_index))
                })
                .collect()
        };
        let move_row = get_move_row(last_move);
        if let Some(winning_move) = find_winning_move(move_row, marker) {
            self.metadata.add_winning_coord(winning_move, marker);
        }

        let get_move_column = |last_move: CellCoord| -> Vec<(CellState, CellCoord)> {
            self.cells
                .iter()
                .enumerate()
                .map(|(row_index, row)| {
                    (
                        row[last_move.column],
                        CellCoord::new(row_index, last_move.column),
                    )
                })
                .collect()
        };
        let move_column = get_move_column(last_move);
        if let Some(winning_move) = find_winning_move(move_column, marker) {
            self.metadata.add_winning_coord(winning_move, marker);
        }

        let get_move_diagonal1 = |last_move: CellCoord| -> Option<Vec<(CellState, CellCoord)>> {
            // Only some cells have a 3 cell diagonal.
            // X _ _
            // _ X _
            // _ _ X
            match last_move {
                CellCoord {
                    row: 0, column: 0, ..
                }
                | CellCoord {
                    row: 1, column: 1, ..
                }
                | CellCoord {
                    row: 2, column: 2, ..
                } => Some(vec![
                    (self.cells[0][0], CellCoord::new(0, 0)),
                    (self.cells[1][1], CellCoord::new(1, 1)),
                    (self.cells[2][2], CellCoord::new(2, 2)),
                ]),
                _ => None,
            }
        };
        if let Some(move_diagonal1) = get_move_diagonal1(last_move) {
            if let Some(winning_move) = find_winning_move(move_diagonal1, marker) {
                self.metadata.add_winning_coord(winning_move, marker);
            }
        }

        let get_move_diagonal2 = |last_move: CellCoord| -> Option<Vec<(CellState, CellCoord)>> {
            // Only some cells have a 3 cell diagonal.
            // _ _ X
            // _ X _
            // X _ _
            match last_move {
                CellCoord {
                    row: 2, column: 0, ..
                }
                | CellCoord {
                    row: 1, column: 1, ..
                }
                | CellCoord {
                    row: 0, column: 2, ..
                } => Some(vec![
                    (self.cells[2][0], CellCoord::new(2, 0)),
                    (self.cells[1][1], CellCoord::new(1, 1)),
                    (self.cells[0][2], CellCoord::new(0, 2)),
                ]),
                _ => None,
            }
        };
        if let Some(move_diagonal2) = get_move_diagonal2(last_move) {
            if let Some(winning_move) = find_winning_move(move_diagonal2, marker) {
                self.metadata.add_winning_coord(winning_move, marker);
            }
        }
    }

    pub fn get_winning_move(&self, marker: Marker) -> Option<CellCoord> {
        if let Some(mut vec) = self.metadata.get_winning_coords(marker) {
            vec.pop()
        } else {
            None
        }
    }

    pub fn print_info(&self) {
        self.metadata.print();
    }
}

struct BoardMetadata {
    // winning coords: map of Marker to vector of CellCoord
    winning_coords: HashMap<Marker, Vec<CellCoord>>,
    cell_flags: Vec<Vec<CellFlags>>,
}

#[derive(Copy, Clone)]
enum CellFlags {
    WinningMove(Marker),
    None,
}

impl BoardMetadata {
    fn new() -> BoardMetadata {
        BoardMetadata {
            winning_coords: HashMap::new(),
            cell_flags: vec![
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    fn add_winning_coord(&mut self, winning_coord: CellCoord, marker: Marker) {
        // This may be the first time we access the winning_coords HashMap, so
        // we use `entry().or_insert()`. This allows us to insert a new vector
        // if there isn't one currently at the `marker` key.
        let coords = self.winning_coords.entry(marker).or_insert(Vec::new());
        coords.push(winning_coord);
        //println!("Winning cell {:?} for {:?}", winning_coord, marker);

        self.cell_flags[winning_coord.get_index()].push(CellFlags::WinningMove(marker));
    }

    fn get_winning_coords(&self, marker: Marker) -> Option<Vec<CellCoord>> {
        if let Some(winning_moves) = self.winning_coords.get(&marker) {
            Some(winning_moves.clone())
        } else {
            None
        }
    }

    pub fn print(&self) {
        for i in vec![Marker::X, Marker::O] {
            if let Some(winning_moves) = self.winning_coords.get(&i) {
                if !winning_moves.is_empty() {
                    println!("Winning moves for {:?}:", i);
                    winning_moves.iter().for_each(|cell_coord| {
                        println!("{:?}", cell_coord);
                    })
                }
            }
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
        board.place_marker(CellCoord::new(0, 0), marker);
        board.place_marker(CellCoord::new(0, 1), marker);
        board.place_marker(CellCoord::new(0, 2), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 2), marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_column_win() {
        let marker = Marker::X;
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), marker);
        board.place_marker(CellCoord::new(1, 0), marker);
        board.place_marker(CellCoord::new(2, 0), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(2, 0), marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_row_not_win() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 0), Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(0, 1), Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 1), Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(0, 2), Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 2), Marker::O),
            BoardState::Playing
        );
    }

    #[test]
    fn checks_column_not_win() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 0), Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(1, 0), Marker::X);
        assert_eq!(
            board.check_board_state(CellCoord::new(1, 0), Marker::X),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(2, 0), Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord::new(2, 0), Marker::O),
            BoardState::Playing
        );
    }

    #[test]
    fn checks_diag_win() {
        let marker = Marker::X;
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 0), marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(1, 1), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(1, 1), marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(2, 2), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(2, 2), marker),
            BoardState::Win
        );

        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 2), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(0, 2), marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(1, 1), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(1, 1), marker),
            BoardState::Playing
        );
        board.place_marker(CellCoord::new(2, 0), marker);
        assert_eq!(
            board.check_board_state(CellCoord::new(2, 0), marker),
            BoardState::Win
        );
    }

    #[test]
    fn checks_tie() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(0, 1), Marker::X);
        board.place_marker(CellCoord::new(0, 2), Marker::O);
        board.place_marker(CellCoord::new(1, 0), Marker::O);
        board.place_marker(CellCoord::new(1, 1), Marker::O);
        board.place_marker(CellCoord::new(1, 2), Marker::X);
        board.place_marker(CellCoord::new(2, 0), Marker::X);
        board.place_marker(CellCoord::new(2, 1), Marker::X);
        board.place_marker(CellCoord::new(2, 2), Marker::O);
        assert_eq!(
            board.check_board_state(CellCoord::new(2, 2), Marker::O),
            BoardState::Tie
        );
    }

    #[test]
    fn validates_move_to_used_cell() {
        let marker = Marker::X;
        let mut board = Board::new();
        let move_type = board.validate_move(CellCoord::new(0, 0));
        assert_eq!(move_type, Move::Valid);

        board.place_marker(CellCoord::new(0, 0), marker);
        let move_type = board.validate_move(CellCoord::new(0, 0));
        assert_eq!(move_type, Move::Invalid);
    }

    #[test]
    fn validates_out_of_bounds_move() {
        let board = Board::new();
        let move_type = board.validate_move(CellCoord::new(3, 0));
        assert_eq!(move_type, Move::Invalid);
        let move_type = board.validate_move(CellCoord::new(1, 3));
        assert_eq!(move_type, Move::Invalid);
    }

    #[test]
    fn gets_winning_move_column() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(1, 0), Marker::X);
        let winning_move = board.get_winning_move(Marker::X);
        assert_eq!(winning_move, Some(CellCoord::new(2, 0)));
    }

    #[test]
    fn gets_winning_move_row() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(0, 1), Marker::X);
        let winning_move = board.get_winning_move(Marker::X);
        assert_eq!(winning_move, Some(CellCoord::new(0, 2)));
    }

    #[test]
    fn gets_winning_move_diag() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(1, 1), Marker::X);
        let winning_move = board.get_winning_move(Marker::X);
        assert_eq!(winning_move, Some(CellCoord::new(2, 2)));
    }

    #[test]
    fn gets_winning_move_none() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(2, 1), Marker::X);
        let winning_move = board.get_winning_move(Marker::X);
        assert_eq!(winning_move, None);
    }
}
