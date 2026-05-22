use crate::{
	ColoredPiece, Square,
	attack_tables::KNIGHT_TARGETS,
	errors::ChessError,
	helper::{file_diff, in_bounds, rank_diff},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub(crate) fn knight_targets(&self, knight: ColoredPiece, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(8);

		for attack_square in &KNIGHT_TARGETS.get().unwrap()[from_square as usize] {
			match self.board[*attack_square as usize] {
				None => {
					target_moves.push(Move {
						colored_piece: knight,
						from_square: from_square,
						to_square: *attack_square,
						move_kind: MoveKind::Quiet,
					});
				}
				Some(colored_piece) => {
					if colored_piece.side != knight.side {
						target_moves.push(Move {
							colored_piece: knight,
							from_square: from_square,
							to_square: *attack_square,
							move_kind: MoveKind::Capture,
						});
					}
				}
			}
		}

		Ok(target_moves)
	}
}

#[cfg(test)]
mod tests;
