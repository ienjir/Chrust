use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::{Side, Square, position::Position};

pub fn empty_position() -> Position {
	Position {
		board: [None; 64],
		side_to_move: Side::White,
		castle: [false; 4],
		en_passant: None,
		king_squares: [4, 60],
		fullmove_number: 0,
		halfmove_clock: 0,
	}
}

// Helper functions for move_gen tests
pub fn has_move(moves: &[Move], from: Square, to: Square, kind: MoveKind) -> bool {
	moves.iter().any(|m| m.from_square == from && m.to_square == to && m.move_kind == kind)
}

pub fn has_to_square(moves: &[Move], to: Square) -> bool {
	moves.iter().any(|m| m.to_square == to)
}
