use crate::{Piece, Side, Square, errors::ChessError, helper::letter_to_piece, moves::{make_move::{Move, MoveKind}, move_gen::king::get_file_and_rank_difference}, position::{Game, convert_square_string_to_square}};

impl Game {
	pub fn convert_uci_to_move(&self, uci_string: &str) -> Result<Move, ChessError> {
		let from_square = convert_square_string_to_square(&uci_string[..2])?;

		let to_square = convert_square_string_to_square(&uci_string[2..4])?;

		let colored_piece = self.position.board[from_square as usize].ok_or(ChessError::NoPieceOnSquare { square: from_square })?;

		let mut mv = Move {
			from_square,
			to_square,
			move_kind: MoveKind::Quiet,
			colored_piece,
		};

		if self.position.board[to_square as usize].is_some() {
			mv.move_kind = MoveKind::Capture;
		}

		if uci_string.len() == 5 {
			let promotion_piece = match letter_to_piece(uci_string.chars().last().unwrap()) {
				Ok(x) => Some(x),
				Err(x) => return Err(ChessError::FenError { fen_error: x }),
			};

			mv.move_kind = MoveKind::Promotion {
				promotion_piece: promotion_piece.expect("make_move.rs: make_move_unvalidated: promotion piece is empty"),
			}
		}

		if colored_piece.piece == Piece::Pawn {
			let (_file_diff, rank_diff) = get_file_and_rank_difference(from_square, to_square);

			let direction: i16 = match colored_piece.side {
				Side::White => 8,
				Side::Black => -8,
			};

			if rank_diff == 2 {
				mv.move_kind = MoveKind::DoublePawnPush {
					passed_square: (from_square as i16 + direction) as u8,
				}
			}

			if Some(to_square) == self.position.en_passant {
				mv.move_kind = MoveKind::EnPassant {
					capture_square: (to_square as i16 - direction) as u8,
				}
			}
		}

		if colored_piece.piece == Piece::King {
			let king_from_square: i16 = match self.position.side_to_move {
				Side::White => 4,
				Side::Black => 60,
			};

			let file_difference: i16 = to_square as i16 - from_square as i16;

			if king_from_square == from_square as i16 && file_difference.abs() == 2 {
				let rook_from: Square;
				let rook_to: Square;

				if file_difference.is_negative() {
					rook_from = (king_from_square - 4) as u8;
					rook_to = (king_from_square - 1) as u8;
				} else {
					rook_from = (king_from_square + 3) as u8;
					rook_to = (king_from_square + 1) as u8;
				}

				mv.move_kind = MoveKind::Castling { rook_from: rook_from, rook_to: rook_to }
			}
		}

		Ok(mv)

	}

}
