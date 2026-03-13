use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::ChessError,
	helper::{file_diff, in_bounds, rank},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	// Without promotion
	pub fn pawn_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(4);

		let pawn = match self.get_validated_colored_piece(from_square, Piece::Pawn) {
			Ok(x) => x,
			Err(x) => return Err(x),
		};

		self.push_moves(&mut target_moves, pawn, from_square);

		let (capture_offsets, last_rank): ([i16; 2], u8) = match pawn.side {
			Side::White => ([7, 9], 7u8),
			Side::Black => ([-7, -9], 0u8),
		};

		for capture_offset in capture_offsets {
			let capture_candidate_i = from_square as i16 + capture_offset;

			if !in_bounds(capture_candidate_i) || file_diff(capture_candidate_i, from_square) != 1 {
				continue;
			}

			let capture_candidate_u = capture_candidate_i as u8;

			// En passant
			if let Some(en_passant_square) = self.en_passant {
				if en_passant_square == capture_candidate_u {
					let captured_square = match pawn.side {
						Side::White => (en_passant_square as i16 - 8) as u8,
						Side::Black => (en_passant_square as i16 + 8) as u8,
					};

					target_moves.push(Move {
						colored_piece: pawn,
						from_square,
						to_square: capture_candidate_u,
						move_kind: MoveKind::EnPassant { capture_square: captured_square },
					});
				}
			}

			if let Some(piece) = self.board[capture_candidate_u as usize] {
				if piece.side == pawn.side {
					continue;
				}

				// Promotion capture
				if rank(capture_candidate_u) == last_rank {
					promotion_moves(&mut target_moves, pawn, from_square, capture_candidate_u);
				} else {
					// Capture
					target_moves.push(Move {
						colored_piece: pawn,
						from_square,
						to_square: capture_candidate_u as u8,
						move_kind: MoveKind::Capture,
					});
				}
			}
		}

		Ok(target_moves)
	}

	pub fn push_moves(&self, target_moves: &mut Vec<Move>, colored_piece: ColoredPiece, from_square: Square) {
		let (push_offset, start_rank, last_rank) = match colored_piece.side {
			Side::White => (8i16, 1u8, 7u8),
			Side::Black => (-8i16, 6u8, 0u8),
		};

		let single_push_candidate_i = from_square as i16 + push_offset;
		if !in_bounds(single_push_candidate_i) {
			return;
		}

		let single_push_candidate_u = single_push_candidate_i as u8;
		if self.board[single_push_candidate_u as usize].is_some() {
			return;
		}

		// Promotion and quiet move
		if rank(single_push_candidate_u) == last_rank {
			promotion_moves(target_moves, colored_piece, from_square, single_push_candidate_u);
		} else {
			target_moves.push(Move {
				colored_piece,
				from_square,
				to_square: single_push_candidate_u as u8,
				move_kind: MoveKind::Quiet,
			});
		}

		// Double push
		if rank(from_square) != start_rank {
			return;
		}

		let double_push_candidate_i = from_square as i16 + (push_offset * 2);
		if !in_bounds(double_push_candidate_i) {
			return;
		}

		let double_push_candidate_u = double_push_candidate_i as u8;
		if self.board[double_push_candidate_u as usize].is_some() {
			return;
		}

		target_moves.push(Move {
			colored_piece,
			from_square,
			to_square: double_push_candidate_u,
			move_kind: MoveKind::DoublePawnPush { passed_square: single_push_candidate_u },
		});
	}
}

pub fn promotion_moves(target_moves: &mut Vec<Move>, colored_piece: ColoredPiece, from_square: Square, to_square: Square) {
	for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
		target_moves.push(Move {
			colored_piece,
			from_square,
			to_square,
			move_kind: MoveKind::Promotion { promotion_piece: piece },
		});
	}
}
