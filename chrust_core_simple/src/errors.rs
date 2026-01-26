use crate::{Piece, Square};

#[derive(Debug)]
pub enum MoveError {
    NoPieceOnInitalSquare(Square),
    OutOfBounds
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveGenError {
    NotASquareOnBoard {square: Square}, 
    WrongPieceTypeOnSquare {expected_piece: Piece, found_piece: Piece, square: Square},
    NoPieceOnSquare {square: Square},
    NotImplemented,
}
