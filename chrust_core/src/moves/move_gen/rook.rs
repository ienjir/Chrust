use crate::{Piece, Square, errors::ChessError, moves::make_move::Move, position::Position};

impl Position {
	pub fn rook_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(14);

		let rook = match self.get_validated_colored_piece(from_square, Piece::Rook) {
			Ok(x) => x,
			Err(x) => return Err(x),
		};

		self.horizontal_vertical_slider(from_square, rook, &mut target_moves);

		Ok(target_moves)
	}
}
