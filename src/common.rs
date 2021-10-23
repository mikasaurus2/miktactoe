#[derive(Debug)]
pub enum Marker {
    X,
    O,
}

#[derive(Debug, Copy, Clone)]
pub struct CellCoord {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Valid,
    Invalid,
}

