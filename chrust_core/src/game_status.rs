use crate::{ColoredPiece, Piece, Side, Square, errors::ChessError, helper::file_rank, moves::make_move::Move, position::Position};

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

	pub fn is_insufficient_material(&self) -> bool {
		let mut white_knights: u8 = 0;
		let mut black_knights: u8 = 0;
		let mut white_bishop_color: Option<u8> = None;
		let mut black_bishop_color: Option<u8> = None;

		for (i, square) in self.board.iter().enumerate() {
			let Some(colored_piece) = square else {
				continue;
			};

			match colored_piece.piece {
				Piece::King => {}
				Piece::Bishop => {
					let bishop_color = {
						let (file, rank) = file_rank(i as u8);
						(file + rank) % 2
					};
					match colored_piece.side {
						Side::White => {
							if white_bishop_color.is_some() {
								return false;
							}
							white_bishop_color = Some(bishop_color);
						}
						Side::Black => {
							if black_bishop_color.is_some() {
								return false;
							}
							black_bishop_color = Some(bishop_color);
						}
					}
				}
				Piece::Knight => match colored_piece.side {
					Side::White => {
						if white_knights > 0 {
							return false;
						}
						white_knights += 1;
					}
					Side::Black => {
						if black_knights > 0 {
							return false;
						}
						black_knights += 1;
					}
				},
				_ => return false,
			}
		}

		let total_knights = white_knights + black_knights;

		// 2 knights of any side are sufficient
		if total_knights >= 2 {
			return false;
		}

		// Knight + any bishop = sufficient
		if total_knights != 0 && (white_bishop_color.is_some() || black_bishop_color.is_some()) {
			return false;
		}

		// K+B vs K+B: only insufficient if bishops are on the same square color
		if let (Some(wc), Some(bc)) = (white_bishop_color, black_bishop_color) {
			return wc == bc;
		}

		true
	}
}
