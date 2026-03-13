use std::u8;

use crate::{ColoredPiece, Piece, Side, Square, errors::ChessError, position::Position};

pub fn file(square: Square) -> u8 {
	square % 8
}

pub fn rank(square: Square) -> u8 {
	square / 8
}

pub fn square(file: u8, rank: u8) -> Square {
	rank * 8 + file
}

pub fn file_rank(square: Square) -> (u8, u8) {
	(file(square), rank(square))
}

impl ColoredPiece {
	pub fn to_char(&self) -> char {
		let mut piece_char = match self.piece {
			Piece::Pawn => 'p',
			Piece::Knight => 'n',
			Piece::Bishop => 'b',
			Piece::Rook => 'r',
			Piece::Queen => 'q',
			Piece::King => 'k',
		};

		if self.side == Side::White {
			piece_char = piece_char.to_ascii_uppercase();
		}

		piece_char
	}
}

/// Checks if a `Square` is in the 64 squares of a chessboard
pub fn is_square_on_board(from_square: Square) -> Result<(), ChessError> {
	if !(0..=63).contains(&from_square) {
		return Err(ChessError::NotASquareOnBoard { square: from_square as i16 });
	} else {
		return Ok(());
	}
}

pub fn in_bounds(candidate: i16) -> bool {
	(0..=63).contains(&candidate)
}

pub fn is_valid_promomotion_piece(promotion_piece: Piece) -> Result<(), ChessError> {
	if promotion_piece == Piece::Pawn {
		return Err(ChessError::PromotionPieceCantBePawn);
	}

	Ok(())
}

pub fn file_diff(candidate: i16, from_square: Square) -> i16 {
	(file(candidate as u8) as i16 - file(from_square) as i16).abs()
}

pub fn rank_diff(candidate: i16, from_square: Square) -> i16 {
	(rank(candidate as u8) as i16 - rank(from_square) as i16).abs()
}

pub fn is_right_piece_type(from_piece: ColoredPiece, expected_piece: Piece) -> Result<(), ChessError> {
	if from_piece.piece != expected_piece {
		return Err(ChessError::WrongPieceType {
			expected_piece,
			found_piece: from_piece.piece,
		});
	}

	Ok(())
}

pub fn is_right_piece_side(from_piece: ColoredPiece, expected_side: Side) -> Result<(), ChessError> {
	if from_piece.side != expected_side {
		return Err(ChessError::WrongSide {
			expected_side,
			found_side: from_piece.side,
		});
	} else {
		Ok(())
	}
}

impl Position {
	/// Gets a colored piece that is validated so that it acutually exists. Also validates the from_square
	pub fn get_validated_colored_piece(&self, from_square: Square, expected_piece: Piece) -> Result<ColoredPiece, ChessError> {
		let col_piece = match self.get_piece_from_square(from_square) {
			Ok(x) => x,
			Err(x) => return Err(x),
		};

		if let Err(x) = self.validate_colored_piece(col_piece, expected_piece) {
			return Err(x);
		}

		Ok(col_piece)
	}

	pub fn validate_colored_piece(&self, colored_piece: ColoredPiece, expected_piece: Piece) -> Result<(), ChessError> {
		if let Err(x) = is_right_piece_side(colored_piece, self.side_to_move) {
			return Err(x);
		}

		if let Err(x) = is_right_piece_type(colored_piece, expected_piece) {
			return Err(x);
		}

		Ok(())
	}

	pub fn get_piece_from_square(&self, from_square: Square) -> Result<ColoredPiece, ChessError> {
		if let Err(x) = is_square_on_board(from_square) {
			return Err(x);
		}

		match self.board[from_square as usize] {
			Some(p) => return Ok(p),
			None => return Err(ChessError::NoPieceOnSquare { square: from_square }),
		};
	}
}

impl Side {
	pub fn opponent(&self) -> Side {
		match self {
			Side::White => Side::Black,
			Side::Black => Side::White,
		}
	}
}
