use crate::{Piece, Side, Square};

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
	NotImplemented,
	GameIsFinished,
	NothingToUndo,
	FenError {
		fen_error: FenError,
	},
	InvalidPromotionPiece {
		piece: Piece,
	},
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

#[derive(Debug, PartialEq, Eq)]
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

impl From<FenError> for ChessError {
	fn from(fen_error: FenError) -> Self {
		ChessError::FenError { fen_error }
	}
}
