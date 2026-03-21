mod common;

use chrust_core::game_status::GameStatus;
use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::{Game, Position};
use chrust_core::zobrist::zobrist;
use chrust_core::{ColoredPiece, Piece, Side, errors::ChessError};
use common::{empty_game, empty_position, game_from_fen};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Build a Move from its components; `colored_piece` is taken from the board.
fn mv(pos: &Position, from: u8, to: u8, kind: MoveKind) -> Move {
	Move {
		from_square: from,
		to_square: to,
		move_kind: kind,
		colored_piece: pos.board[from as usize].expect("helper mv(): no piece on from_square"),
	}
}

// ══════════════════════════════════════════════════════════════════════════════
// make_move validation tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn make_move_errors_if_initial_square_empty() {
	let mut game = empty_game();

	let mv = Move {
		from_square: 0,
		to_square: 1,
		move_kind: MoveKind::Quiet,
		colored_piece: ColoredPiece { piece: Piece::Pawn, side: Side::White },
	};
	assert!(matches!(game.make_move(&mv), Err(ChessError::NoPieceOnSquare { square: 0 })));
}

#[test]
fn make_move_errors_if_from_square_out_of_bounds() {
	let mut game = empty_game();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let mv1 = Move {
		from_square: 64,
		to_square: 0,
		move_kind: MoveKind::Quiet,
		colored_piece: pawn,
	};
	let mv3 = Move {
		from_square: 200,
		to_square: 201,
		move_kind: MoveKind::Quiet,
		colored_piece: pawn,
	};

	assert!(matches!(game.make_move(&mv1), Err(ChessError::NotASquareOnBoard { square: 64 })));
	assert!(matches!(game.make_move(&mv3), Err(ChessError::NotASquareOnBoard { square: 200 })));
}

#[test]
fn make_move_errors_if_to_square_out_of_bounds() {
	let mut game = empty_game();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	game.position.board[0] = Some(pawn);
	let mv2 = Move {
		from_square: 0,
		to_square: 64,
		move_kind: MoveKind::Quiet,
		colored_piece: pawn,
	};

	assert!(matches!(game.make_move(&mv2), Err(ChessError::NotASquareOnBoard { square: 64 })));
}

#[test]
fn make_move_errors_if_move_not_in_legal_list() {
	let mut game = empty_game();

	// A rook on a1 cannot move diagonally to h8.
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	game.position.board[0] = Some(rook);

	let illegal = Move {
		from_square: 0,
		to_square: 63,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};
	assert!(matches!(game.make_move(&illegal), Err(ChessError::NotAValidMove)));
}

#[test]
fn promotion_errors_if_promotion_piece_is_none() {
	let mut game = empty_game();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	game.position.board[48] = Some(pawn); // a7

	// The pawn generator emits Promotion { promotion_piece: Some(Piece::Pawn) }
	// as a sentinel.  A None promotion_piece is never in the legal list so the
	// move is rejected (NotAValidMove) before the None-check is even reached.
	let mv = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Pawn },
		colored_piece: pawn,
	};
	assert!(matches!(game.make_move(&mv), Err(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// make_move_unvalidated board state tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn quiet_move_clears_source_and_sets_target() {
	let mut pos = empty_position();

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[0] = Some(rook); // a1

	let m = mv(&pos, 0, 7, MoveKind::Quiet); // a1 → h1
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[0], None);
	assert_eq!(pos.board[7], Some(rook));
}

#[test]
fn capture_move_replaces_enemy_piece() {
	let mut pos = empty_position();

	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[0] = Some(white_rook);
	pos.board[7] = Some(black_knight);

	let m = mv(&pos, 0, 7, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[0], None);
	assert_eq!(pos.board[7], Some(white_rook));
}

#[test]
fn capture_stores_captured_piece_in_undo() {
	let mut pos = empty_position();

	let white_bishop = ColoredPiece { piece: Piece::Bishop, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[10] = Some(white_bishop);
	pos.board[28] = Some(black_pawn);

	let m = mv(&pos, 10, 28, MoveKind::Capture);
	let undo = pos.make_move_unvalidated(m).unwrap();

	assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn en_passant_clears_capture_square_and_moves_pawn() {
	let mut pos = empty_position();

	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[33] = Some(white_pawn); // b5
	pos.board[34] = Some(black_pawn); // c5 — the en-passant captured pawn
	pos.en_passant = Some(42); // c6 — the square the white pawn moves to

	let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
	let undo = pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[33], None);
	assert_eq!(pos.board[34], None);
	assert_eq!(pos.board[42], Some(white_pawn));
	assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn double_pawn_push_moves_piece() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn); // a2

	let m = mv(&pos, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 }); // a2 → a4
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[8], None);
	assert_eq!(pos.board[24], Some(pawn));
}

#[test]
fn promotion_changes_piece_type() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[48] = Some(pawn); // a7

	// The pawn generator now emits 4 separate moves for each promotion choice.
	// Test that promoting to Queen works correctly.
	let m = mv(&pos, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Queen });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[48], None);
	assert_eq!(pos.board[56], Some(ColoredPiece { piece: Piece::Queen, side: Side::White }));
}

#[test]
fn promotion_with_pawn_sentinel_is_rejected() {
	// The Pawn piece type is NOT valid for promotions.
	// make_move_unvalidated checks this in apply_move_to_board.
	let mut pos = empty_position();
	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[48] = Some(pawn); // a7

	let m = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Pawn },
		colored_piece: pawn,
	};
	// Should be rejected by apply_move_to_board's validation
	assert!(matches!(pos.make_move_unvalidated(m), Err(ChessError::PromotionPieceCantBePawn)), "promotion to Pawn should be rejected");
}

