use crate::{Piece, Square};

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    NotASquareOnBoard {
        square: Square,
    },
    WrongPieceTypeOnSquare {
        expected_piece: Piece,
        found_piece: Piece,
        square: Square,
    },
    NoPieceOnSquare {
        square: Square,
    },
    PromotionPieceCantBeEmpty,
    NotImplemented,
}

#[derive(Debug)]
pub enum FenError {
    InvalidPieceChar(char),
    SquareLenghtIsnt2Wide(usize),
    OutOfBounds(u8),
    InvalidFile(char),
    InvalidRank(char),
    InvalidCastlingRights(char),
    MissingFenParts,
    NotAValideSide,
}
