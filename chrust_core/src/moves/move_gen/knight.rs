use crate::{
	Piece, Square,
	errors::ChessError,
	helper::{file, file_rank, rank},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub fn knight_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(8);

		let knight = match self.get_validated_colored_piece(from_square, Piece::Knight) {
			Ok(x) => x,
			Err(x) => return Err(x),
		};

		let (from_file_i, from_rank_i) = file_rank(from_square);
		let from_file_i = from_file_i as i16;
		let from_rank_i = from_rank_i as i16;

		let directions: [i16; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

		for direction in directions {
			let candidate_square_i = from_square as i16 + direction;

			if !(0..=63).contains(&candidate_square_i) {
				continue;
			}

			let file_difference_i = (file(candidate_square_i as u8) as i16 - from_file_i).abs();
			let rank_difference_i = (rank(candidate_square_i as u8) as i16 - from_rank_i).abs();

			let is_allowed_jump = (file_difference_i == 2 && rank_difference_i == 1) || (file_difference_i == 1 && rank_difference_i == 2);

			if !is_allowed_jump {
				continue;
			}

			let candidate_occupant = self.board[candidate_square_i as usize];
			match candidate_occupant {
				None => {
					target_moves.push(Move {
						colored_piece: knight,
						from_square: from_square,
						to_square: candidate_square_i as u8,
						move_kind: MoveKind::Quiet,
					});
				}
				Some(colored_piece) => {
					if colored_piece.side != knight.side {
						target_moves.push(Move {
							colored_piece: knight,
							from_square: from_square,
							to_square: candidate_square_i as u8,
							move_kind: MoveKind::Capture,
						});
					}
					continue;
				}
			};
		}

		Ok(target_moves)
	}
}
