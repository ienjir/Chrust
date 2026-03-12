use std::{u8, usize};

use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::ChessError,
	helper::{file, rank},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub fn king_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(8);

		let king = self.get_validated_colored_piece(from_square, Piece::King)?;

		self.check_castling(&mut target_moves, from_square, king.side)?;

		let directions: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

		for direction in directions {
			let candidate_square_u = match get_validated_candidate_square(from_square, direction) {
				Ok(x) => x,
				Err(_x) => continue,
			};

			let (file_difference_i, rank_difference_i) = get_file_and_rank_difference(from_square, candidate_square_u);

			if !(file_difference_i <= 1 && rank_difference_i <= 1) {
				continue;
			}

			let candidate_occupant = self.board[candidate_square_u as usize];
			match candidate_occupant {
				None => {
					target_moves.push(Move {
						colored_piece: king,
						from_square: from_square,
						to_square: candidate_square_u,
						move_kind: MoveKind::Quiet,
					});
				}
				Some(colored_piece) => {
					if colored_piece.side != king.side {
						target_moves.push(Move {
							colored_piece: king,
							from_square: from_square,
							to_square: candidate_square_u,
							move_kind: MoveKind::Capture,
						});
					}

					continue;
				}
			};
		}

		Ok(target_moves)
	}

	fn is_square_safe(&self, square: u8, opponent: Side) -> Result<bool, ChessError> {
		Ok(self.is_square_attacked(square, opponent)?.is_none())
	}

	pub fn check_castling(&self, target_moves: &mut Vec<Move>, from_square: Square, king_side: Side) -> Result<(), ChessError> {
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

	pub fn check_castling_queen_or_king_side(&self, target_moves: &mut Vec<Move>, is_king_half: bool, from_square: Square, king_side: Side) -> Result<(), ChessError> {
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
		if !self.is_square_safe(king_to, king_side.opponent())? {
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

pub fn get_validated_candidate_square(from_square: Square, direction: i16) -> Result<Square, ChessError> {
	let candidate_square_i = from_square as i16 + direction;

	if !(0..=63).contains(&candidate_square_i) {
		return Err(ChessError::NotASquareOnBoard { square: candidate_square_i });
	}

	Ok(candidate_square_i as u8)
}

pub fn get_file_and_rank_difference(from_square: Square, substracting_square: Square) -> (i16, i16) {
	let file_difference_i = (file(from_square) as i16 - file(substracting_square) as i16).abs();
	let rank_difference_i = (rank(from_square) as i16 - rank(substracting_square) as i16).abs();
	(file_difference_i, rank_difference_i)
}