#[test]
fn castling_white_kingside_moves_king_and_rook() {
	let mut pos = empty_position();

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king); // e1
	pos.board[7] = Some(rook); // h1
	pos.castle[0] = true; // white kingside

	let m = mv(&pos, 4, 6, MoveKind::Castling { rook_from: 7, rook_to: 5 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[4], None);
	assert_eq!(pos.board[6], Some(king));
	assert_eq!(pos.board[7], None);
	assert_eq!(pos.board[5], Some(rook));
}

#[test]
fn castling_white_queenside_moves_king_and_rook() {
	let mut pos = empty_position();

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king); // e1
	pos.board[0] = Some(rook); // a1
	pos.castle[1] = true; // white queenside

	let m = mv(&pos, 4, 2, MoveKind::Castling { rook_from: 0, rook_to: 3 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[4], None);
	assert_eq!(pos.board[2], Some(king));
	assert_eq!(pos.board[0], None);
	assert_eq!(pos.board[3], Some(rook));
}

#[test]
fn castling_black_kingside_moves_king_and_rook() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[60] = Some(king); // e8
	pos.board[63] = Some(rook); // h8
	pos.castle[2] = true; // black kingside

	let m = mv(&pos, 60, 62, MoveKind::Castling { rook_from: 63, rook_to: 61 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[60], None);
	assert_eq!(pos.board[62], Some(king));
	assert_eq!(pos.board[63], None);
	assert_eq!(pos.board[61], Some(rook));
}

#[test]
fn castling_black_queenside_moves_king_and_rook() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[60] = Some(king); // e8
	pos.board[56] = Some(rook); // a8
	pos.castle[3] = true; // black queenside

	let m = mv(&pos, 60, 58, MoveKind::Castling { rook_from: 56, rook_to: 59 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[60], None);
	assert_eq!(pos.board[58], Some(king));
	assert_eq!(pos.board[56], None);
	assert_eq!(pos.board[59], Some(rook));
}

// ── side_to_move ──────────────────────────────────────────────────────────────

#[test]
fn make_move_toggles_side_to_move_white_to_black() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn); // a2

	let m = mv(&pos, 8, 16, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.side_to_move, Side::Black);
}

#[test]
fn make_move_toggles_side_to_move_black_to_white() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[48] = Some(pawn); // a7

	let m = mv(&pos, 48, 40, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.side_to_move, Side::White);
}

// ── halfmove_clock ────────────────────────────────────────────────────────────

#[test]
fn halfmove_clock_resets_on_pawn_move() {
	let mut pos = empty_position();
	pos.halfmove_clock = 10;

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn); // a2

	let m = mv(&pos, 8, 16, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_resets_on_capture() {
	let mut pos = empty_position();
	pos.halfmove_clock = 5;

	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[0] = Some(white_rook);
	pos.board[7] = Some(black_knight);

	let m = mv(&pos, 0, 7, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_resets_on_en_passant() {
	let mut pos = empty_position();
	pos.halfmove_clock = 3;

	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[33] = Some(white_pawn); // b5
	pos.board[34] = Some(black_pawn); // c5
	pos.en_passant = Some(42); // c6

	let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_increments_on_quiet_non_pawn_move() {
	let mut pos = empty_position();
	pos.halfmove_clock = 4;

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[0] = Some(rook); // a1

	let m = mv(&pos, 0, 7, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.halfmove_clock, 5);
}

// ── fullmove_number ───────────────────────────────────────────────────────────

#[test]
fn fullmove_number_increments_only_after_black_move() {
	// Per chess rules, fullmove_counter increments after Black's move only.
	let mut pos = empty_position();
	assert_eq!(pos.fullmove_counter, 0);

	// White's move — fullmove_counter should NOT change.
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(white_pawn); // a2
	let m1 = mv(&pos, 8, 16, MoveKind::Quiet);
	pos.make_move_unvalidated(m1).unwrap();
	assert_eq!(pos.fullmove_counter, 0, "fullmove_counter should not change after White moves");

	// Black's move — fullmove_counter should increment.
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[48] = Some(black_pawn); // a7
	let m2 = mv(&pos, 48, 40, MoveKind::Quiet);
	pos.make_move_unvalidated(m2).unwrap();
	assert_eq!(pos.fullmove_counter, 1, "fullmove_counter should increment after Black moves");
}

// ── en_passant state ──────────────────────────────────────────────────────────

#[test]
fn double_pawn_push_sets_en_passant_square() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn); // a2

	let m = mv(&pos, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.en_passant, Some(16));
}

#[test]
fn any_other_move_clears_en_passant() {
	let mut pos = empty_position();
	pos.en_passant = Some(42); // some leftover en passant square

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[0] = Some(rook);

	let m = mv(&pos, 0, 7, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.en_passant, None);
}

// ── castling rights revocation ────────────────────────────────────────────────

#[test]
fn white_king_move_revokes_both_white_castling_rights() {
	let mut pos = empty_position();
	pos.castle = [true, true, true, true];

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	pos.board[4] = Some(king); // e1

	let m = mv(&pos, 4, 5, MoveKind::Quiet); // e1 → f1
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[0], "white kingside right should be revoked");
	assert!(!pos.castle[1], "white queenside right should be revoked");
	assert!(pos.castle[2], "black kingside right should be untouched");
	assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
fn black_king_move_revokes_both_black_castling_rights() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.castle = [true, true, true, true];

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	pos.board[60] = Some(king); // e8

	let m = mv(&pos, 60, 61, MoveKind::Quiet); // e8 → f8
	pos.make_move_unvalidated(m).unwrap();

	assert!(pos.castle[0], "white kingside right should be untouched");
	assert!(pos.castle[1], "white queenside right should be untouched");
	assert!(!pos.castle[2], "black kingside right should be revoked");
	assert!(!pos.castle[3], "black queenside right should be revoked");
}

#[test]
fn white_kingside_rook_move_revokes_white_kingside_right() {
	// H1 rook (square 7) → castle[0] = white kingside.
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[7] = Some(rook); // h1

	let m = mv(&pos, 7, 6, MoveKind::Quiet); // h1 → g1
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[0], "white kingside right should be revoked after h1 rook moves");
	assert!(pos.castle[1], "white queenside right should be untouched");
}

