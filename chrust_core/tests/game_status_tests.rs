mod common;

use chrust_core::moves::make_move::MoveKind;
use chrust_core::{ColoredPiece, Piece, Side};
use common::empty_position;

// ══════════════════════════════════════════════════════════════════════════════
// is_draw_by_fifty_moves tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn is_draw_by_fifty_moves_with_number_over_100() {
	let mut pos = empty_position();
	pos.halfmove_clock = 150;

	assert!(pos.is_draw_by_fifty_moves(), "halfmove_clock 150 should be a draw");
}

#[test]
fn is_draw_by_fifty_moves_with_number_under_100() {
	let mut pos = empty_position();
	pos.halfmove_clock = 50;

	assert!(!pos.is_draw_by_fifty_moves(), "halfmove_clock 50 should not be a draw");
}

#[test]
fn is_draw_by_fifty_moves_with_exactly_100() {
	let mut pos = empty_position();
	pos.halfmove_clock = 100;

	assert!(pos.is_draw_by_fifty_moves(), "halfmove_clock 100 should be a draw (fifty-move rule)");
}

#[test]
fn is_draw_by_fifty_moves_with_zero() {
	let pos = empty_position();
	// halfmove_clock defaults to 0

	assert!(!pos.is_draw_by_fifty_moves(), "halfmove_clock 0 should not be a draw");
}

#[test]
fn is_draw_by_fifty_moves_boundary_99() {
	let mut pos = empty_position();
	pos.halfmove_clock = 99;

	assert!(!pos.is_draw_by_fifty_moves(), "halfmove_clock 99 should not be a draw yet");
}

#[test]
fn is_draw_by_fifty_moves_boundary_101() {
	let mut pos = empty_position();
	pos.halfmove_clock = 101;

	assert!(pos.is_draw_by_fifty_moves(), "halfmove_clock 101 should be a draw");
}

// ══════════════════════════════════════════════════════════════════════════════
// get_all_legal_moves_for_side tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn get_all_legal_moves_for_side_white_multiple_pieces() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[12] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });
	pos.board[19] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.king_squares = [4, 60];

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should have moves from king, rook, and knight combined
	assert!(!moves.is_empty(), "white should have legal moves");

	// Verify moves from different pieces are included
	let has_king_moves = moves.iter().any(|m| m.from_square == 4);
	let has_rook_moves = moves.iter().any(|m| m.from_square == 12);
	let has_knight_moves = moves.iter().any(|m| m.from_square == 19);

	assert!(has_king_moves, "should include king moves");
	assert!(has_rook_moves, "should include rook moves");
	assert!(has_knight_moves, "should include knight moves");
}

#[test]
fn get_all_legal_moves_for_side_black_multiple_pieces() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.board[52] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black });
	pos.board[43] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black });
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::Black;

	let moves = pos.get_all_legal_moves_for_side(Side::Black).expect("get_all_legal_moves failed");

	// Should have moves from black king, bishop, and queen
	assert!(!moves.is_empty(), "black should have legal moves");

	let has_bishop_moves = moves.iter().any(|m| m.from_square == 52);
	let has_queen_moves = moves.iter().any(|m| m.from_square == 43);

	assert!(has_bishop_moves, "should include bishop moves");
	assert!(has_queen_moves, "should include queen moves");
}

#[test]
fn get_all_legal_moves_for_side_different_position() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.king_squares = [27, 60];

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Verify we get moves from both king and pawn
	let has_king_moves = moves.iter().any(|m| m.from_square == 27);
	let has_pawn_moves = moves.iter().any(|m| m.from_square == 35);

	assert!(has_king_moves, "should include king moves from d4");
	assert!(has_pawn_moves, "should include pawn moves from d5");
}

#[test]
fn get_all_legal_moves_for_side_no_moves_stalemate() {
	// Position: white king on a1, black king on c2, black queen on c1 -> stalemate
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.board[9] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // b2
	pos.board[2] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // c1
	pos.king_squares = [0, 9];
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	assert_eq!(moves.len(), 0, "stalemate position should have no legal moves");
}

#[test]
fn get_all_legal_moves_for_side_no_moves_checkmate() {
	// Back rank mate: white king on h1, black rook on h8, black rook on a1, white pawns blocking
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // h1
	pos.board[63] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // h8
	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // a1
	pos.board[14] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // g2
	pos.board[15] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // h2
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [7, 56];
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	assert_eq!(moves.len(), 0, "checkmate position should have no legal moves");
}

