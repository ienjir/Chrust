use super::*;
use crate::errors::ChessError;
use crate::helper::{file_diff, in_bounds};
use crate::moves::make_move::MoveKind;
use crate::test_common::{empty_position, has_move, has_to_square};
use crate::{ColoredPiece, Piece, Side};

#[test]
fn w_pawn_c2() {
	let mut pos = empty_position();

	pos.board[10] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let pawn = pos.board[10].unwrap();
	let moves = pos.pawn_targets(pawn, 10).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 2);

	assert!(has_move(&moves, 10, 18, MoveKind::Quiet));
	assert!(has_move(&moves, 10, 26, MoveKind::DoublePawnPush { passed_square: 18 }));
	assert!(!has_to_square(&moves, 17));
	assert!(!has_to_square(&moves, 19));
}

#[test]
fn b_pawn_c7() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[50] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let pawn = pos.board[50].unwrap();
	let moves = pos.pawn_targets(pawn, 50).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 2);

	assert!(has_move(&moves, 50, 42, MoveKind::Quiet));
	assert!(has_move(&moves, 50, 34, MoveKind::DoublePawnPush { passed_square: 42 }));
	assert!(!has_to_square(&moves, 41));
	assert!(!has_to_square(&moves, 43));
}

#[test]
fn w_pawn_e4_enemy_f5_friendly_d5() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	pos.board[37] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	pos.board[35] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let pawn = pos.board[28].unwrap();
	let moves = pos.pawn_targets(pawn, 28).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 2);

	assert!(has_move(&moves, 28, 36, MoveKind::Quiet));
	assert!(has_move(&moves, 28, 37, MoveKind::Capture));
	assert!(!has_to_square(&moves, 35));
}

#[test]
fn w_pawn_d2_blocked_by_piece_d3() {
	let mut pos = empty_position();

	pos.board[11] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	pos.board[19] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let pawn = pos.board[11].unwrap();
	let moves = pos.pawn_targets(pawn, 11).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 0);
}

#[test]
fn w_pawn_d2_blocked_on_double_move() {
	let mut pos = empty_position();

	pos.board[11] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	pos.board[27] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let pawn = pos.board[11].unwrap();
	let moves = pos.pawn_targets(pawn, 11).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 1);
	assert!(has_move(&moves, 11, 19, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 27));
}

#[test]
fn w_pawn_a2_edge_capture_b3() {
	let mut pos = empty_position();

	pos.board[8] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	pos.board[17] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let pawn = pos.board[8].unwrap();
	let moves = pos.pawn_targets(pawn, 8).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 3);
	assert!(has_move(&moves, 8, 16, MoveKind::Quiet));
	assert!(has_move(&moves, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 }));
	assert!(has_move(&moves, 8, 17, MoveKind::Capture));
	assert!(!has_to_square(&moves, 15));
}

#[test]
fn b_pawn_h7_edge_capture_g6() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[55] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	pos.board[46] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let pawn = pos.board[55].unwrap();
	let moves = pos.pawn_targets(pawn, 55).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 3);
	assert!(has_move(&moves, 55, 47, MoveKind::Quiet));
	assert!(has_move(&moves, 55, 39, MoveKind::DoublePawnPush { passed_square: 47 }));
	assert!(has_move(&moves, 55, 46, MoveKind::Capture));
	assert!(!has_to_square(&moves, 48));
}

#[test]
fn pawn_wrong_piece_e2() {
	let mut pos = empty_position();

	pos.board[12] = Some(ColoredPiece { piece: Piece::King, side: Side::White });

	assert_eq!(
		pos.get_validated_colored_piece(12, Piece::Pawn),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Pawn,
			found_piece: Piece::King
		})
	);
}

#[test]
fn pawn_no_piece_e2() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(12), Err(ChessError::NoPieceOnSquare { square: 12 }))
}

#[test]
fn pawn_try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(65), Err(ChessError::NotASquareOnBoard { square: 65 }))
}

#[test]
fn pawn_wrong_side_returns_wrong_side_error() {
	// Black pawn on the board but it's White's turn → WrongSide.
	let mut pos = empty_position(); // side_to_move = White

	pos.board[50] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	assert_eq!(
		pos.get_validated_colored_piece(50, Piece::Pawn),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn w_pawn_e5_en_passant_d6() {
	let mut pos = empty_position();

	pos.board[36] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.en_passant = Some(43);

	let pawn = pos.board[36].unwrap();
	let moves = pos.pawn_targets(pawn, 36).expect("pawn_targets returned Err");

	assert!(has_move(&moves, 36, 44, MoveKind::Quiet));
	assert!(has_move(&moves, 36, 43, MoveKind::EnPassant { capture_square: 35 }));
}

#[test]
fn b_pawn_d4_en_passant_c3() {
	// Black pawn on d4 (sq 27), en passant square = c3 (sq 18).
	// Captured pawn is on c4 (sq 26).
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[27] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.en_passant = Some(18); // c3

	let pawn = pos.board[27].unwrap();
	let moves = pos.pawn_targets(pawn, 27).expect("pawn_targets returned Err");

	assert!(has_move(
		&moves,
		27,
		18,
		MoveKind::EnPassant { capture_square: 26 } // c4, black side: ep_sq + 8
	));
}

#[test]
fn w_pawn_a7_promotion() {
	// White pawn on a7 (sq 48), target a8 (sq 56) is empty.
	// Generator emits Promotion with Pawn sentinel.
	let mut pos = empty_position();

	pos.board[48] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let pawn = pos.board[48].unwrap();
	let moves = pos.pawn_targets(pawn, 48).expect("pawn_targets returned Err");

	// The generator now emits 4 separate moves, one for each promotion piece type
	assert_eq!(moves.len(), 4);
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Queen }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Rook }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Bishop }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Knight }));
}

