use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::{
	position::{load_position_from_fen, Position},
	Side, Square,
};

pub fn empty_position() -> Position {
	Position {
		board: [None; 64],
		side_to_move: Side::White,
		castle: [false; 4],
		en_passant: None,
		zobrist_hash: 0,
		king_squares: [4, 60],
		fullmove_counter: 0,
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

// ── Zobrist hash test helpers ────────────────────────────────────────────────

/// Verify that position's zobrist_hash matches compute_hash()
pub fn assert_hash_matches_computed(pos: &Position) {
	let computed = pos.compute_hash();
	assert_eq!(pos.zobrist_hash, computed, "Incremental hash 0x{:016x} doesn't match computed hash 0x{:016x}", pos.zobrist_hash, computed);
}

/// Create a position from FEN and compute initial hash
pub fn position_with_hash(fen: &str) -> Position {
	let mut pos = load_position_from_fen(fen).unwrap();
	pos.zobrist_hash = pos.compute_hash();
	pos
}

/// Make a move and verify hash is updated correctly
pub fn make_move_and_verify_hash(pos: &mut Position, mv: Move) {
	pos.make_move_unvalidated(mv).unwrap();
	assert_hash_matches_computed(pos);
}