#[test]
fn get_all_legal_moves_for_side_in_check_with_escape_moves() {
	// White king on e1 in check from rook on e8, but can move to safety
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// King should have escape moves (d1, f1, d2, f2)
	assert!(!moves.is_empty(), "king in check should have escape moves");
	assert!(moves.iter().all(|m| m.from_square == 4), "only king moves should be legal when in check");
}

#[test]
fn get_all_legal_moves_for_side_in_check_with_blocking_move() {
	// White king on e1 in check from rook on e8, white rook can block
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[11] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // d2
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should have king escape moves AND rook blocking move to e2
	assert!(!moves.is_empty(), "should have moves to escape check");

	let has_blocking_move = moves.iter().any(|m| m.from_square == 11 && m.to_square == 12);
	assert!(has_blocking_move, "should include blocking move d2-e2");
}

#[test]
fn get_all_legal_moves_for_side_empty_board_for_side() {
	// Only black pieces on board, check white's moves
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.board[52] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });
	pos.king_squares = [4, 60]; // White king position but no piece there

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	assert_eq!(moves.len(), 0, "side with no pieces should have no moves");
}

#[test]
fn get_all_legal_moves_for_side_only_king_remaining() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.king_squares = [4, 60];

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// King should have 5 legal moves (e2, d1, f1, d2, f2)
	assert_eq!(moves.len(), 5, "lone king should have 5 moves from e1");
	assert!(moves.iter().all(|m| m.from_square == 4), "all moves should be king moves");
}

#[test]
fn get_all_legal_moves_for_side_with_pinned_piece() {
	// White rook pinned on e2, can only move along e-file
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should include pinned rook's moves (only along e-file)
	let pinned_rook_moves: Vec<_> = moves.iter().filter(|m| m.from_square == 12).collect();
	assert!(!pinned_rook_moves.is_empty(), "pinned rook should still have legal moves along pin line");

	// Verify pinned rook can't move horizontally
	let has_horizontal_move = moves.iter().any(|m| m.from_square == 12 && (m.to_square == 11 || m.to_square == 13));
	assert!(!has_horizontal_move, "pinned rook should not be able to move horizontally");
}

#[test]
fn get_all_legal_moves_for_side_double_check_only_king_moves() {
	// Double check: only king can move
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e2 (could block single check)
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8
	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // a1
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// In double check, ONLY king moves are legal (can't block two attacks)
	assert!(moves.iter().all(|m| m.from_square == 4), "in double check, only king moves should be legal");
	assert!(moves.iter().all(|m| m.colored_piece.piece == Piece::King), "all moves should be king moves");
}

#[test]
fn get_all_legal_moves_for_side_with_en_passant_available() {
	// White pawn can capture black pawn en passant
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[36] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e5
	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // d5
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.en_passant = Some(43); // d6 (en passant target square)
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should include en passant capture
	let has_en_passant = moves.iter().any(|m| m.from_square == 36 && m.to_square == 43 && matches!(m.move_kind, MoveKind::EnPassant { .. }));
	assert!(has_en_passant, "should include en passant move");
}

#[test]
fn get_all_legal_moves_for_side_with_castling_available() {
	// White can castle kingside
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[7] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // h1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.castle = [true, false, false, false]; // White can castle kingside
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should include kingside castling
	let has_castling = moves.iter().any(|m| m.from_square == 4 && m.to_square == 6 && matches!(m.move_kind, MoveKind::Castling { .. }));
	assert!(has_castling, "should include kingside castling move");
}

#[test]
fn get_all_legal_moves_for_side_with_promotion_available() {
	// White pawn on 7th rank can promote
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[52] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e7
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];
	pos.side_to_move = Side::White;

	let moves = pos.get_all_legal_moves_for_side(Side::White).expect("get_all_legal_moves failed");

	// Should include promotion moves (4 promotions: Q, R, B, N)
	let promotion_moves: Vec<_> = moves.iter().filter(|m| m.from_square == 52 && matches!(m.move_kind, MoveKind::Promotion { .. })).collect();

	assert_eq!(promotion_moves.len(), 4, "should have 4 promotion moves (Q, R, B, N)");
}

