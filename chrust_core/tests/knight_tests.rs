mod common;

use chrust_core::errors::ChessError;
use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::{ColoredPiece, Piece, Side, Square};
use common::{empty_position, has_move, has_to_square};

#[test]
fn knight_e4_empty_board() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[28] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let moves = pos.knight_targets(28).expect("knight_targets returned Err");

	assert_eq!(moves.len(), 8);

	assert!(has_move(&moves, 28, 43, MoveKind::Quiet));
	assert!(has_move(&moves, 28, 45, MoveKind::Quiet));
	assert!(has_move(&moves, 28, 11, MoveKind::Quiet));
	assert!(has_move(&moves, 28, 22, MoveKind::Quiet));
}

#[test]
fn knight_a8_corner_test() {
	let mut pos = empty_position();

	pos.board[56] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let moves = pos.knight_targets(56).expect("knight_targets returned Err");

	assert_eq!(moves.len(), 2);

	assert!(has_move(&moves, 56, 50, MoveKind::Quiet));
	assert!(has_move(&moves, 56, 41, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 39));
	assert!(!has_to_square(&moves, 16));
}

#[test]
fn knight_g8_enemy_h6() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[62] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	pos.board[47] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let moves = pos.knight_targets(62).expect("knight_targets returned Err");

	assert_eq!(moves.len(), 3);

	assert!(has_move(&moves, 62, 47, MoveKind::Capture));
}

#[test]
fn knight_d1_friendly_f2() {
	let mut pos = empty_position();

	pos.board[3] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	pos.board[13] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let moves = pos.knight_targets(3).expect("knight_targets returned Err");

	assert_eq!(moves.len(), 3);

	assert!(!has_to_square(&moves, 13));
}

#[test]
fn wrong_piece_e8() {
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::White });

	assert_eq!(
		pos.knight_targets(60),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Knight,
			found_piece: Piece::King
		})
	);
}

#[test]
fn no_piece_d5() {
	let pos = empty_position();

	assert_eq!(pos.knight_targets(35), Err(ChessError::NoPieceOnSquare { square: 35 }))
}

#[test]
fn try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(pos.knight_targets(65), Err(ChessError::NotASquareOnBoard { square: 65 }))
}

#[test]
fn wrong_side_returns_wrong_side_error() {
	// Black knight on the board but it's White's turn.
	let mut pos = empty_position(); // side_to_move = White

	pos.board[28] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	assert_eq!(
		pos.knight_targets(28),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn white_knight_e4_empty_board() {
	let mut pos = empty_position(); // side_to_move = White

	pos.board[28] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let moves = pos.knight_targets(28).expect("knight_targets returned Err");

	assert_eq!(moves.len(), 8);
	// Verify all 8 L-shapes from e4 (sq 28)
	assert!(has_move(&moves, 28, 45, MoveKind::Quiet)); // f6  +17
	assert!(has_move(&moves, 28, 43, MoveKind::Quiet)); // d6  +15
	assert!(has_move(&moves, 28, 38, MoveKind::Quiet)); // g5  +10
	assert!(has_move(&moves, 28, 34, MoveKind::Quiet)); // c5  +6
	assert!(has_move(&moves, 28, 22, MoveKind::Quiet)); // g3  -6
	assert!(has_move(&moves, 28, 18, MoveKind::Quiet)); // c3  -10
	assert!(has_move(&moves, 28, 13, MoveKind::Quiet)); // f2  -15
	assert!(has_move(&moves, 28, 11, MoveKind::Quiet)); // d2  -17
}