#[test]
fn b_pawn_h2_promotion() {
	// Black pawn on h2 (sq 15), target h1 (sq 7) is empty.
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[15] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let pawn = pos.board[15].unwrap();
	let moves = pos.pawn_targets(pawn, 15).expect("pawn_targets returned Err");

	// The generator now emits 4 separate moves, one for each promotion piece type
	assert_eq!(moves.len(), 4);
	assert!(has_move(&moves, 15, 7, MoveKind::Promotion { promotion_piece: Piece::Queen }));
	assert!(has_move(&moves, 15, 7, MoveKind::Promotion { promotion_piece: Piece::Rook }));
	assert!(has_move(&moves, 15, 7, MoveKind::Promotion { promotion_piece: Piece::Bishop }));
	assert!(has_move(&moves, 15, 7, MoveKind::Promotion { promotion_piece: Piece::Knight }));
}

#[test]
fn w_pawn_b7_promotion_capture_on_a8() {
	// White pawn on b7 (sq 49), enemy on a8 (sq 56).
	// The diagonal capture on the last rank is emitted as MoveKind::Capture
	// (the generator does not emit a combined promotion-capture kind).
	// The quiet promotion to b8 (sq 57) is also available.
	let mut pos = empty_position();

	pos.board[49] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[56] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let pawn = pos.board[49].unwrap();
	let moves = pos.pawn_targets(pawn, 49).expect("pawn_targets returned Err");

	// Should have 8 total moves: 4 quiet promotions to b8 + 4 promotion captures to a8
	assert_eq!(moves.len(), 8);

	// Quiet promotions to b8 (sq 57)
	for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
		assert!(has_move(&moves, 49, 57, MoveKind::Promotion { promotion_piece: piece }));
	}

	// Promotion captures on a8 (sq 56)
	for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
		assert!(has_move(&moves, 49, 56, MoveKind::Promotion { promotion_piece: piece }));
	}
}

#[test]
fn w_pawn_a7_promotion_blocked() {
	// White pawn on a7 (sq 48) with a piece on a8 (sq 56) — no quiet promotion.
	let mut pos = empty_position();

	pos.board[48] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[56] = Some(ColoredPiece {
		piece: Piece::Rook,
		side: Side::White, // own piece blocking
	});

	let pawn = pos.board[48].unwrap();
	let moves = pos.pawn_targets(pawn, 48).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 0);
}

#[test]
fn b_pawn_d7_blocked_on_transit_square() {
	// Symmetric of the White double-push-blocked test.
	// Black pawn on d7 (sq 51), friendly on d6 (sq 43) — no moves at all.
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[51] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.board[43] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let pawn = pos.board[51].unwrap();
	let moves = pos.pawn_targets(pawn, 51).expect("pawn_targets returned Err");

	assert_eq!(moves.len(), 0);
}

// Tests for helper functions

#[test]
fn test_in_bounds() {
	assert!(in_bounds(0));
	assert!(in_bounds(32));
	assert!(in_bounds(63));
	assert!(!in_bounds(-1));
	assert!(!in_bounds(64));
	assert!(!in_bounds(100));
}

#[test]
fn test_file_diff() {
	// Same file
	assert_eq!(file_diff(0, 8), 0); // a1 to a2
	assert_eq!(file_diff(7, 15), 0); // h1 to h2

	// Adjacent files
	assert_eq!(file_diff(1, 0), 1); // b1 to a1
	assert_eq!(file_diff(0, 1), 1); // a1 to b1

	// Diagonal moves
	assert_eq!(file_diff(9, 0), 1); // b2 to a1
	assert_eq!(file_diff(7, 0), 7); // h1 to a1

	// Wrap-around edge case (should be 7, not 1)
	assert_eq!(file_diff(8, 7), 7); // a2 to h1 (files 0 and 7)
}

#[test]
fn test_promotion_moves() {
	let colored_piece = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let mut moves = Vec::new();

	promotion_moves(&mut moves, colored_piece, 48, 56); // a7 to a8

	assert_eq!(moves.len(), 4);
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Queen }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Rook }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Bishop }));
	assert!(has_move(&moves, 48, 56, MoveKind::Promotion { promotion_piece: Piece::Knight }));
}
