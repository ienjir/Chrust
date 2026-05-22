use std::{u8, usize};

use crate::{
	ColoredPiece, Piece, Side, Square,
	attack_tables::KING_TARGETS,
	errors::ChessError,
	helper::{file, rank},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub(crate) fn king_targets(&self, king: ColoredPiece, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(8);

		self.check_castling(&mut target_moves, from_square, king.side)?;

		for attack_square in &KING_TARGETS.get().unwrap()[from_square as usize] {
			match self.board[*attack_square as usize] {
				None => {
					target_moves.push(Move {
						colored_piece: king,
						from_square: from_square,
						to_square: *attack_square,
						move_kind: MoveKind::Quiet,
					});
				}
				Some(colored_piece) => {
					if colored_piece.side != king.side {
						target_moves.push(Move {
							colored_piece: king,
							from_square: from_square,
							to_square: *attack_square,
							move_kind: MoveKind::Capture,
						});
					}
				}
			};
		}

		Ok(target_moves)
	}

	pub fn is_square_safe(&self, square: u8, opponent: Side) -> Result<bool, ChessError> {
		Ok(self.is_square_attacked(square, opponent)?.is_none())
	}

	pub(crate) fn check_castling(&self, target_moves: &mut Vec<Move>, from_square: Square, king_side: Side) -> Result<(), ChessError> {
		if self.is_king_in_check(king_side)?.is_some() {
			return Ok(());
		}

		let (king_side_castling_allowed, queen_side_castling) = match king_side {
			Side::White => (self.castle[0], self.castle[1]),
			Side::Black => (self.castle[2], self.castle[3]),
		};

		if king_side_castling_allowed {
			self.check_castling_queen_or_king_side(target_moves, true, from_square, king_side)?;
		}

		if queen_side_castling {
			self.check_castling_queen_or_king_side(target_moves, false, from_square, king_side)?;
		}

		Ok(())
	}

	pub(crate) fn check_castling_queen_or_king_side(&self, target_moves: &mut Vec<Move>, is_king_half: bool, from_square: Square, king_side: Side) -> Result<(), ChessError> {
		let from_square_i = from_square as i16;
		let (king_to, king_travel, rook_from, rook_to, side_squares): (u8, u8, u8, u8, [u8; 3]) = match is_king_half {
			true => (
				(from_square_i + 2) as u8,
				(from_square_i + 1) as u8,
				(from_square_i + 3) as u8,
				(from_square_i + 1) as u8,
				[
					(from_square_i + 1) as u8,
					(from_square_i + 1) as u8,
					(from_square_i + 2) as u8,
				],
			),
			false => (
				(from_square_i - 2) as u8,
				(from_square_i - 1) as u8,
				(from_square_i - 4) as u8,
				(from_square_i - 1) as u8,
				[
					(from_square_i - 1) as u8,
					(from_square_i - 2) as u8,
					(from_square_i - 3) as u8,
				],
			),
		};

		match self.board[rook_from as usize] {
			Some(piece) if piece.piece == Piece::Rook && piece.side == king_side => {}
			_ => {
				return Ok(());
			}
		}

		if !side_squares.iter().all(|&square| self.board[square as usize].is_none()) {
			return Ok(());
		}

		match self.is_square_safe(king_to, king_side.opponent()) {
			Ok(x) => {
				if !x {
					return Ok(());
				}
			}
			Err(x) => return Err(x),
		}
		match self.is_square_safe(king_travel, king_side.opponent()) {
			Ok(x) => {
				if !x {
					return Ok(());
				}
			}
			Err(x) => return Err(x),
		}

		target_moves.push(Move {
			from_square,
			to_square: king_to,
			move_kind: MoveKind::Castling { rook_from, rook_to },
			colored_piece: ColoredPiece { piece: Piece::King, side: king_side },
		});

		Ok(())
	}
}

pub(crate) fn get_file_and_rank_difference(from_square: Square, substracting_square: Square) -> (i16, i16) {
	let file_difference_i = (file(from_square) as i16 - file(substracting_square) as i16).abs();
	let rank_difference_i = (rank(from_square) as i16 - rank(substracting_square) as i16).abs();
	(file_difference_i, rank_difference_i)
}

#[cfg(test)]
mod tests;
