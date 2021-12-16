use crate::common::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

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

type SetIndex = usize;

#[derive(Debug, Copy, Clone)]
pub enum SetType {
    Row(SetIndex),
    Column(SetIndex),
    Diag1,
    Diag2,
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

    fn get_cell_char(&self, cell_state: &CellState) -> char {
        match cell_state {
            CellState::X => 'X',
            CellState::O => 'O',
            CellState::Empty => '_',
        }
    }

    pub fn display(&self) {
        println!(
            "\n{} {} {}\n{} {} {}\n{} {} {}\n",
            self.get_cell_char(&self.cells[0][0]),
            self.get_cell_char(&self.cells[0][1]),
            self.get_cell_char(&self.cells[0][2]),
            self.get_cell_char(&self.cells[1][0]),
            self.get_cell_char(&self.cells[1][1]),
            self.get_cell_char(&self.cells[1][2]),
            self.get_cell_char(&self.cells[2][0]),
            self.get_cell_char(&self.cells[2][1]),
            self.get_cell_char(&self.cells[2][2])
        );
    }

    pub fn get_cellstate_char(&self, index: usize) -> char {
        let row = index / 3;
        let col = index % 3;
        self.get_cell_char(&self.cells[row][col])
    }

    pub fn place_marker(&mut self, cell_coord: CellCoord, marker: Marker) {
        match marker {
            Marker::X => self.cells[cell_coord.row][cell_coord.column] = CellState::X,
            Marker::O => self.cells[cell_coord.row][cell_coord.column] = CellState::O,
        }
        self.marker_count += 1;
        self.update_board_metadata(cell_coord);
    }

