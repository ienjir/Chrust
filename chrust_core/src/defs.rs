pub mod nr_of {
    pub const SQUARES = usize = 64;
    pub const SIDES: usize = 2;
}

pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

pub enum Side {
    White = 0,
    Black = 1,
}

pub type Squares = usize; 
