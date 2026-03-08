use core::fmt;
use crate::{Piece, Side, Square};

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    NotImplemented,
    PromotionPieceCantBeEmpty,
    NotASquareOnBoard { square: Square },
    NoPieceOnSquare { square: Square },
    NotAValidMove,
    KingIsAttacked { squares: Vec<Square> },
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

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
	    ChessError::NotAValidMove => write!(f, "Not a valid move"),
            ChessError::NotImplemented => write!(f, "Not implemented"),
	    ChessError::KingIsAttacked { squares: _ } => write!(f, "King is attacked"),
            ChessError::PromotionPieceCantBeEmpty => write!(f, "Promotion piece can't be empty"),
            ChessError::NotASquareOnBoard { square } => write!(f, "Not a square on board: {square}"),
            ChessError::NoPieceOnSquare { square } => write!(f, "No piece on square: {square}"),
            ChessError::WrongPieceType { expected_piece, found_piece } => {
                write!(f, "Wrong piece type: expected {expected_piece}, found {found_piece}")
            }
            ChessError::WrongSide { expected_side, found_side } => {
                write!(f, "Wrong side: expected {expected_side}, found {found_side}")
            }
        }
    }
}
