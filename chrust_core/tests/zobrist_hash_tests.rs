mod common;

use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::zobrist::zobrist;
use chrust_core::{ColoredPiece, Piece, Side};
use common::{assert_hash_matches_computed, empty_position, make_move_and_verify_hash, position_with_hash};

// ══════════════════════════════════════════════════════════════════════════════
// compute_hash() Basic Tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn compute_hash_empty_board_white_to_move() {
	let pos = empty_position();
	let hash = pos.compute_hash();
	// Empty board with white to move should have hash 0 (no pieces, no castling, no ep)
	assert_eq!(hash, 0);
}

#[test]
fn compute_hash_empty_board_black_to_move() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	let hash = pos.compute_hash();

	// Should equal just the side hash
	let z = zobrist();
	assert_eq!(hash, z.side);
}

#[test]
fn compute_hash_single_white_pawn_a2() {
	let mut pos = empty_position();
	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn); // a2

	let hash = pos.compute_hash();
	let z = zobrist();

	// Should be exactly the hash for white pawn on a2
	assert_eq!(hash, z.pieces[0][8]);
}

#[test]
fn compute_hash_single_black_knight_g8() {
	let mut pos = empty_position();
	let knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[62] = Some(knight); // g8

	let hash = pos.compute_hash();
	let z = zobrist();

	// Black knight = piece_index 7
	assert_eq!(hash, z.pieces[7][62]);
}

#[test]
fn compute_hash_two_pieces_different_colors() {
	let mut pos = empty_position();
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_queen = ColoredPiece { piece: Piece::Queen, side: Side::Black };
	pos.board[4] = Some(white_rook); // e1
	pos.board[59] = Some(black_queen); // d8

	let hash = pos.compute_hash();
	let z = zobrist();

	// Should be XOR of both pieces
	let expected = z.pieces[3][4] ^ z.pieces[10][59];
	assert_eq!(hash, expected);
}

#[test]
fn compute_hash_starting_position() {
	let pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// Just verify it's non-zero and consistent
	assert_ne!(pos.zobrist_hash, 0);
	let recomputed = pos.compute_hash();
	assert_eq!(pos.zobrist_hash, recomputed);
}

#[test]
fn compute_hash_side_to_move_matters() {
	let pos_white = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let pos_black = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");

	// Hashes should differ by exactly the side hash
	let z = zobrist();
	assert_eq!(pos_white.zobrist_hash ^ z.side, pos_black.zobrist_hash);
}

#[test]
fn compute_hash_same_position_same_hash() {
	let pos1 = position_with_hash("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1");
	let pos2 = position_with_hash("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1");

	assert_eq!(pos1.zobrist_hash, pos2.zobrist_hash);
}

#[test]
fn compute_hash_different_piece_same_square() {
	let mut pos1 = empty_position();
	let mut pos2 = empty_position();

	pos1.board[28] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });
	pos2.board[28] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	let hash1 = pos1.compute_hash();
	let hash2 = pos2.compute_hash();

	assert_ne!(hash1, hash2);
}

#[test]
fn compute_hash_same_piece_different_square() {
	let mut pos1 = empty_position();
	let mut pos2 = empty_position();

	let knight = ColoredPiece { piece: Piece::Knight, side: Side::White };
	pos1.board[28] = Some(knight); // e4
	pos2.board[29] = Some(knight); // e5 (actually f4, but different square)

	let hash1 = pos1.compute_hash();
	let hash2 = pos2.compute_hash();

	assert_ne!(hash1, hash2);
}

#[test]
fn compute_hash_with_castling_rights() {
	let pos_all = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
	let pos_white_only = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1");
	let pos_none = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1");

	// All three should have different hashes
	assert_ne!(pos_all.zobrist_hash, pos_white_only.zobrist_hash);
	assert_ne!(pos_all.zobrist_hash, pos_none.zobrist_hash);
	assert_ne!(pos_white_only.zobrist_hash, pos_none.zobrist_hash);
}

#[test]
fn compute_hash_with_en_passant_square() {
	let pos_no_ep = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
	let pos_e3_ep = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
	let pos_d3_ep = position_with_hash("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");

	// All three should have different hashes
	assert_ne!(pos_no_ep.zobrist_hash, pos_e3_ep.zobrist_hash);
	assert_ne!(pos_no_ep.zobrist_hash, pos_d3_ep.zobrist_hash);
	assert_ne!(pos_e3_ep.zobrist_hash, pos_d3_ep.zobrist_hash);
}

