use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::{ChessError, FenError},
	game_status::GameStatus,
	helper::square,
	moves::make_move::Move,
};

pub struct Game {
	pub position: Position,
	pub hash_history: Vec<u64>,
	pub move_history: Vec<Move>,
	pub undo_history: Vec<Undo>,
	pub game_status: GameStatus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
	pub board: [Option<ColoredPiece>; 64],
	pub side_to_move: Side,
	pub(crate) castle: [bool; 4],
	pub(crate) en_passant: Option<Square>,
	pub(crate) king_squares: [Square; 2],
	pub(crate) halfmove_clock: u32,
	pub(crate) fullmove_counter: u32,
	pub(crate) zobrist_hash: u64,
}

#[derive(Copy, Clone)]
pub struct Undo {
	pub(crate) captured_piece: Option<ColoredPiece>,
	pub(crate) previous_castling_rights: [bool; 4],
	pub(crate) previous_en_passant: Option<Square>,
	pub(crate) previous_halfway_clock: u32,
	pub(crate) fullmove_number: u32,
	pub(crate) previous_king_squares: [Square; 2],
}

impl Game {
	pub fn try_from_fen(fen_string: &str) -> Result<Game, ChessError> {
		let position = load_position_from_fen(fen_string)?;
		let mut game = Game {
			position,
			hash_history: Vec::new(),
			move_history: Vec::new(),
			undo_history: Vec::new(),
			game_status: GameStatus::Playing,
		};
		game.update_game_status()?;
		Ok(game)
	}
}

pub fn load_position_from_fen(fen: &str) -> Result<Position, FenError> {
	let mut position = Position {
		board: [None; 64],
		castle: [false; 4],
		en_passant: None,
		side_to_move: Side::White,
		king_squares: [4, 60],
		halfmove_clock: 0,
		fullmove_counter: 0,
		zobrist_hash: 0,
	};

	let fen_parts: Vec<&str> = fen.split_whitespace().collect();

	if fen_parts.len() != 6 {
		return Err(FenError::MissingFenParts);
	}

	position.fullmove_counter = load_clock(fen_parts[5])?;

	position.halfmove_clock = load_clock(fen_parts[4])?;

	load_en_passant(&mut position, fen_parts[3])?;

	load_castling_ability(&mut position, fen_parts[2])?;

	load_side_to_move(&mut position, fen_parts[1])?;

	load_piece_placement(&mut position, fen_parts[0])?;

	for (square, piece) in position.board.iter().enumerate() {
		if let Some(colored_piece) = piece {
			if colored_piece.piece == Piece::King {
				match colored_piece.side {
					Side::White => position.king_squares[0] = square as u8,
					Side::Black => position.king_squares[1] = square as u8,
				}
			}
		}
	}

	position.zobrist_hash = position.compute_hash();

	Ok(position)
}

impl Position {
	pub fn print_board(&self) {
		for rank in (0..8).rev() {
			print!("{} ", rank + 1);
			for file in 0..8 {
				let square = square(file, rank);
				let test = self.board[square as usize];
				match test {
					Some(piece) => print!(" {}", piece.to_char()),
					None => print!(" ."),
				}
			}
			print!("\n");
		}
		println!("   a b c d e f g h");
	}
}

fn load_castling_ability(position: &mut Position, castling_rules: &str) -> Result<(), FenError> {
	for castle_char in castling_rules.chars() {
		match castle_char {
			'K' => position.castle[0] = true,
			'Q' => position.castle[1] = true,
			'k' => position.castle[2] = true,
			'q' => position.castle[3] = true,
			'-' => position.castle = [false; 4],
			other => return Err(FenError::InvalidCastlingRights(other)),
		}
	}

	Ok(())
}

fn load_en_passant(position: &mut Position, en_passant_string: &str) -> Result<(), FenError> {
	if en_passant_string == "-" {
		return Ok(());
	}

	match convert_square_string_to_square(en_passant_string) {
		Ok(x) => position.en_passant = Some(x),
		Err(x) => return Err(x),
	}

	Ok(())
}

fn load_side_to_move(position: &mut Position, side_to_move: &str) -> Result<(), FenError> {
	match side_to_move {
		"b" => position.side_to_move = Side::Black,
		"w" => position.side_to_move = Side::White,
		_ => return Err(FenError::NotAValideSide),
	};

	Ok(())
}

fn load_piece_placement(position: &mut Position, fen_board: &str) -> Result<(), FenError> {
	let fen_ranks = fen_board.split("/");

	let mut current_rank = 7;
	for rank_str in fen_ranks {
		let mut file = 0;

		for c in rank_str.chars() {
			if let Some(digit) = c.to_digit(10) {
				file += digit as u8;
			} else {
				let piece_side = if c.is_uppercase() {
					Side::White
				} else {
					Side::Black
				};
				let piece_type = match c.to_ascii_lowercase() {
					'k' => Piece::King,
					'p' => Piece::Pawn,
					'n' => Piece::Knight,
					'b' => Piece::Bishop,
					'r' => Piece::Rook,
					'q' => Piece::Queen,
					_ => return Err(FenError::InvalidPieceChar(c)),
				};

				let piece = ColoredPiece { piece: piece_type, side: piece_side };

				let target_square = square(file, current_rank);
				position.board[target_square as usize] = Some(piece);

				file += 1;
			}
		}

		if current_rank > 0 {
			current_rank -= 1;
		}
	}

	Ok(())
}

fn load_clock(clock_string: &str) -> Result<u32, FenError> {
	// please dont judge me i am to lazy to find a var name
	let new_clock_string = clock_string.to_string();
	match new_clock_string.parse::<u32>() {
		Ok(x) => return Ok(x),
		Err(_) => return Err(FenError::InvalidNumber(new_clock_string.to_string())),
	};
}

fn convert_square_string_to_square(square_string: &str) -> Result<u8, FenError> {
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

#[cfg(test)]
mod tests;