#[test]
fn white_queenside_rook_move_revokes_white_queenside_right() {
	// A1 rook (square 0) → castle[1] = white queenside.
	// NOTE: make_move currently clears castle[0] instead of castle[1] for
	// square 0 (the indices are swapped — see bug report).  This test asserts
	// the CORRECT expected behaviour; it will fail until the bug is fixed.
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[0] = Some(rook); // a1

	let m = mv(&pos, 0, 1, MoveKind::Quiet); // a1 → b1
	pos.make_move_unvalidated(m).unwrap();

	assert!(pos.castle[0], "white kingside right should be untouched");
	assert!(!pos.castle[1], "white queenside right should be revoked after a1 rook moves");
}

#[test]
fn black_kingside_rook_move_revokes_black_kingside_right() {
	// H8 rook (square 63) → castle[2] = black kingside.
	// NOTE: make_move currently clears castle[3] instead of castle[2] for
	// square 63 (the indices are swapped — see bug report).
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[63] = Some(rook); // h8

	let m = mv(&pos, 63, 62, MoveKind::Quiet); // h8 → g8
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[2], "black kingside right should be revoked after h8 rook moves");
	assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
fn black_queenside_rook_move_revokes_black_queenside_right() {
	// A8 rook (square 56) → castle[3] = black queenside.
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[56] = Some(rook); // a8

	let m = mv(&pos, 56, 57, MoveKind::Quiet); // a8 → b8
	pos.make_move_unvalidated(m).unwrap();

	assert!(pos.castle[2], "black kingside right should be untouched");
	assert!(!pos.castle[3], "black queenside right should be revoked after a8 rook moves");
}

#[test]
fn capturing_white_kingside_rook_on_h1_revokes_white_kingside_right() {
	// A black piece captures the White h1 rook → castle[0] (white kingside)
	// should be cleared.
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.castle = [true, true, false, false];

	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[15] = Some(black_rook); // h2
	pos.board[7] = Some(white_rook); // h1

	let m = mv(&pos, 15, 7, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[0], "white kingside right should be revoked when h1 rook is captured");
	assert!(pos.castle[1], "white queenside right should be untouched");
}

#[test]
fn capturing_white_queenside_rook_on_a1_revokes_white_queenside_right() {
	// A black piece captures the White a1 rook → castle[1] (white queenside)
	// should be cleared.
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.castle = [true, true, false, false];

	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[8] = Some(black_rook); // a2
	pos.board[0] = Some(white_rook); // a1

	let m = mv(&pos, 8, 0, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert!(pos.castle[0], "white kingside right should be untouched");
	assert!(!pos.castle[1], "white queenside right should be revoked when a1 rook is captured");
}

#[test]
fn castling_revokes_both_rights_for_that_side() {
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king);
	pos.board[7] = Some(rook);

	let m = mv(&pos, 4, 6, MoveKind::Castling { rook_from: 7, rook_to: 5 });
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[0], "white kingside right should be revoked after castling");
	assert!(!pos.castle[1], "white queenside right should be revoked after castling");
}

// ── king_squares tracking ─────────────────────────────────────────────────────

#[test]
fn king_squares_updated_after_white_king_moves() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	pos.board[4] = Some(king); // e1

	let m = mv(&pos, 4, 5, MoveKind::Quiet); // e1 → f1
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.king_squares[0], 5, "white king square should be f1");
	assert_eq!(pos.king_squares[1], 60, "black king square should be unchanged");
}

#[test]
fn king_squares_updated_after_black_king_moves() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.king_squares = [4, 60];

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	pos.board[60] = Some(king); // e8

	let m = mv(&pos, 60, 61, MoveKind::Quiet); // e8 → f8
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.king_squares[0], 4, "white king square should be unchanged");
	assert_eq!(pos.king_squares[1], 61, "black king square should be f8");
}

#[test]
fn king_squares_updated_after_castling() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];
	pos.castle[0] = true;

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king);
	pos.board[7] = Some(rook);

	let m = mv(&pos, 4, 6, MoveKind::Castling { rook_from: 7, rook_to: 5 });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.king_squares[0], 6, "white king square should be g1 after kingside castle");
}

#[test]
fn non_king_move_does_not_update_king_squares() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[0] = Some(rook);

	let m = mv(&pos, 0, 7, MoveKind::Quiet);
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.king_squares[0], 4, "white king square should not change on rook move");
	assert_eq!(pos.king_squares[1], 60, "black king square should not change on rook move");
}

// ── undo tests ────────────────────────────────────────────────────────────────

