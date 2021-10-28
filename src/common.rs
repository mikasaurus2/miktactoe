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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CellCoord {
    pub row: usize,
    pub column: usize,
    // An index associated with the row and column cell. Used for
    // accessing flat arrays.
    index: usize,
}

impl CellCoord {
    pub fn new(row: usize, column: usize) -> CellCoord {
        CellCoord {
            row,
            column,
            index: row * 3 + column,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Valid,
    Invalid,
}
