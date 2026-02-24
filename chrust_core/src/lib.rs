pub mod moves;
pub mod position;
pub mod errors;
pub mod helper;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Side { White, Black }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColoredPiece {
    pub piece: Piece,
    pub side: Side,
}

pub type Square = u8;

pub enum CastleRigth { WK, WQ, BK, BQ }