#[test]
fn quiet_move_undo_restores_board() {
	let mut pos = empty_position();

	let knight = ColoredPiece { piece: Piece::Knight, side: Side::White };
	pos.board[1] = Some(knight); // b1

	let m = mv(&pos, 1, 18, MoveKind::Quiet); // b1 → c3
	let undo = pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[1], None);
	assert_eq!(pos.board[18], Some(knight));

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[1], Some(knight));
	assert_eq!(pos.board[18], None);
}

#[test]
fn capture_undo_restores_both_pieces() {
	let mut pos = empty_position();

	let white_queen = ColoredPiece { piece: Piece::Queen, side: Side::White };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[3] = Some(white_queen); // d1
	pos.board[59] = Some(black_rook); // d8

	let m = mv(&pos, 3, 59, MoveKind::Capture);
	let undo = pos.make_move_unvalidated(m).unwrap();

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[3], Some(white_queen));
	assert_eq!(pos.board[59], Some(black_rook));
}

#[test]
fn double_pawn_push_undo_restores_position() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[51] = Some(pawn); // d7

	let m = mv(&pos, 51, 35, MoveKind::DoublePawnPush { passed_square: 43 });
	let undo = pos.make_move_unvalidated(m).unwrap();

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[51], Some(pawn));
	assert_eq!(pos.board[35], None);
}

#[test]
fn castling_undo_restores_king_and_rook() {
	let mut pos = empty_position();
	pos.castle[0] = true;

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king); // e1
	pos.board[7] = Some(rook); // h1

	let m = mv(&pos, 4, 6, MoveKind::Castling { rook_from: 7, rook_to: 5 });
	let undo = pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[4], None);
	assert_eq!(pos.board[6], Some(king));
	assert_eq!(pos.board[7], None);
	assert_eq!(pos.board[5], Some(rook));

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[4], Some(king));
	assert_eq!(pos.board[6], None);
	assert_eq!(pos.board[7], Some(rook));
	assert_eq!(pos.board[5], None);
}

#[test]
fn undo_restores_side_to_move() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[8] = Some(pawn);

	let m = mv(&pos, 8, 16, MoveKind::Quiet);
	let undo = pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.side_to_move, Side::Black);

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.side_to_move, Side::White);
}

// BUG: undo_move does not restore halfmove_clock (it is incremented by
// make_move but never restored on undo).  Once undo_move saves/restores
#[test]
fn undo_restores_all_metadata() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.en_passant = Some(16);
	pos.castle = [true, false, true, false];
	pos.king_squares = [4, 60];
	pos.fullmove_counter = 42;
	pos.halfmove_clock = 7;

	let knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[57] = Some(knight); // b8

	let m = mv(&pos, 57, 42, MoveKind::Quiet);
	let undo = pos.make_move_unvalidated(m).unwrap();

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.side_to_move, Side::Black);
	assert_eq!(pos.en_passant, Some(16));
	assert_eq!(pos.castle, [true, false, true, false]);
	assert_eq!(pos.king_squares, [4, 60]);
	assert_eq!(pos.fullmove_counter, 42);
	assert_eq!(pos.halfmove_clock, 7);
}

// ── undo — WIP branches ───────────────────────────────────────────────────────

#[test]
fn en_passant_undo_restores_all_squares() {
	let mut pos = empty_position();

	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[33] = Some(white_pawn); // b5
	pos.board[34] = Some(black_pawn); // c5
	pos.en_passant = Some(42); // c6

	let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
	let undo = pos.make_move_unvalidated(m).unwrap();

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[33], Some(white_pawn), "white pawn should be restored to b5");
	assert_eq!(pos.board[34], Some(black_pawn), "captured black pawn should be restored to c5");
	assert_eq!(pos.board[42], None, "c6 should be empty after undo");
}

#[test]
fn promotion_undo_restores_original_pawn() {
	let mut pos = empty_position();

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[48] = Some(pawn); // a7

	let m = mv(&pos, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Knight });
	let undo = pos.make_move_unvalidated(m).unwrap();

	pos.undo_move(undo, m).unwrap();

	assert_eq!(pos.board[48], Some(pawn), "original pawn should be restored to a7");
	assert_eq!(pos.board[56], None, "a8 should be empty after undo");
}

#[test]
fn promotion_capture_changes_piece_and_captures() {
	let mut pos = empty_position();

	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[48] = Some(white_pawn); // a7
	pos.board[57] = Some(black_rook); // b8

	// Promote to Queen while capturing the rook
	let m = mv(&pos, 48, 57, MoveKind::Promotion { promotion_piece: Piece::Queen });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[48], None, "a7 should be empty");
	assert_eq!(pos.board[57], Some(ColoredPiece { piece: Piece::Queen, side: Side::White }), "b8 should have white queen");
}

#[test]
fn promotion_capture_all_piece_types() {
	// Test that all four promotion piece types work with captures
	for (piece_type, idx) in [
		(Piece::Queen, 0),
		(Piece::Rook, 1),
		(Piece::Bishop, 2),
		(Piece::Knight, 3),
	] {
		let mut pos = empty_position();

		let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
		let black_bishop = ColoredPiece { piece: Piece::Bishop, side: Side::Black };
		pos.board[48] = Some(white_pawn); // a7
		pos.board[57] = Some(black_bishop); // b8

		let m = mv(&pos, 48, 57, MoveKind::Promotion { promotion_piece: piece_type });
		pos.make_move_unvalidated(m).unwrap();

		assert_eq!(pos.board[48], None, "a7 should be empty (test {idx})");
		assert_eq!(pos.board[57], Some(ColoredPiece { piece: piece_type, side: Side::White }), "b8 should have white {piece_type:?} (test {idx})");
	}
}

