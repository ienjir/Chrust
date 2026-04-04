use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::{ChessError, FenError},
	moves::{
		make_move::{Move, MoveKind},
		move_gen::king::get_file_and_rank_difference,
	},
	position::Game,
};

/// Deprecated - Do not use
pub fn convert_square_to_string(square: u8) -> String {
	let file = (b'a' + (square % 8)) as char;
	let rank = (b'1' + (square / 8)) as char;
	format!("{}{}", file, rank)
}

pub fn convert_string_to_square(square_string: &str) -> Result<Square, FenError> {
	if square_string.len() != 2 {
		return Err(FenError::SquareLenghtIsnt2Wide(square_string.len()));
	}

	let chars: Vec<char> = square_string.to_lowercase().chars().collect();

	let file = (chars[0] as u8).wrapping_sub(b'a');
	if file > 7 {
		return Err(FenError::InvalidFile(chars[0]));
	}

	let rank = chars[1].to_digit(10).map(|d| d as u8).and_then(|d| d.checked_sub(1)).filter(|&d| d < 8).ok_or(FenError::InvalidRank(chars[1]))?;

	let square_index = rank * 8 + file;

	if square_index > 63 {
		return Err(FenError::OutOfBounds(square_index));
	}

	Ok(square_index)
}

pub fn letter_to_piece(piece_char: char) -> Result<Piece, FenError> {
	let piece_type = match piece_char.to_ascii_lowercase() {
		'k' => Piece::King,
		'p' => Piece::Pawn,
		'n' => Piece::Knight,
		'b' => Piece::Bishop,
		'r' => Piece::Rook,
		'q' => Piece::Queen,
		_ => return Err(FenError::InvalidPieceChar(piece_char)),
	};

	Ok(piece_type)
}

impl Piece {
	pub fn to_char(&self) -> char {
		let piece_char = match self {
			Piece::Pawn => 'p',
			Piece::Knight => 'n',
			Piece::Bishop => 'b',
			Piece::Rook => 'r',
			Piece::Queen => 'q',
			Piece::King => 'k',
		};

		piece_char
	}
}

impl Move {
	pub fn to_uci(&self) {
		let mut uci_string = String::new();

		uci_string.push_str(&convert_square_to_string(self.to_square));

		if let MoveKind::Promotion { promotion_piece } = self.move_kind {
			uci_string.push(promotion_piece.to_char());
		}
	}
}

impl Game {
	pub fn convert_uci_to_move(&self, uci_string: &str) -> Result<Move, ChessError> {
		let from_square = convert_string_to_square(&uci_string[..2])?;

		let to_square = convert_string_to_square(&uci_string[2..4])?;

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

impl ColoredPiece {
	pub fn to_char(&self) -> char {
		let mut piece_char = self.piece.to_char();

		if self.side == Side::White {
			piece_char = piece_char.to_ascii_uppercase();
		}

		piece_char
	}
}

#[cfg(test)]
mod tests;
