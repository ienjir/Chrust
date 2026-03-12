use crate::{
	ColoredPiece, Piece, Square,
	errors::ChessError,
	helper::{file, is_right_piece_side, rank},
	moves::make_move::{Move, MoveKind},
	position::Position,
};

impl Position {
	pub fn slider_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
		let mut target_moves: Vec<Move> = Vec::with_capacity(27);
		let colored_piece = self.get_piece_from_square(from_square)?;

		is_right_piece_side(colored_piece, self.side_to_move)?;

		match colored_piece.piece {
			Piece::Queen => {
				self.diagonal_slider(from_square, colored_piece, &mut target_moves);
				self.horizontal_vertical_slider(from_square, colored_piece, &mut target_moves);
			}
			Piece::Rook => {
				self.horizontal_vertical_slider(from_square, colored_piece, &mut target_moves);
			}
			Piece::Bishop => {
				self.diagonal_slider(from_square, colored_piece, &mut target_moves);
			}
			_ => {
				return Err(ChessError::WrongPieceType {
					expected_piece: Piece::Queen,
					found_piece: colored_piece.piece,
				});
			}
		};

		Ok(target_moves)
	}

	pub fn diagonal_slider(&self, from_square: Square, colored_piece: ColoredPiece, target_moves: &mut Vec<Move>) {
		let directions: [i16; 4] = [-7, 7, -9, 9];

		self.slider(from_square, colored_piece, target_moves, directions);
	}

	pub fn horizontal_vertical_slider(&self, from_square: Square, colored_piece: ColoredPiece, target_moves: &mut Vec<Move>) {
		let directions: [i16; 4] = [-8, 8, -1, 1];

		self.slider(from_square, colored_piece, target_moves, directions);
	}

	fn slider(&self, from_square: Square, colored_piece: ColoredPiece, target_moves: &mut Vec<Move>, directions: [i16; 4]) {
		for direction in directions {
			self.slide_ray(from_square, direction, |to_square, occupant| {
				match occupant {
					None => {
						target_moves.push(Move {
							colored_piece,
							from_square,
							to_square,
							move_kind: MoveKind::Quiet,
						});
						true // continue
					}
					Some(occ) => {
						if colored_piece.side != occ.side {
							target_moves.push(Move {
								colored_piece,
								from_square,
								to_square,
								move_kind: MoveKind::Capture,
							});
						}
						false // stop
					}
				}
			});
		}
	}

	pub fn slide_ray(&self, from_square: Square, direction: i16, mut on_square: impl FnMut(u8, Option<ColoredPiece>) -> bool) {
		let mut step_from_i = from_square as i16;

		loop {
			let step_to_i = step_from_i + direction;

			if !(0..=63).contains(&step_to_i) {
				break;
			}

			let step_from_u = step_from_i as u8;
			let step_to_u = step_to_i as u8;

			let file_diff = (file(step_to_u) as i16 - file(step_from_u) as i16).abs();
			let rank_diff = (rank(step_to_u) as i16 - rank(step_from_u) as i16).abs();
			let is_rook_ray = direction.abs() == 8 || direction.abs() == 1;
			let is_bishop_ray = direction.abs() == 7 || direction.abs() == 9;
			if is_rook_ray {
				if !((direction.abs() == 8 && file_diff == 0 && rank_diff == 1) || (direction.abs() == 1 && file_diff == 1 && rank_diff == 0)) {
					break;
				}
			} else if is_bishop_ray {
				if !(file_diff == 1 && rank_diff == 1) {
					break;
				}
			}

			let occupant = self.board[step_to_u as usize];
			if !on_square(step_to_u, occupant) {
				break;
			}

			step_from_i = step_to_i;
		}
	}
}