#[test]
fn promotion_capture_undo_restores_both_pieces() {
	let mut pos = empty_position();

	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[48] = Some(white_pawn); // a7
	pos.board[57] = Some(black_knight); // b8

	let m = mv(&pos, 48, 57, MoveKind::Promotion { promotion_piece: Piece::Rook });
	let undo = pos.make_move_unvalidated(m).unwrap();

	// Verify promotion + capture happened
	assert_eq!(pos.board[48], None);
	assert_eq!(pos.board[57], Some(ColoredPiece { piece: Piece::Rook, side: Side::White }));

	// Undo the move
	pos.undo_move(undo, m).unwrap();

	// Verify both pieces are restored
	assert_eq!(pos.board[48], Some(white_pawn), "white pawn should be restored to a7");
	assert_eq!(pos.board[57], Some(black_knight), "black knight should be restored to b8");
}

#[test]
fn black_promotion_capture_works() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	let white_queen = ColoredPiece { piece: Piece::Queen, side: Side::White };
	pos.board[15] = Some(black_pawn); // h2
	pos.board[6] = Some(white_queen); // g1

	let m = mv(&pos, 15, 6, MoveKind::Promotion { promotion_piece: Piece::Bishop });
	pos.make_move_unvalidated(m).unwrap();

	assert_eq!(pos.board[15], None, "h2 should be empty");
	assert_eq!(pos.board[6], Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }), "g1 should have black bishop");
}

#[test]
fn capturing_black_kingside_rook_on_h8_revokes_black_kingside_right() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	// White rook captures the Black h8 rook → castle[2] (black kingside) cleared.
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[55] = Some(white_rook); // h7
	pos.board[63] = Some(black_rook); // h8

	let m = mv(&pos, 55, 63, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert!(!pos.castle[2], "black kingside right should be revoked when h8 rook is captured");
	assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
fn capturing_black_queenside_rook_on_a8_revokes_black_queenside_right() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	// White rook captures the Black a8 rook → castle[3] (black queenside) cleared.
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	pos.board[48] = Some(white_rook); // a7
	pos.board[56] = Some(black_rook); // a8

	let m = mv(&pos, 48, 56, MoveKind::Capture);
	pos.make_move_unvalidated(m).unwrap();

	assert!(pos.castle[2], "black kingside right should be untouched");
	assert!(!pos.castle[3], "black queenside right should be revoked when a8 rook is captured");
}

// ══════════════════════════════════════════════════════════════════════════════
// Individual helper function tests
// ══════════════════════════════════════════════════════════════════════════════

// ── apply_move_to_board tests ─────────────────────────────────────────────────

#[test]
fn apply_move_to_board_quiet_move() {
	let mut pos = empty_position();
	let knight = ColoredPiece { piece: Piece::Knight, side: Side::White };
	pos.board[1] = Some(knight);

	let m = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: knight,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, knight, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[1], None);
	assert_eq!(pos.board[18], Some(knight));
	assert_eq!(undo.captured_piece, None);
}

#[test]
fn apply_move_to_board_capture() {
	let mut pos = empty_position();
	let white_bishop = ColoredPiece { piece: Piece::Bishop, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[10] = Some(white_bishop);
	pos.board[28] = Some(black_pawn);

	let m = Move {
		from_square: 10,
		to_square: 28,
		move_kind: MoveKind::Capture,
		colored_piece: white_bishop,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, white_bishop, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[10], None);
	assert_eq!(pos.board[28], Some(white_bishop));
	assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn apply_move_to_board_en_passant() {
	let mut pos = empty_position();
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	pos.board[33] = Some(white_pawn); // b5
	pos.board[34] = Some(black_pawn); // c5

	let m = Move {
		from_square: 33,
		to_square: 42,
		move_kind: MoveKind::EnPassant { capture_square: 34 },
		colored_piece: white_pawn,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, white_pawn, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[33], None);
	assert_eq!(pos.board[34], None);
	assert_eq!(pos.board[42], Some(white_pawn));
	assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn apply_move_to_board_promotion() {
	let mut pos = empty_position();
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[48] = Some(white_pawn); // a7

	let m = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: white_pawn,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, white_pawn, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[48], None);
	assert_eq!(pos.board[56], Some(ColoredPiece { piece: Piece::Queen, side: Side::White }));
	assert_eq!(undo.captured_piece, None);
}

#[test]
fn apply_move_to_board_promotion_capture() {
	let mut pos = empty_position();
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_knight = ColoredPiece { piece: Piece::Knight, side: Side::Black };
	pos.board[48] = Some(white_pawn); // a7
	pos.board[57] = Some(black_knight); // b8

	let m = Move {
		from_square: 48,
		to_square: 57,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Rook },
		colored_piece: white_pawn,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, white_pawn, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[48], None);
	assert_eq!(pos.board[57], Some(ColoredPiece { piece: Piece::Rook, side: Side::White }));
	assert_eq!(undo.captured_piece, Some(black_knight));
}

#[test]
fn apply_move_to_board_castling() {
	let mut pos = empty_position();
	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	pos.board[4] = Some(king); // e1
	pos.board[7] = Some(rook); // h1

	let m = Move {
		from_square: 4,
		to_square: 6,
		move_kind: MoveKind::Castling { rook_from: 7, rook_to: 5 },
		colored_piece: king,
	};
	let mut undo = pos.build_undo();

	pos.apply_move_to_board(m, king, &mut undo, zobrist()).unwrap();

	assert_eq!(pos.board[4], None);
	assert_eq!(pos.board[6], Some(king));
	assert_eq!(pos.board[7], None);
	assert_eq!(pos.board[5], Some(rook));
}

#[test]
fn apply_move_to_board_rejects_pawn_promotion() {
	let mut pos = empty_position();
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	pos.board[48] = Some(white_pawn);

	let m = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Pawn },
		colored_piece: white_pawn,
	};
	let mut undo = pos.build_undo();

	assert!(matches!(pos.apply_move_to_board(m, white_pawn, &mut undo, zobrist()), Err(ChessError::PromotionPieceCantBePawn)));
}

