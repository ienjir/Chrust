use core::fmt;

pub mod errors;
pub mod helper;
pub mod moves;
pub mod position;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Side {
	White,
	Black,
}

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

pub enum CastleRigth {
	WK,
	WQ,
	BK,
	BQ,
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Piece::King => write!(f, "King"),
			Piece::Queen => write!(f, "Queen"),
			Piece::Rook => write!(f, "Rook"),
			Piece::Bishop => write!(f, "Bishop"),
			Piece::Knight => write!(f, "Knight"),
			Piece::Pawn => write!(f, "Pawn"),
		}
	}
}

impl fmt::Display for Side {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Side::White => write!(f, "White"),
			Side::Black => write!(f, "Black"),
		}
	}
}
