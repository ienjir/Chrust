use crate::{ Piece, Square, errors::ChessError, moves::make_move::{Move}, position::Position };

impl Position {
    pub fn bishop_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(13);

	let bishop = match self.get_validated_colored_piece(from_square, Piece::Bishop) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	self.diagonal_slider(from_square, bishop, &mut target_moves);

        Ok(target_moves)
    }
}

