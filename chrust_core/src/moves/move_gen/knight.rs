use crate::{
	ColoredPiece, Square,
	errors::ChessError,
	helper::{file_diff, in_bounds, rank_diff},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub(crate) fn knight_targets(&self, knight: ColoredPiece, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(8);

		let directions: [i16; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

		for direction in directions {
			let candidate_square_i = from_square as i16 + direction;

			if !in_bounds(candidate_square_i) {
				continue;
			}

			if !matches!((file_diff(candidate_square_i, from_square), rank_diff(candidate_square_i, from_square)), (2, 1) | (1, 2)) {
				continue;
			}

			match self.board[candidate_square_i as usize] {
				None => {
					target_moves.push(Move {
						colored_piece: knight,
						from_square: from_square,
						to_square: candidate_square_i as u8,
						move_kind: MoveKind::Quiet,
					});
				}
				Some(colored_piece) => {
					if colored_piece.side == knight.side {
						continue;
					}

					target_moves.push(Move {
						colored_piece: knight,
						from_square: from_square,
						to_square: candidate_square_i as u8,
						move_kind: MoveKind::Capture,
					});
				}
			};
		}

		Ok(target_moves)
	}
}

#[cfg(test)]
mod tests;
