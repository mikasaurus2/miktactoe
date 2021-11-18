#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum Marker {
    X,
    O,
}

impl Marker {
    pub fn opposite(marker: Marker) -> Marker {
        match marker {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CellState {
    X,
    O,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CellType {
    Corner,
    Edge,
    Center
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub row: usize,
    pub column: usize,
    // An index associated with the row and column cell. Used for
    // accessing flat arrays.
    index: usize,
    cell_type: CellType,
}

impl CellCoord {
    pub fn new(row: usize, column: usize) -> CellCoord {
        let cell_type = match (row, column) {
            (0, 0) | (0, 2) | (2, 2) | (2, 0) => CellType::Corner,
            (0, 1) | (1, 2) | (2, 1) | (1, 0) => CellType::Edge,
            _ => CellType::Center,
        };

        CellCoord {
            row,
            column,
            index: row * 3 + column,
            cell_type,
        }
    }

    pub fn get_cell_type(&self) -> CellType {
        self.cell_type
    }
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Valid,
    AlreadyUsed,
    OutOfBounds,
}
