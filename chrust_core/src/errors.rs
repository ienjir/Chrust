use crate::{Piece, Side, Square};

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    NotImplemented,
    PromotionPieceCantBeEmpty,
    NotASquareOnBoard { square: Square },
    NoPieceOnSquare { square: Square },
    WrongPieceType {
        expected_piece: Piece,
        found_piece: Piece,
    },
    WrongSide {
        expected_side: Side,
        found_side: Side,
    }
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
