use crate::{Side, Square, errors::ChessError, moves::make_move::Move, position::Position};

pub enum GameStatus {
	Playing,
	InCheck,
	Checkmate(Side),
	Stalemate,
	DrawByFiftyMoves,
	DrawByRepetition,
	DrawByInsufficientMaterial,
}

impl Position {
	pub fn is_draw_by_fifty_moves(&self) -> bool {
		self.halfmove_clock >= 100
	}

	pub fn get_all_legal_moves_for_side(&mut self, side: Side) -> Result<Vec<Move>, ChessError> {
		let squares: Vec<Square> = self.board.iter().enumerate().filter_map(|(sq, piece)| piece.filter(|p| p.side == side).map(|_| sq as u8)).collect();

		let mut legal_moves: Vec<Move> = Vec::new();
		for square in squares {
			legal_moves.extend(self.get_legal_moves(square, side)?);
		}

		Ok(legal_moves)
	}

	pub fn is_checkmate_for_side(&mut self, side: Side) -> Result<bool, ChessError> {
		if self.is_king_in_check(side)?.is_none() {
			return Ok(false);
		}

		if !self.get_all_legal_moves_for_side(side)?.is_empty() {
			return Ok(false);
		}

		Ok(true)
	}

	pub fn is_stalemate_for_side(&mut self, side: Side) -> Result<bool, ChessError> {
		if self.is_king_in_check(side)?.is_some() {
			return Ok(false);
		}

		if !self.get_all_legal_moves_for_side(side)?.is_empty() {
			return Ok(false);
		}

		Ok(true)
	}
}
