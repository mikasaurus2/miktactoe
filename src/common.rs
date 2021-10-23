#[derive(Debug)]
pub enum Marker {
    X,
    O,
}

pub struct CellCoord {
    pub column: usize,
    pub row: usize,
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Valid,
    Invalid,
}

