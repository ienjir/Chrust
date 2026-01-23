use crate::{Piece, Square};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveGenError {
    NotASquareOnBoard {square: Square}, 
    WrongPieceTypeOnSquare {expected_piece: Piece, found_piece: Piece, square: Square},
    NoPieceOnSquare {square: Square},
}
