use crate::{Piece, Side, Square};
use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
	NotImplemented,
	PromotionPieceCantBePawn,
	GameIsFinished,
	NothingToUndo,
	NotASquareOnBoard {
		square: i16,
	},
	NoPieceOnSquare {
		square: Square,
	},
	NotAValidMove,
	KingIsAttacked {
		squares: Vec<Square>,
	},
	WrongPieceType {
		expected_piece: Piece,
		found_piece: Piece,
	},
	WrongSide {
		expected_side: Side,
		found_side: Side,
	},
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
	InvalidNumber(String),
}

impl fmt::Display for ChessError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ChessError::NothingToUndo => write!(f, "Nothing to undo"),
			ChessError::GameIsFinished => write!(f, "Game is finished"),
			ChessError::NotAValidMove => write!(f, "Not a valid move"),
			ChessError::NotImplemented => write!(f, "Not implemented"),
			ChessError::KingIsAttacked { squares: _ } => write!(f, "King is attacked"),
			ChessError::PromotionPieceCantBePawn => write!(f, "Promotion piece can't be pawn"),
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
