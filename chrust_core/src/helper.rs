use std::u8;

use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::{ChessError},
	game_status::GameStatus,
	position::{Game, Position},
};

pub(crate) fn file(square: Square) -> u8 {
	square % 8
}

pub(crate) fn rank(square: Square) -> u8 {
	square / 8
}

pub fn square(file: u8, rank: u8) -> Square {
	rank * 8 + file
}

pub fn file_rank(square: Square) -> (u8, u8) {
	(file(square), rank(square))
}

impl Game {
	pub fn is_legal_game_state(&self) -> bool {
		if self.game_status == GameStatus::Playing || self.game_status == GameStatus::InCheck {
			return true;
		}

		false
	}
}

/// Checks if a `Square` is in the 64 squares of a chessboard
pub(crate) fn is_square_on_board(from_square: Square) -> Result<(), ChessError> {
	if !(0..=63).contains(&from_square) {
		return Err(ChessError::NotASquareOnBoard { square: from_square as i16 });
	} else {
		return Ok(());
	}
}

pub(crate) fn in_bounds(candidate: i16) -> bool {
	(0..=63).contains(&candidate)
}

pub(crate) fn is_valid_promomotion_piece(promotion_piece: Piece) -> Result<(), ChessError> {
	if promotion_piece == Piece::Pawn {
		return Err(ChessError::InvalidPromotionPiece { piece: Piece::Pawn });
	}

	if promotion_piece == Piece::King {
		return Err(ChessError::InvalidPromotionPiece { piece: Piece::King });
	}

	Ok(())
}

pub(crate) fn file_diff(candidate: i16, from_square: Square) -> i16 {
	(file(candidate as u8) as i16 - file(from_square) as i16).abs()
}

pub(crate) fn rank_diff(candidate: i16, from_square: Square) -> i16 {
	(rank(candidate as u8) as i16 - rank(from_square) as i16).abs()
}

#[cfg(test)]
pub(crate) fn is_right_piece_type(from_piece: ColoredPiece, expected_piece: Piece) -> Result<(), ChessError> {
	if from_piece.piece != expected_piece {
		return Err(ChessError::WrongPieceType {
			expected_piece,
			found_piece: from_piece.piece,
		});
	}

	Ok(())
}

pub(crate) fn is_right_piece_side(from_piece: ColoredPiece, expected_side: Side) -> Result<(), ChessError> {
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
	#[cfg(test)]
	pub(crate) fn get_validated_colored_piece(&self, from_square: Square, expected_piece: Piece) -> Result<ColoredPiece, ChessError> {
		let col_piece = self.get_piece_from_square(from_square)?;

		if let Err(x) = self.validate_colored_piece(col_piece, expected_piece) {
			return Err(x);
		}

		Ok(col_piece)
	}

	#[cfg(test)]
	pub(crate) fn validate_colored_piece(&self, colored_piece: ColoredPiece, expected_piece: Piece) -> Result<(), ChessError> {
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

#[cfg(test)]
mod tests;