// ── update_en_passant tests ───────────────────────────────────────────────────

#[test]
fn update_en_passant_sets_on_double_pawn_push() {
	let mut pos = empty_position();
	pos.en_passant = None;

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let m = Move {
		from_square: 8,
		to_square: 24,
		move_kind: MoveKind::DoublePawnPush { passed_square: 16 },
		colored_piece: pawn,
	};

	pos.update_en_passant(m);

	assert_eq!(pos.en_passant, Some(16));
}

#[test]
fn update_en_passant_clears_on_quiet_move() {
	let mut pos = empty_position();
	pos.en_passant = Some(42);

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 0,
		to_square: 7,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.update_en_passant(m);

	assert_eq!(pos.en_passant, None);
}

#[test]
fn update_en_passant_clears_on_capture() {
	let mut pos = empty_position();
	pos.en_passant = Some(16);

	let bishop = ColoredPiece { piece: Piece::Bishop, side: Side::White };
	let m = Move {
		from_square: 10,
		to_square: 28,
		move_kind: MoveKind::Capture,
		colored_piece: bishop,
	};

	pos.update_en_passant(m);

	assert_eq!(pos.en_passant, None);
}

// ── update_clocks tests ───────────────────────────────────────────────────────

#[test]
fn update_clocks_resets_halfmove_on_pawn_move() {
	let mut pos = empty_position();
	pos.halfmove_clock = 5;

	let pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let m = Move {
		from_square: 8,
		to_square: 16,
		move_kind: MoveKind::Quiet,
		colored_piece: pawn,
	};

	pos.update_clocks_and_side(m, zobrist());

	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn update_clocks_resets_halfmove_on_capture() {
	let mut pos = empty_position();
	pos.halfmove_clock = 3;

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 0,
		to_square: 7,
		move_kind: MoveKind::Capture,
		colored_piece: rook,
	};

	pos.update_clocks_and_side(m, zobrist());

	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn update_clocks_increments_halfmove_on_quiet_non_pawn() {
	let mut pos = empty_position();
	pos.halfmove_clock = 2;

	let knight = ColoredPiece { piece: Piece::Knight, side: Side::White };
	let m = Move {
		from_square: 1,
		to_square: 18,
		move_kind: MoveKind::Quiet,
		colored_piece: knight,
	};

	pos.update_clocks_and_side(m, zobrist());

	assert_eq!(pos.halfmove_clock, 3);
}

#[test]
fn update_clocks_toggles_side_white_to_black() {
	let mut pos = empty_position();
	pos.side_to_move = Side::White;

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 0,
		to_square: 7,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.update_clocks_and_side(m, zobrist());

	assert_eq!(pos.side_to_move, Side::Black);
}

#[test]
fn update_clocks_toggles_side_black_to_white_and_increments_fullmove() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;
	pos.fullmove_counter = 10;

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let m = Move {
		from_square: 56,
		to_square: 57,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.update_clocks_and_side(m, zobrist());

	assert_eq!(pos.side_to_move, Side::White);
	assert_eq!(pos.fullmove_counter, 11);
}

// ── update_king_positions tests ───────────────────────────────────────────────

#[test]
fn update_king_positions_white_king() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let m = Move {
		from_square: 4,
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: king,
	};

	pos.update_king_positions(m);

	assert_eq!(pos.king_squares[0], 5);
	assert_eq!(pos.king_squares[1], 60);
}

#[test]
fn update_king_positions_black_king() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let m = Move {
		from_square: 60,
		to_square: 61,
		move_kind: MoveKind::Quiet,
		colored_piece: king,
	};

	pos.update_king_positions(m);

	assert_eq!(pos.king_squares[0], 4);
	assert_eq!(pos.king_squares[1], 61);
}

#[test]
fn update_king_positions_non_king_does_not_update() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 0,
		to_square: 7,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.update_king_positions(m);

	assert_eq!(pos.king_squares[0], 4);
	assert_eq!(pos.king_squares[1], 60);
}

// ── set_castle_rights tests ───────────────────────────────────────────────────

#[test]
fn set_castle_rights_white_king_from_e1_revokes_both() {
	let mut pos = empty_position();
	pos.castle = [true, true, true, true];

	let king = ColoredPiece { piece: Piece::King, side: Side::White };
	let m = Move {
		from_square: 4, // e1
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: king,
	};

	pos.set_castle_rights(m);

	assert!(!pos.castle[0], "white kingside should be revoked");
	assert!(!pos.castle[1], "white queenside should be revoked");
	assert!(pos.castle[2], "black kingside should be untouched");
	assert!(pos.castle[3], "black queenside should be untouched");
}

#[test]
fn set_castle_rights_black_king_from_e8_revokes_both() {
	let mut pos = empty_position();
	pos.castle = [true, true, true, true];

	let king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let m = Move {
		from_square: 60, // e8
		to_square: 61,
		move_kind: MoveKind::Quiet,
		colored_piece: king,
	};

	pos.set_castle_rights(m);

	assert!(pos.castle[0], "white kingside should be untouched");
	assert!(pos.castle[1], "white queenside should be untouched");
	assert!(!pos.castle[2], "black kingside should be revoked");
	assert!(!pos.castle[3], "black queenside should be revoked");
}