// ══════════════════════════════════════════════════════════════════════════════
// is_checkmate_for_side tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn is_checkmate_for_side_white_not_in_check() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.king_squares = [4, 60];

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(!is_mate, "position without check should not be checkmate");
}

#[test]
fn is_checkmate_for_side_black_not_in_check() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.king_squares = [4, 60];

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(!is_mate, "position without check should not be checkmate");
}

#[test]
fn is_checkmate_for_side_white_in_check_can_escape() {
	// White king in check but can move to safety
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(!is_mate, "check with escape moves available is not checkmate");
}

#[test]
fn is_checkmate_for_side_black_in_check_can_block() {
	// Black king in check from white rook on e1, black rook can block on e7
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e1 (checking)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[3] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // d1 (can block on e-file)
	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.king_squares = [0, 60];
	pos.side_to_move = Side::Black;

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(!is_mate, "check with blocking move available is not checkmate");
}

#[test]
fn is_checkmate_for_side_white_in_check_can_capture() {
	// White king can capture attacking piece
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e2 (checking, but capturable)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(!is_mate, "check with capture available is not checkmate");
}

#[test]
fn is_checkmate_for_side_white_stalemate_not_checkmate() {
	// White has no moves but not in check (stalemate, not checkmate)
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.board[16] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a3
	pos.king_squares = [0, 16];

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(!is_mate, "stalemate (no check, no moves) is not checkmate");
}

#[test]
fn is_checkmate_for_side_black_stalemate_not_checkmate() {
	// Black has no moves but not in check
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[45] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // f6
	pos.board[47] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // h6
	pos.king_squares = [45, 63];

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(!is_mate, "stalemate is not checkmate");
}

#[test]
fn is_checkmate_for_side_white_back_rank_mate() {
	// Simple checkmate: white king on e1, black queen on e2, black rook on e8 - king has no escape
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // e2 (checking)
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8 (controls e-file)
	pos.board[1] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // b1 (controls d1/f1)
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];
	pos.side_to_move = Side::White;

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(is_mate, "checkmate should be detected");
}

#[test]
fn is_checkmate_for_side_black_back_rank_mate() {
	// Simple checkmate: black king on e8, white queen on e7, white rook on e1 - king has no escape
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // e7 (checking)
	pos.board[4] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e1 (controls e-file)
	pos.board[57] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // b8 (controls d8/f8)
	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.king_squares = [0, 60];
	pos.side_to_move = Side::Black;

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(is_mate, "checkmate should be detected");
}

#[test]
fn is_checkmate_for_side_white_smothered_mate() {
	// King on e1, black queen on h4 checking (protected by bishop), pawn blocks e2
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e2
	pos.board[31] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // h4 (checking e1 along diagonal)
	pos.board[54] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // g7 (protects queen)
	pos.board[3] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // d1
	pos.board[5] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // f1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::White;

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(is_mate, "checkmate should be detected");
}

#[test]
fn is_checkmate_for_side_white_queen_and_king_mate() {
	// Queen and king vs lone king checkmate
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.board[8] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // a2
	pos.board[16] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a3
	pos.king_squares = [0, 16];

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(is_mate, "queen and king mate should be checkmate");
}

#[test]
fn is_checkmate_for_side_black_rook_and_king_mate_on_edge() {
	// Rook and king vs lone king on edge: king on h8, white king on g6, white rook on a8
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[56] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // a8 (checking)
	pos.board[46] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // g6
	pos.king_squares = [46, 63];

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(is_mate, "rook and king mate on edge should be checkmate");
}

#[test]
fn is_checkmate_for_side_white_double_check_checkmate() {
	// King on e1, black queen on e2 checking (protected), can't escape or capture
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // e2 (checking)
	pos.board[20] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e3 (protects queen)
	pos.board[3] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // d1
	pos.board[5] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // f1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::White;

	let is_mate = pos.is_checkmate_for_side(Side::White).expect("is_checkmate failed");

	assert!(is_mate, "checkmate should be detected");
}

#[test]
fn is_checkmate_for_side_black_two_bishops_mate() {
	// Two bishops mate: black king on h8, white bishops on f6 and g6 checking, white king on f7
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[45] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // f6 (checking)
	pos.board[46] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // g6
	pos.board[53] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // f7
	pos.king_squares = [53, 63];
	pos.side_to_move = Side::Black;

	let is_mate = pos.is_checkmate_for_side(Side::Black).expect("is_checkmate failed");

	assert!(is_mate, "two bishops mate should be checkmate");
}