    pub fn validate_move(&self, cell_coord: CellCoord) -> Move {
        // We only check the upper bound, because column and row are usize,
        // which is always >= 0.
        if cell_coord.column <= 2 && cell_coord.row <= 2 {
            if let CellState::Empty = self.cells[cell_coord.row][cell_coord.column] {
                Move::Valid
            } else {
                Move::AlreadyUsed
            }
        } else {
            Move::OutOfBounds
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

    fn update_board_metadata(&mut self, last_move: CellCoord) {
        match last_move.get_cell_type() {
            CellType::Corner => self.metadata.remove_corner_move(last_move),
            CellType::Edge => self.metadata.remove_edge_move(last_move),
            _ => (),
        }

        self.metadata.reset();

        let get_row =
            |row_index: usize, cells: &[[CellState; 3]; 3]| -> Vec<(CellState, CellCoord)> {
                cells[row_index]
                    .iter()
                    .enumerate()
                    .map(|(column_index, &cell_state)| {
                        (cell_state, CellCoord::new(row_index, column_index))
                    })
                    .collect()
            };

        for row_index in 0..3 {
            let row = get_row(row_index, &self.cells);
            self.scan_set(row, SetType::Row(row_index));
        }

        let get_col =
            |col_index: usize, cells: &[[CellState; 3]; 3]| -> Vec<(CellState, CellCoord)> {
                cells
                    .iter()
                    .enumerate()
                    .map(|(row_index, row)| (row[col_index], CellCoord::new(row_index, col_index)))
                    .collect()
            };

        for col_index in 0..3 {
            let col = get_col(col_index, &self.cells);
            self.scan_set(col, SetType::Column(col_index));
        }

        let get_diag1 = |cells: &[[CellState; 3]; 3]| -> Vec<(CellState, CellCoord)> {
            vec![
                (cells[0][0], CellCoord::new(0, 0)),
                (cells[1][1], CellCoord::new(1, 1)),
                (cells[2][2], CellCoord::new(2, 2)),
            ]
        };
        let diag1 = get_diag1(&self.cells);
        self.scan_set(diag1, SetType::Diag1);

        let get_diag2 = |cells: &[[CellState; 3]; 3]| -> Vec<(CellState, CellCoord)> {
            vec![
                (cells[0][2], CellCoord::new(0, 2)),
                (cells[1][1], CellCoord::new(1, 1)),
                (cells[2][0], CellCoord::new(2, 0)),
            ]
        };
        let diag2 = get_diag2(&self.cells);
        self.scan_set(diag2, SetType::Diag2);
    }

    fn scan_set(&mut self, set: Vec<(CellState, CellCoord)>, set_type: SetType) {
        let (cell_states, cell_coords): (Vec<_>, Vec<_>) = set.into_iter().unzip();

        let mut x_count: u8 = 0;
        let mut x_coord: CellCoord = CellCoord::new(0, 0);
        let mut o_count: u8 = 0;
        let mut o_coord: CellCoord = CellCoord::new(0, 0);
        let mut empty: u8 = 0;
        let mut empty_coord: CellCoord = CellCoord::new(0, 0);

        for (index, cell) in cell_states.iter().enumerate() {
            match cell {
                CellState::X => {
                    x_coord = cell_coords[index];
                    x_count += 1;
                }
                CellState::O => {
                    o_coord = cell_coords[index];
                    o_count += 1;
                }
                CellState::Empty => {
                    empty += 1;
                    empty_coord = cell_coords[index];
                }
            }
        }

        match (x_count, o_count, empty) {
            (2, 0, 1) => {
                // winning move for X
                self.metadata.add_winning_coord(empty_coord, Marker::X);
            }
            (0, 2, 1) => {
                // winning move for O
                self.metadata.add_winning_coord(empty_coord, Marker::O);
            }
            (1, 0, 2) => {
                // This set is a fork potential for X.
                match set_type {
                    SetType::Row(index) => {
                        self.metadata
                            .add_potential_fork(x_coord, SetType::Row(index), Marker::X)
                    }
                    SetType::Column(index) => {
                        self.metadata
                            .add_potential_fork(x_coord, SetType::Column(index), Marker::X)
                    }
                    SetType::Diag1 => {
                        self.metadata
                            .add_potential_fork(x_coord, SetType::Diag1, Marker::X)
                    }
                    SetType::Diag2 => {
                        self.metadata
                            .add_potential_fork(x_coord, SetType::Diag2, Marker::X)
                    }
                }
            }
            (0, 1, 2) => {
                // This set is a fork potential for O.
                match set_type {
                    SetType::Row(index) => {
                        self.metadata
                            .add_potential_fork(o_coord, SetType::Row(index), Marker::O)
                    }
                    SetType::Column(index) => {
                        self.metadata
                            .add_potential_fork(o_coord, SetType::Column(index), Marker::O)
                    }
                    SetType::Diag1 => {
                        self.metadata
                            .add_potential_fork(o_coord, SetType::Diag1, Marker::O)
                    }
                    SetType::Diag2 => {
                        self.metadata
                            .add_potential_fork(o_coord, SetType::Diag2, Marker::O)
                    }
                }
            }
            _ => (),
        }
    }

    pub fn get_winning_move(&self, marker: Marker) -> Option<CellCoord> {
        if let Some(mut vec) = self.metadata.get_winning_coords(marker) {
            vec.pop()
        } else {
            None
        }
    }

    pub fn get_forking_move(&self, marker: Marker) -> Vec<CellCoord> {
        self.metadata.get_fork_coords(marker).into_iter().collect()
    }

    pub fn get_single_marker_sets(&self, marker: Marker) -> Vec<(CellCoord, SetType)> {
        self.metadata.get_potential_forks(marker)
    }

    pub fn get_corner_move(&self) -> Option<CellCoord> {
        self.metadata.get_corner_coords()
    }

    pub fn get_edge_move(&self) -> Option<CellCoord> {
        self.metadata.get_edge_coords()
    }

    pub fn print_info(&self) {
        self.metadata.print();
    }
}

struct BoardMetadata {
    winning_moves: HashMap<Marker, Vec<CellCoord>>,
    corner_moves: Vec<CellCoord>,
    edge_moves: Vec<CellCoord>,
    x_potential_forks: Vec<(CellCoord, SetType)>,
    o_potential_forks: Vec<(CellCoord, SetType)>,
}

impl BoardMetadata {
    fn new() -> BoardMetadata {
        let corner_moves = vec![
            CellCoord::new(0, 0),
            CellCoord::new(0, 2),
            CellCoord::new(2, 2),
            CellCoord::new(2, 0),
        ];
        let edge_moves = vec![
            CellCoord::new(0, 1),
            CellCoord::new(1, 2),
            CellCoord::new(2, 1),
            CellCoord::new(1, 0),
        ];
        BoardMetadata {
            winning_moves: HashMap::new(),
            corner_moves,
            edge_moves,
            x_potential_forks: Vec::new(),
            o_potential_forks: Vec::new(),
        }
    }

    fn add_winning_coord(&mut self, winning_coord: CellCoord, marker: Marker) {
        // This may be the first time we access the winning_moves HashMap, so
        // we use `entry().or_insert()`. This allows us to insert a new vector
        // if there isn't one currently at the `marker` key.
        let coords = self.winning_moves.entry(marker).or_insert(Vec::new());
        coords.push(winning_coord);
    }

    fn get_winning_coords(&self, marker: Marker) -> Option<Vec<CellCoord>> {
        if let Some(winning_moves) = self.winning_moves.get(&marker) {
            Some(winning_moves.clone())
        } else {
            None
        }
    }

    fn add_potential_fork(&mut self, coord: CellCoord, set_type: SetType, marker: Marker) {
        match marker {
            Marker::X => self.x_potential_forks.push((coord, set_type)),
            Marker::O => self.o_potential_forks.push((coord, set_type)),
        }
    }

    fn get_fork_coords(&self, marker: Marker) -> HashSet<CellCoord> {
        // Fork coordinates are the intersection of two sets (row, col, diag) that have
        // fork potential.
        let get_intersection = |set1: SetType, set2: SetType| -> Option<CellCoord> {
            match (set1, set2) {
                // Hmm. We have to account for any order of the sets. This is awkward.
                // I should think of a better way.
                (SetType::Row(row_index), SetType::Column(col_index)) => {
                    Some(CellCoord::new(row_index, col_index))
                }
                (SetType::Column(col_index), SetType::Row(row_index)) => {
                    Some(CellCoord::new(row_index, col_index))
                }
                (SetType::Row(row_index), SetType::Diag1) => {
                    Some(CellCoord::new(row_index, row_index))
                }
                (SetType::Diag1, SetType::Row(row_index)) => {
                    Some(CellCoord::new(row_index, row_index))
                }
                (SetType::Row(row_index), SetType::Diag2) => {
                    Some(CellCoord::new(row_index, 2 - row_index))
                }
                (SetType::Diag2, SetType::Row(row_index)) => {
                    Some(CellCoord::new(row_index, 2 - row_index))
                }
                (SetType::Column(col_index), SetType::Diag1) => {
                    Some(CellCoord::new(col_index, col_index))
                }
                (SetType::Diag1, SetType::Column(col_index)) => {
                    Some(CellCoord::new(col_index, col_index))
                }
                (SetType::Column(col_index), SetType::Diag2) => {
                    Some(CellCoord::new(2 - col_index, col_index))
                }
                (SetType::Diag2, SetType::Column(col_index)) => {
                    Some(CellCoord::new(2 - col_index, col_index))
                }
                _ => None,
            }
        };

        let mut result = HashSet::new();
        match marker {
            Marker::X => {
                for entry1 in &self.x_potential_forks {
                    for entry2 in &self.x_potential_forks {
                        if entry1.0 != entry2.0 {
                            if let Some(coord) = get_intersection(entry1.1, entry2.1) {
                                result.insert(coord);
                            }
                        }
                    }
                }
            }
            Marker::O => {
                for entry1 in &self.o_potential_forks {
                    for entry2 in &self.o_potential_forks {
                        if entry1.0 != entry2.0 {
                            if let Some(coord) = get_intersection(entry1.1, entry2.1) {
                                result.insert(coord);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn get_potential_forks(&self, marker: Marker) -> Vec<(CellCoord, SetType)> {
        match marker {
            Marker::X => self.x_potential_forks.clone(),
            Marker::O => self.o_potential_forks.clone(),
        }
    }

    fn get_corner_coords(&self) -> Option<CellCoord> {
        self.corner_moves.choose(&mut rand::thread_rng()).copied()
    }

    fn remove_corner_move(&mut self, coord: CellCoord) {
        self.corner_moves.retain(|&cell_coord| cell_coord != coord);
    }

    fn get_edge_coords(&self) -> Option<CellCoord> {
        self.edge_moves.choose(&mut rand::thread_rng()).copied()
    }

    fn remove_edge_move(&mut self, coord: CellCoord) {
        self.edge_moves.retain(|&cell_coord| cell_coord != coord);
    }

    pub fn reset(&mut self) {
        self.winning_moves.clear();
        self.x_potential_forks.clear();
        self.o_potential_forks.clear();
    }

    pub fn print(&self) {
        for i in vec![Marker::X, Marker::O] {
            if let Some(winning_moves) = self.winning_moves.get(&i) {
                if !winning_moves.is_empty() {
                    println!("Winning moves for {:?}:", i);
                    winning_moves.iter().for_each(|cell_coord| {
                        println!("{:?}", cell_coord);
                    })
                }
            }
        }

        println!("Corner moves:");
        self.corner_moves.iter().for_each(|cell_coord| {
            println!("  {:?}", cell_coord);
        });
        println!("Edge moves:");
        self.edge_moves.iter().for_each(|cell_coord| {
            println!("  {:?}", cell_coord);
        });
        println!("X Potential Forks:");
        self.x_potential_forks.iter().for_each(|set_type| {
            println!("  {:?}", set_type);
        });
        println!("O Potential Forks:");
        self.o_potential_forks.iter().for_each(|set_type| {
            println!("  {:?}", set_type);
        });
        println!("X forking moves:");
        println!("{:?}", self.get_fork_coords(Marker::X));
        println!("O forking moves:");
        println!("{:?}", self.get_fork_coords(Marker::O));
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
        assert_eq!(move_type, Move::AlreadyUsed);
    }

    #[test]
    fn validates_out_of_bounds_move() {
        let board = Board::new();
        let move_type = board.validate_move(CellCoord::new(3, 0));
        assert_eq!(move_type, Move::OutOfBounds);
        let move_type = board.validate_move(CellCoord::new(1, 3));
        assert_eq!(move_type, Move::OutOfBounds);
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

    #[test]
    fn gets_forking_move() {
        let mut board = Board::new();
        board.place_marker(CellCoord::new(0, 0), Marker::X);
        board.place_marker(CellCoord::new(2, 2), Marker::X);
        let mut forking_moves = board.get_forking_move(Marker::X);
        forking_moves.sort();
        assert_eq!(forking_moves.len(), 2);
        assert_eq!(forking_moves[0], CellCoord::new(0, 2));
        assert_eq!(forking_moves[1], CellCoord::new(2, 0));
    }

    #[test]
    fn gets_corner_moves() {
        let mut board = Board::new();
        let mut corners = Vec::new();
        while let Some(coord) = board.get_corner_move() {
            corners.push(coord);
            board.place_marker(coord, Marker::X);
        }
        corners.sort();
        assert_eq!(corners.len(), 4);
        assert_eq!(
            corners,
            vec![
                CellCoord::new(0, 0),
                CellCoord::new(0, 2),
                CellCoord::new(2, 0),
                CellCoord::new(2, 2)
            ]
        );
    }

    #[test]
    fn gets_edge_moves() {
        let mut board = Board::new();
        let mut edges = Vec::new();
        while let Some(coord) = board.get_edge_move() {
            edges.push(coord);
            board.place_marker(coord, Marker::X);
        }
        edges.sort();
        assert_eq!(edges.len(), 4);
        assert_eq!(
            edges,
            vec![
                CellCoord::new(0, 1),
                CellCoord::new(1, 0),
                CellCoord::new(1, 2),
                CellCoord::new(2, 1)
            ]
        );
    }
}