#[test]
fn set_castle_rights_h1_rook_from_revokes_white_kingside() {
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 7, // h1
		to_square: 6,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(!pos.castle[0], "white kingside should be revoked");
	assert!(pos.castle[1], "white queenside should be untouched");
}

#[test]
fn set_castle_rights_a1_rook_from_revokes_white_queenside() {
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 0, // a1
		to_square: 1,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(pos.castle[0], "white kingside should be untouched");
	assert!(!pos.castle[1], "white queenside should be revoked");
}

#[test]
fn set_castle_rights_h8_rook_from_revokes_black_kingside() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let m = Move {
		from_square: 63, // h8
		to_square: 62,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(!pos.castle[2], "black kingside should be revoked");
	assert!(pos.castle[3], "black queenside should be untouched");
}

#[test]
fn set_castle_rights_a8_rook_from_revokes_black_queenside() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let m = Move {
		from_square: 56, // a8
		to_square: 57,
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(pos.castle[2], "black kingside should be untouched");
	assert!(!pos.castle[3], "black queenside should be revoked");
}

#[test]
fn set_castle_rights_capture_on_h1_revokes_white_kingside() {
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let m = Move {
		from_square: 15,
		to_square: 7, // h1
		move_kind: MoveKind::Capture,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(!pos.castle[0], "white kingside should be revoked");
	assert!(pos.castle[1], "white queenside should be untouched");
}

#[test]
fn set_castle_rights_capture_on_a1_revokes_white_queenside() {
	let mut pos = empty_position();
	pos.castle = [true, true, false, false];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };
	let m = Move {
		from_square: 8,
		to_square: 0, // a1
		move_kind: MoveKind::Capture,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(pos.castle[0], "white kingside should be untouched");
	assert!(!pos.castle[1], "white queenside should be revoked");
}

#[test]
fn set_castle_rights_capture_on_h8_revokes_black_kingside() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 55,
		to_square: 63, // h8
		move_kind: MoveKind::Capture,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(!pos.castle[2], "black kingside should be revoked");
	assert!(pos.castle[3], "black queenside should be untouched");
}

#[test]
fn set_castle_rights_capture_on_a8_revokes_black_queenside() {
	let mut pos = empty_position();
	pos.castle = [false, false, true, true];

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let m = Move {
		from_square: 48,
		to_square: 56, // a8
		move_kind: MoveKind::Capture,
		colored_piece: rook,
	};

	pos.set_castle_rights(m);

	assert!(pos.castle[2], "black kingside should be untouched");
	assert!(!pos.castle[3], "black queenside should be revoked");
}

// ══════════════════════════════════════════════════════════════════════════════
// Game::make_move — history tracking
// ══════════════════════════════════════════════════════════════════════════════

// Square helpers for readability:
//   a1=0 … h1=7 | a2=8 … h2=15 | … | a8=56 … h8=63

#[test]
fn make_move_pushes_to_hash_history() {
	// After a successful move the current position hash must be recorded.
	// Position: white rook a1, kings on e1/e8. White plays Ra1-a2.
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");
	assert_eq!(game.hash_history.len(), 0);

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let mv = Move {
		from_square: 0, // a1
		to_square: 8,   // a2
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};
	game.make_move(&mv).unwrap();

	assert_eq!(game.hash_history.len(), 1, "hash_history should have one entry after one move");
}

#[test]
fn make_move_pushes_to_move_history() {
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let mv = Move {
		from_square: 0, // a1
		to_square: 8,   // a2
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};
	game.make_move(&mv).unwrap();

	assert_eq!(game.move_history.len(), 1, "move_history should have one entry after one move");
	assert_eq!(game.move_history[0], mv);
}

#[test]
fn make_move_pushes_to_undo_history() {
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let mv = Move {
		from_square: 0, // a1
		to_square: 8,   // a2
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};
	game.make_move(&mv).unwrap();

	assert_eq!(game.undo_history.len(), 1, "undo_history should have one entry after one move");
}

#[test]
fn make_move_hash_in_history_matches_position_hash() {
	// The hash stored in hash_history must equal the position hash after the move.
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");

	let rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let mv = Move {
		from_square: 0, // a1
		to_square: 8,   // a2
		move_kind: MoveKind::Quiet,
		colored_piece: rook,
	};
	game.make_move(&mv).unwrap();

	assert_eq!(game.hash_history[0], game.position.zobrist_hash, "hash_history entry must equal the position's zobrist hash after the move",);
}