// ══════════════════════════════════════════════════════════════════════════════
// is_stalemate_for_side tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn is_stalemate_for_side_white_classic_corner_stalemate() {
	// White king on a1, black king on b3, black queen on c2 - classic stalemate
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.board[17] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // b3
	pos.board[10] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // c2
	pos.king_squares = [0, 17];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(is_stalemate, "white should be in stalemate");
}

#[test]
fn is_stalemate_for_side_black_classic_corner_stalemate() {
	// Black king on h8, white king on g6, white queen on f7 - classic stalemate
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[46] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // g6
	pos.board[53] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // f7
	pos.king_squares = [46, 63];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(is_stalemate, "black should be in stalemate");
}

#[test]
fn is_stalemate_for_side_white_in_check_with_no_moves_not_stalemate() {
	// White king in checkmate - should return false (it's checkmate, not stalemate)
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // e2 (checking)
	pos.board[20] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e3 (protects queen)
	pos.board[3] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // d1
	pos.board[5] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // f1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(!is_stalemate, "checkmate position should not be stalemate");
}

#[test]
fn is_stalemate_for_side_black_in_check_with_no_moves_not_stalemate() {
	// Black king in checkmate - should return false
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // e7 (checking)
	pos.board[4] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e1 (controls e-file)
	pos.board[57] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // b8 (controls d8/f8)
	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.king_squares = [0, 60];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(!is_stalemate, "checkmate position should not be stalemate");
}

#[test]
fn is_stalemate_for_side_white_not_in_check_with_moves_available() {
	// White has moves available - should return false
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(!is_stalemate, "position with available moves should not be stalemate");
}

#[test]
fn is_stalemate_for_side_black_not_in_check_with_moves_available() {
	// Black has moves available - should return false
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(!is_stalemate, "position with available moves should not be stalemate");
}

#[test]
fn is_stalemate_for_side_white_in_check_with_moves_available() {
	// White in check but can escape - should return false
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // e8 (checking)
	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // a8
	pos.king_squares = [4, 56];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(!is_stalemate, "check with escape moves should not be stalemate");
}

#[test]
fn is_stalemate_for_side_black_in_check_with_moves_available() {
	// Black in check but can escape - should return false
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[4] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // e1 (checking)
	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.king_squares = [0, 60];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(!is_stalemate, "check with escape moves should not be stalemate");
}

#[test]
fn is_stalemate_for_side_white_pawn_stalemate() {
	// White king on a8, white pawn on a7, black king on c7 - white is stalemated
	let mut pos = empty_position();

	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a8
	pos.board[48] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // a7
	pos.board[50] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // c7
	pos.king_squares = [56, 50];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(is_stalemate, "white should be in stalemate (blocked by own pawn)");
}

#[test]
fn is_stalemate_for_side_black_pawn_stalemate() {
	// Black king on h1, black pawn on h2, white king on f2 - black is stalemated
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h1
	pos.board[15] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // h2
	pos.board[13] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // f2
	pos.king_squares = [13, 7];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(is_stalemate, "black should be in stalemate (blocked by own pawn)");
}

#[test]
fn is_stalemate_for_side_white_king_vs_multiple_pieces() {
	// White has only king, surrounded but not in check - stalemate
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1
	pos.board[17] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // b3
	pos.board[10] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // c2
	pos.king_squares = [0, 17];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(is_stalemate, "white king trapped should be stalemate");
}

#[test]
fn is_stalemate_for_side_black_king_vs_multiple_pieces() {
	// Black has only king, surrounded but not in check - stalemate
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[46] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // g6
	pos.board[53] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // f7
	pos.king_squares = [46, 63];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(is_stalemate, "black king trapped should be stalemate");
}

#[test]
fn is_stalemate_for_side_white_rook_stalemate_pattern() {
	// White king on a8, black king on c7, black queen on b6 - stalemate
	let mut pos = empty_position();

	pos.board[56] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a8
	pos.board[50] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // c7
	pos.board[41] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // b6
	pos.king_squares = [56, 50];
	pos.side_to_move = Side::White;

	let is_stalemate = pos.is_stalemate_for_side(Side::White).expect("is_stalemate failed");

	assert!(is_stalemate, "white should be in stalemate");
}

