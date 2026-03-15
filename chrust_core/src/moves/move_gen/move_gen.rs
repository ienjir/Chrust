use crate::{ColoredPiece, Piece, Square, errors::ChessError, moves::make_move::Move, position::Position};

impl Position {
	pub fn get_legal_moves(&mut self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::new();
		let colored_piece = self.get_piece_from_square(from_square)?;
		let pseudo_moves = self.get_pseduo_legal_moves(from_square, colored_piece)?;

		for pseudo_move in pseudo_moves {
			let moving_side = self.side_to_move;
			let undo = self.make_move_unvalidated(pseudo_move)?;
			let in_check = self.is_king_in_check(moving_side)?;
			self.undo_move(undo, pseudo_move)?;
			if in_check.is_none() {
				target_moves.push(pseudo_move);
			}
		}

		Ok(target_moves)
	}

	pub fn get_pseduo_legal_moves(&self, from_square: Square, colored_piece: ColoredPiece) -> Result<Vec<Move>, ChessError> {
		match colored_piece.piece {
			Piece::Rook => self.slider_targets(colored_piece, from_square),
			Piece::Bishop => self.slider_targets(colored_piece, from_square),
			Piece::King => self.king_targets(colored_piece, from_square),
			Piece::Pawn => self.pawn_targets(colored_piece, from_square),
			Piece::Knight => self.knight_targets(colored_piece, from_square),
			Piece::Queen => self.slider_targets(colored_piece, from_square),
		}
	}
}
