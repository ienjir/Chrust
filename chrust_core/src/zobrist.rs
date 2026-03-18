use std::sync::LazyLock;

use crate::{ColoredPiece, Piece, Side, position::Position};

pub struct ZobristTable {
	pub pieces: [[u64; 64]; 12],
	pub side: u64,
	pub castling: [u64; 4],
	pub enpassant: [u64; 8],
}

static ZOBRIST: LazyLock<ZobristTable> = LazyLock::new(ZobristTable::new);

struct Splitmix64(u64);

pub fn zobrist() -> &'static ZobristTable {
	&ZOBRIST
}

impl ZobristTable {
	pub fn new() -> Self {
		let mut rng = Splitmix64 (6769420);

		let mut pieces = [[0u64; 64]; 12];
		for piece in 0..12 {
			for square in 0..64 {
				pieces[piece][square] = rng.next_u64();
			}
		}

		ZobristTable {
			pieces,
			side: rng.next_u64(),
			castling: std::array::from_fn(|_| rng.next_u64()),
			enpassant: std::array::from_fn(|_| rng.next_u64()) ,
		}
	}
}

impl Splitmix64 {
	fn next_u64(&mut self) -> u64 {
		self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
		let mut z = self.0;
		z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
		z ^ (z >> 31)
	}
}

pub fn piece_index(colored_piece: ColoredPiece) -> usize {
	let piece_idx = match colored_piece.piece {
		Piece::Pawn   => 0,
		Piece::Knight => 1,
		Piece::Bishop => 2,
		Piece::Rook   => 3,
		Piece::Queen  => 4,
		Piece::King   => 5,
	};
	let side_offset = match colored_piece.side {
		Side::White => 0,
		Side::Black => 6,
	};
	piece_idx + side_offset
}

impl Position {
	pub fn compute_hash(&self) -> u64 {
		let z = zobrist();
		let mut hash = 0u64;

		for (square, piece) in self.board.iter().enumerate() {
			if let Some(cp) = piece {
				hash ^= z.pieces[piece_index(*cp)][square];
			}
		}

		if self.side_to_move == Side::Black {
			hash ^= z.side;
		}

		for i in 0..4 {
			if self.castle[i] {
				hash ^= z.castling[i];
			}
		}

		if let Some(ep_square) = self.en_passant {
			let file = (ep_square % 8) as usize;
			hash ^= z.enpassant[file];
		}

		hash
	}
}