// ══════════════════════════════════════════════════════════════════════════════
// Game::make_move — game-over guard
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn make_move_rejected_when_game_is_stalemate() {
	let mut game = empty_game();
	game.game_status = GameStatus::Stalemate;

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	game.position.board[4] = Some(white_king); // e1

	let mv = Move {
		from_square: 4,
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	assert!(matches!(game.make_move(&mv), Err(ChessError::GameIsFinished)), "make_move must return GameIsFinished when the game is drawn",);
}

#[test]
fn make_move_rejected_when_game_is_checkmate() {
	let mut game = empty_game();
	game.game_status = GameStatus::CheckmateForSide(Side::Black);

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	game.position.board[4] = Some(white_king); // e1

	let mv = Move {
		from_square: 4,
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	assert!(matches!(game.make_move(&mv), Err(ChessError::GameIsFinished)), "make_move must return GameIsFinished when the game is already over",);
}

#[test]
fn make_move_rejected_when_game_is_draw_by_fifty_moves() {
	let mut game = empty_game();
	game.game_status = GameStatus::DrawByFiftyMoves;

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	game.position.board[4] = Some(white_king);

	let mv = Move {
		from_square: 4,
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	assert!(matches!(game.make_move(&mv), Err(ChessError::GameIsFinished)));
}

// ══════════════════════════════════════════════════════════════════════════════
// Game::make_move — game status update after move
// These tests verify update_game_status() is called after each move.
// ══════════════════════════════════════════════════════════════════════════════

/// Fool's mate (fastest checkmate in chess):
/// 1.f3 e5 2.g4 Qh4#
/// After black plays Qd8-h4 the white king on e1 is completely surrounded:
///   d1=own queen, d2=own pawn, e2=own pawn, f1=own bishop, f2 attacked by Qh4.
#[test]
fn fools_mate_sets_status_to_checkmate() {
	// FEN after 1.f3 e5 2.g4 — black to move, one move before checkmate.
	let mut game = game_from_fen("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2");

	let black_queen = ColoredPiece { piece: Piece::Queen, side: Side::Black };
	// Qd8-h4: d8=59, h4=31
	let mv = Move {
		from_square: 59, // d8
		to_square: 31,   // h4
		move_kind: MoveKind::Quiet,
		colored_piece: black_queen,
	};
	game.make_move(&mv).unwrap();

	// White is checkmated; Black wins → CheckmateForSide(Black).
	assert!(matches!(game.game_status, GameStatus::CheckmateForSide(Side::Black)), "after Fool's Mate, game status must be CheckmateForSide(Black) — Black wins",);
}

/// After a move that puts the opponent in check, game_status must be InCheck.
/// Position: white Ke1(4), Qd5(35), black Ke8(60). White plays Qd5-e6(44).
/// Queen on e6 attacks king on e8 along the e-file (e7 is empty).
#[test]
fn move_giving_check_sets_status_to_in_check() {
	let mut game = game_from_fen("4k3/8/8/3Q4/8/8/8/4K3 w - - 0 1");

	let white_queen = ColoredPiece { piece: Piece::Queen, side: Side::White };
	// Qd5-e6: d5=35, e6=44
	let mv = Move {
		from_square: 35, // d5
		to_square: 44,   // e6
		move_kind: MoveKind::Quiet,
		colored_piece: white_queen,
	};
	game.make_move(&mv).unwrap();

	assert!(matches!(game.game_status, GameStatus::InCheck));
}

/// Stalemate: white Kf6(45), Qg5(38), black Kh8(63). White plays Qg5-g6(46).
/// After Qg6 the black king on h8 has no legal move and is not in check:
///   g8(62) attacked by queen along g-file; h7(55) attacked by queen on the g6-h7 diagonal;
///   g7(54) attacked by white king on f6.
#[test]
fn move_causing_stalemate_sets_status_to_stalemate() {
	let mut game = game_from_fen("7k/8/5K2/6Q1/8/8/8/8 w - - 0 1");

	let white_queen = ColoredPiece { piece: Piece::Queen, side: Side::White };
	// Qg5-g6: g5=38, g6=46
	let mv = Move {
		from_square: 38, // g5
		to_square: 46,   // g6
		move_kind: MoveKind::Quiet,
		colored_piece: white_queen,
	};
	game.make_move(&mv).unwrap();

	assert!(matches!(game.game_status, GameStatus::Stalemate));
}

/// Fifty-move draw: the halfmove clock starts at 99; after one quiet king move it reaches 100.
/// Position: white Ke1(4), black Ke8(60). White plays Ke1-f1(5).
#[test]
fn quiet_move_at_halfmove_99_triggers_fifty_move_draw() {
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/4K3 w - - 99 1");

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	// Ke1-f1: e1=4, f1=5
	let mv = Move {
		from_square: 4, // e1
		to_square: 5,   // f1
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	game.make_move(&mv).unwrap();

	assert!(matches!(game.game_status, GameStatus::DrawByFiftyMoves));
}

/// Three-fold repetition with two bare kings.
/// Moves: 1.Ke1-f1 Ke8-d8 2.Kf1-e1 Kd8-e8 3.Ke1-f1
/// After move 5 the hash from move 1 (Kf1, Kd8, black to move) appears a
/// second time in hash_history, so is_draw_by_repetition returns true.
#[test]
fn threefold_repetition_triggers_draw_by_repetition() {
	let mut game = game_from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };

	let ke1_f1 = Move {
		from_square: 4,
		to_square: 5,
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	let ke8_d8 = Move {
		from_square: 60,
		to_square: 59,
		move_kind: MoveKind::Quiet,
		colored_piece: black_king,
	};
	let kf1_e1 = Move {
		from_square: 5,
		to_square: 4,
		move_kind: MoveKind::Quiet,
		colored_piece: white_king,
	};
	let kd8_e8 = Move {
		from_square: 59,
		to_square: 60,
		move_kind: MoveKind::Quiet,
		colored_piece: black_king,
	};

	game.make_move(&ke1_f1).unwrap(); // 1. Ke1-f1
	game.make_move(&ke8_d8).unwrap(); // 1... Ke8-d8
	game.make_move(&kf1_e1).unwrap(); // 2. Kf1-e1
	game.make_move(&kd8_e8).unwrap(); // 2... Kd8-e8
	game.make_move(&ke1_f1).unwrap(); // 3. Ke1-f1 — hash from move 1 seen again

	assert!(matches!(game.game_status, GameStatus::DrawByRepetition));
}
