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
		if self.halfmove_clock <= 100 {
			return true;
		}

		false
	}

	pub fn get_all_legal_moves(&mut self) -> Result<Vec<Move>, ChessError> {
		let squares: Vec<Square> = self.board.iter().enumerate().
			filter_map(|(sq, piece)| {
				piece.filter(|p| p.side == self.side_to_move).map(|_| sq as u8)
			}).collect();

		let mut legal_moves: Vec<Move> = Vec::new();	
		for square in squares {
			legal_moves.extend(self.get_legal_moves(square)?);
		}

		Ok(legal_moves)
	}
}