#[test]
fn is_stalemate_for_side_black_rook_stalemate_pattern() {
	// Black king on h1, white king on f2, white queen on g3 - stalemate
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h1
	pos.board[13] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // f2
	pos.board[22] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // g3
	pos.king_squares = [13, 7];
	pos.side_to_move = Side::Black;

	let is_stalemate = pos.is_stalemate_for_side(Side::Black).expect("is_stalemate failed");

	assert!(is_stalemate, "black should be in stalemate");
}

// ══════════════════════════════════════════════════════════════════════════════
// is_insufficient_material tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn is_insufficient_material_only_kings() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king vs king should be insufficient material");
}

#[test]
fn is_insufficient_material_king_and_bishop_vs_king_white() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king + bishop vs king should be insufficient material");
}

#[test]
fn is_insufficient_material_king_and_bishop_vs_king_black() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // e7
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king vs king + bishop should be insufficient material");
}

#[test]
fn is_insufficient_material_two_bishops_same_side() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // e2 (light square)
	pos.board[13] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // f2 (dark square)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + 2 bishops vs king should be sufficient material");
}

#[test]
fn is_insufficient_material_two_bishops_different_sides_same_square_color() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[0] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // a1 (dark square: (0+0)%2 = 0)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[63] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // h8 (dark square: (7+7)%2 = 0)
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king + bishop vs king + bishop (same color) should be insufficient material");
}

#[test]
fn is_insufficient_material_two_bishops_different_sides_different_square_color() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[0] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // a1 (dark square: (0+0)%2 = 0)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[56] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // a8 (light square: (0+7)%2 = 1)
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + bishop vs king + bishop (different color) should be sufficient material");
}

#[test]
fn is_insufficient_material_king_and_knight_vs_king_white() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king + knight vs king should be insufficient material");
}

#[test]
fn is_insufficient_material_king_and_knight_vs_king_black() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // e7
	pos.king_squares = [4, 60];

	assert!(pos.is_insufficient_material(), "king vs king + knight should be insufficient material");
}

#[test]
fn is_insufficient_material_one_knight_each_side() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // e7
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + knight vs king + knight should be sufficient material");
}

#[test]
fn is_insufficient_material_two_knights_same_side_white() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[13] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // f2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + 2 knights vs king should be sufficient material");
}

#[test]
fn is_insufficient_material_two_knights_same_side_black() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // e7
	pos.board[53] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // f7
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king vs king + 2 knights should be sufficient material");
}

#[test]
fn is_insufficient_material_knight_and_bishop_same_side_white() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[13] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // f2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + knight + bishop vs king should be sufficient material");
}

#[test]
fn is_insufficient_material_knight_and_bishop_same_side_black() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // e7
	pos.board[53] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // f7
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king vs king + knight + bishop should be sufficient material");
}

#[test]
fn is_insufficient_material_knight_and_bishop_different_sides() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // e7
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + knight vs king + bishop should be sufficient material");
}

#[test]
fn is_insufficient_material_with_pawn() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "pawn present should be sufficient material");
}

#[test]
fn is_insufficient_material_with_rook() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // a1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "rook present should be sufficient material");
}

#[test]
fn is_insufficient_material_with_queen() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[3] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // d1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "queen present should be sufficient material");
}

#[test]
fn is_insufficient_material_with_multiple_pieces() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White }); // a1
	pos.board[1] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // b1
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[63] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // h8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "multiple pieces should be sufficient material");
}

#[test]
fn is_insufficient_material_three_knights_same_side() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // e2
	pos.board[13] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // f2
	pos.board[14] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // g2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + 3 knights vs king should be sufficient material");
}

#[test]
fn is_insufficient_material_multiple_bishops_same_side() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[0] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // a1 (dark)
	pos.board[2] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // c1 (dark)
	pos.board[5] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // f1 (light)
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + 3 bishops vs king should be sufficient material");
}

#[test]
fn is_insufficient_material_king_and_bishop_vs_king_and_knight() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // e1
	pos.board[12] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }); // e2
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // e8
	pos.board[52] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // e7
	pos.king_squares = [4, 60];

	assert!(!pos.is_insufficient_material(), "king + bishop vs king + knight should be sufficient material");
}