// ══════════════════════════════════════════════════════════════════════════════
// Incremental Hash - Quiet Moves
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_updated_quiet_move_knight() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	let mv = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_quiet_move_bishop() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");

	// Move white bishop from c1 to f4
	let mut pos_white = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");

	let mv = Move {
		from_square: 2,
		to_square: 21,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Bishop, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos_white, mv);
}

#[test]
fn hash_updated_quiet_move_rook() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 0,
		to_square: 1,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Rook, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_quiet_move_queen() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// This will be blocked, so let's use a more open position
	let mut pos_open = position_with_hash("rnbqkbnr/pppppppp/8/8/8/3Q4/PPPPPPPP/RNB1KBNR w KQkq - 0 1");

	let mv = Move {
		from_square: 19,
		to_square: 28,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Queen, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos_open, mv);
}

#[test]
fn hash_updated_quiet_move_king() {
	let mut pos = position_with_hash("4k3/8/8/8/8/8/8/4K3 w - - 0 1");

	let mv = Move {
		from_square: 4,
		to_square: 12,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_quiet_move_pawn() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	let mv = Move {
		from_square: 12,
		to_square: 20,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

// ══════════════════════════════════════════════════════════════════════════════
// Incremental Hash - Captures
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_updated_simple_capture() {
	let mut pos = position_with_hash("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1");

	// d4 captures e5
	let mv = Move {
		from_square: 27, // d4
		to_square: 36,   // e5
		move_kind: MoveKind::Capture,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_capture_promotion() {
	let mut pos = position_with_hash("4k3/4P3/8/8/8/8/8/4K3 w - - 0 1");
	pos.board[51] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // d7 black pawn
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 52,
		to_square: 59,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_capture_on_rook_square() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/4P3/R3K2R w KQkq - 0 1");

	// Capture black rook on a8 with white pawn (unrealistic but tests the hash)
	pos.board[48] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // a7
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Capture,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_en_passant_capture() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// Setup en passant position
	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // d5
	pos.board[36] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // e5
	pos.en_passant = Some(44); // e6
	pos.side_to_move = Side::White;
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 35,
		to_square: 44,
		move_kind: MoveKind::EnPassant { capture_square: 36 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

// ══════════════════════════════════════════════════════════════════════════════
// Incremental Hash - Special Moves
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_updated_double_pawn_push() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	let mv = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_promotion_to_queen() {
	let mut pos = position_with_hash("4k3/4P3/8/8/8/8/8/4K3 w - - 0 1");

	let mv = Move {
		from_square: 52,
		to_square: 60,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_promotion_to_rook() {
	let mut pos = position_with_hash("4k3/4P3/8/8/8/8/8/4K3 w - - 0 1");

	let mv = Move {
		from_square: 52,
		to_square: 60,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Rook },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_promotion_to_bishop() {
	let mut pos = position_with_hash("4k3/4P3/8/8/8/8/8/4K3 w - - 0 1");

	let mv = Move {
		from_square: 52,
		to_square: 60,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Bishop },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_promotion_to_knight() {
	let mut pos = position_with_hash("4k3/4P3/8/8/8/8/8/4K3 w - - 0 1");

	let mv = Move {
		from_square: 52,
		to_square: 60,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Knight },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_white_kingside_castle() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 4,
		to_square: 6,
		move_kind: MoveKind::Castling { rook_from: 7, rook_to: 5 },
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_white_queenside_castle() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 4,
		to_square: 2,
		move_kind: MoveKind::Castling { rook_from: 0, rook_to: 3 },
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

#[test]
fn hash_updated_black_kingside_castle() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");

	let mv = Move {
		from_square: 60,
		to_square: 62,
		move_kind: MoveKind::Castling { rook_from: 63, rook_to: 61 },
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::Black },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

// ══════════════════════════════════════════════════════════════════════════════
// Castling Rights Hash Updates
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_updated_king_move_loses_both_castling() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 4,
		to_square: 12,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	// After king moves, white should lose both K and Q castling rights
	assert!(!pos.castle[0], "white kingside castling should be lost");
	assert!(!pos.castle[1], "white queenside castling should be lost");
}

#[test]
fn hash_updated_h1_rook_move_loses_kingside() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 7,
		to_square: 6,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Rook, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	assert!(!pos.castle[0], "white kingside castling should be lost");
	assert!(pos.castle[1], "white queenside castling should remain");
}

#[test]
fn hash_updated_a1_rook_move_loses_queenside() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	let mv = Move {
		from_square: 0,
		to_square: 1,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Rook, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	assert!(pos.castle[0], "white kingside castling should remain");
	assert!(!pos.castle[1], "white queenside castling should be lost");
}

#[test]
fn hash_updated_capture_on_h8_loses_black_kingside() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/7P/R3K2R w KQkq - 0 1");

	// Pawn captures rook on h8 (with promotion)
	pos.board[55] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // h7
	pos.zobrist_hash = pos.compute_hash();

	let mv = Move {
		from_square: 55,
		to_square: 63,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	assert!(!pos.castle[2], "black kingside castling should be lost");
}

#[test]
fn hash_updated_multiple_castling_rights_lost() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

	// Move king
	let mv1 = Move {
		from_square: 4,
		to_square: 12,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv1);

	// Move black king
	let mv2 = Move {
		from_square: 60,
		to_square: 52,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::Black },
	};
	make_move_and_verify_hash(&mut pos, mv2);

	// All castling rights should be lost
	assert!(pos.castle.iter().all(|&c| !c), "all castling rights should be lost");
}

#[test]
fn hash_castling_rights_already_lost_no_change() {
	let mut pos = position_with_hash("r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1");

	// King has no castling rights, move it
	let mv = Move {
		from_square: 4,
		to_square: 12,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
}

// ══════════════════════════════════════════════════════════════════════════════
// En Passant Hash Updates
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_en_passant_set_different_files() {
	// Test e-file
	let mut pos_e = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let mv_e = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos_e, mv_e);

	// Test d-file
	let mut pos_d = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let mv_d = Move {
		from_square: 11,
		to_square: 27,
		move_kind: MoveKind::DoublePawnPush { passed_square: 19 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos_d, mv_d);

	// Different files should produce different hashes
	assert_ne!(pos_e.zobrist_hash, pos_d.zobrist_hash);
}

#[test]
fn hash_en_passant_cleared_by_quiet_move() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

	// Black makes a move that clears en passant
	let mv = Move {
		from_square: 57,
		to_square: 42,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};

	make_move_and_verify_hash(&mut pos, mv);
	assert_eq!(pos.en_passant, None);
}

#[test]
fn hash_en_passant_cleared_by_capture() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// Setup position with en passant and a capture available
	pos.board[12] = None; // remove e2 pawn
	pos.board[28] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e4
	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // d5
	pos.en_passant = Some(43); // d6
	pos.side_to_move = Side::White;
	pos.zobrist_hash = pos.compute_hash();

	// Capture
	let mv = Move {
		from_square: 28,
		to_square: 35,
		move_kind: MoveKind::Capture,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);
	assert_eq!(pos.en_passant, None);
}

#[test]
fn hash_en_passant_file_specific() {
	let pos_e4 = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
	let pos_d4 = position_with_hash("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");

	// Different en passant files should give different hashes
	assert_ne!(pos_e4.zobrist_hash, pos_d4.zobrist_hash);
}

// ══════════════════════════════════════════════════════════════════════════════
// Side to Move Hash Updates
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_toggles_after_white_move() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let hash_before = pos.zobrist_hash;

	let mv = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	// Side should have toggled from white to black
	assert_eq!(pos.side_to_move, Side::Black);
}

#[test]
fn hash_toggles_after_black_move() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");

	let mv = Move {
		from_square: 52,
		to_square: 36,
		move_kind: MoveKind::DoublePawnPush { passed_square: 44 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::Black },
	};

	make_move_and_verify_hash(&mut pos, mv);

	assert_eq!(pos.side_to_move, Side::White);
}

#[test]
fn hash_consistent_after_full_move() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// White move
	let mv1 = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv1);

	// Black move
	let mv2 = Move {
		from_square: 52,
		to_square: 36,
		move_kind: MoveKind::DoublePawnPush { passed_square: 44 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::Black },
	};
	make_move_and_verify_hash(&mut pos, mv2);

	assert_eq!(pos.side_to_move, Side::White);
}

// ══════════════════════════════════════════════════════════════════════════════
// Complex Sequences
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn hash_consistent_through_opening() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	// 1. e4
	let mv1 = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv1);

	// 1... e5
	let mv2 = Move {
		from_square: 52,
		to_square: 36,
		move_kind: MoveKind::DoublePawnPush { passed_square: 44 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::Black },
	};
	make_move_and_verify_hash(&mut pos, mv2);

	// 2. Nf3
	let mv3 = Move {
		from_square: 6,
		to_square: 21,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv3);

	// 2... Nc6
	let mv4 = Move {
		from_square: 57,
		to_square: 42,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};
	make_move_and_verify_hash(&mut pos, mv4);
}

#[test]
fn hash_transposition_same_position() {
	// Test that different move orders reaching the same position give the same hash
	// Path 1: 1. e4 c5 2. Nf3 Nc6
	let mut pos1 = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	let mv1a = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	pos1.make_move_unvalidated(mv1a).unwrap();

	let mv1b = Move {
		from_square: 50,
		to_square: 34,
		move_kind: MoveKind::DoublePawnPush { passed_square: 42 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::Black },
	};
	pos1.make_move_unvalidated(mv1b).unwrap();

	let mv1c = Move {
		from_square: 6,
		to_square: 21,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};
	pos1.make_move_unvalidated(mv1c).unwrap();

	let mv1d = Move {
		from_square: 57,
		to_square: 42,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};
	pos1.make_move_unvalidated(mv1d).unwrap();
	pos1.zobrist_hash = pos1.compute_hash();

	// Path 2: 1. Nf3 c5 2. e4 Nc6
	let mut pos2 = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	let mv2a = Move {
		from_square: 6,
		to_square: 21,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};
	pos2.make_move_unvalidated(mv2a).unwrap();

	let mv2b = Move {
		from_square: 50,
		to_square: 34,
		move_kind: MoveKind::DoublePawnPush { passed_square: 42 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::Black },
	};
	pos2.make_move_unvalidated(mv2b).unwrap();

	let mv2c = Move {
		from_square: 12,
		to_square: 28,
		move_kind: MoveKind::DoublePawnPush { passed_square: 20 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	pos2.make_move_unvalidated(mv2c).unwrap();

	let mv2d = Move {
		from_square: 57,
		to_square: 42,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};
	pos2.make_move_unvalidated(mv2d).unwrap();
	pos2.zobrist_hash = pos2.compute_hash();

	// Both should reach the same position with the same hash
	assert_eq!(pos1.zobrist_hash, pos2.zobrist_hash);
}

#[test]
fn hash_repetition_detection() {
	let mut pos = position_with_hash("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let initial_hash = pos.zobrist_hash;

	// Move knight out and back
	let mv1 = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};
	pos.make_move_unvalidated(mv1).unwrap();

	let mv2 = Move {
		from_square: 57,
		to_square: 42,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};
	pos.make_move_unvalidated(mv2).unwrap();

	let mv3 = Move {
		from_square: 18,
		to_square: 1,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::White },
	};
	pos.make_move_unvalidated(mv3).unwrap();

	let mv4 = Move {
		from_square: 42,
		to_square: 57,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
	};
	pos.make_move_unvalidated(mv4).unwrap();

	pos.zobrist_hash = pos.compute_hash();

	// Should be back to initial position
	assert_eq!(pos.zobrist_hash, initial_hash);
}

#[test]
fn hash_after_complex_tactical_sequence() {
	let mut pos = position_with_hash("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1");

	// Play a series of moves
	let moves = vec![
		// Bc4
		Move {
			from_square: 5,
			to_square: 33,
			move_kind: MoveKind::Quiet,
			colored_piece: ColoredPiece { piece: Piece::Bishop, side: Side::White },
		},
		// Nf6
		Move {
			from_square: 62, // g8
			to_square: 45,   // f6
			move_kind: MoveKind::Quiet,
			colored_piece: ColoredPiece { piece: Piece::Knight, side: Side::Black },
		},
		// O-O
		Move {
			from_square: 4,
			to_square: 6,
			move_kind: MoveKind::Castling { rook_from: 7, rook_to: 5 },
			colored_piece: ColoredPiece { piece: Piece::King, side: Side::White },
		},
	];

	for mv in moves {
		make_move_and_verify_hash(&mut pos, mv);
	}
}

#[test]
fn hash_multiple_promotions() {
	let mut pos = position_with_hash("4k3/PPPPPPPP/8/8/8/8/8/4K3 w - - 0 1");

	// Promote first pawn
	let mv1 = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv1);

	// Black king moves
	let mv2 = Move {
		from_square: 60,
		to_square: 61,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::King, side: Side::Black },
	};
	make_move_and_verify_hash(&mut pos, mv2);

	// Promote second pawn
	let mv3 = Move {
		from_square: 49,
		to_square: 57,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Rook },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	make_move_and_verify_hash(&mut pos, mv3);
}

#[test]
fn hash_edge_case_all_features_combined() {
	// A move that affects multiple hash components
	let mut pos = position_with_hash("r3k2r/8/8/pP6/8/8/8/R3K2R w KQkq a6 0 1");

	// Capture en passant (affects pieces, en passant, side)
	let mv = Move {
		from_square: 33,
		to_square: 40,
		move_kind: MoveKind::EnPassant { capture_square: 32 },
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};

	make_move_and_verify_hash(&mut pos, mv);

	// Verify en passant was cleared and side changed
	assert_eq!(pos.en_passant, None);
	assert_eq!(pos.side_to_move, Side::Black);
}
