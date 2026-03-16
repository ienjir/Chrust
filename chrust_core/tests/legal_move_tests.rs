mod common;

use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::{ColoredPiece, Piece, Side, errors::ChessError};
use common::{empty_position, has_move, has_to_square};

fn mv(from_square: u8, to_square: u8, move_kind: MoveKind, colored_piece: ColoredPiece) -> Move {
	Move {
		from_square,
		to_square,
		move_kind,
		colored_piece,
	}
}

#[test]
fn pinned_rook_only_gets_moves_along_the_pin_line() {
	let mut pos = empty_position();

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };

	pos.board[4] = Some(white_king); // e1
	pos.board[12] = Some(white_rook); // e2
	pos.board[56] = Some(black_king); // a8
	pos.board[60] = Some(black_rook); // e8
	pos.king_squares = [4, 56];

	let moves = pos.get_legal_moves(12, pos.side_to_move).expect("get_legal_moves returned Err");

	assert_eq!(moves.len(), 6);
	assert!(has_move(&moves, 12, 20, MoveKind::Quiet));
	assert!(has_move(&moves, 12, 28, MoveKind::Quiet));
	assert!(has_move(&moves, 12, 36, MoveKind::Quiet));
	assert!(has_move(&moves, 12, 44, MoveKind::Quiet));
	assert!(has_move(&moves, 12, 52, MoveKind::Quiet));
	assert!(has_move(&moves, 12, 60, MoveKind::Capture));
	assert!(!has_to_square(&moves, 11));
	assert!(!has_to_square(&moves, 13));
}

#[test]
fn checked_rook_only_gets_blocking_move() {
	let mut pos = empty_position();

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };

	pos.board[4] = Some(white_king); // e1
	pos.board[11] = Some(white_rook); // d2
	pos.board[56] = Some(black_king); // a8
	pos.board[60] = Some(black_rook); // e8
	pos.king_squares = [4, 56];

	let moves = pos.get_legal_moves(11, pos.side_to_move).expect("get_legal_moves returned Err");

	assert_eq!(moves.len(), 1);
	assert!(has_move(&moves, 11, 12, MoveKind::Quiet));
}

#[test]
fn king_in_check_cannot_step_forward_into_attacked_square() {
	let mut pos = empty_position();

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };

	pos.board[4] = Some(white_king); // e1
	pos.board[56] = Some(black_king); // a8
	pos.board[60] = Some(black_rook); // e8
	pos.king_squares = [4, 56];

	let moves = pos.get_legal_moves(4, pos.side_to_move).expect("get_legal_moves returned Err");

	assert_eq!(moves.len(), 4);
	assert!(has_move(&moves, 4, 3, MoveKind::Quiet));
	assert!(has_move(&moves, 4, 5, MoveKind::Quiet));
	assert!(has_move(&moves, 4, 11, MoveKind::Quiet));
	assert!(has_move(&moves, 4, 13, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 12));
}

#[test]
fn en_passant_is_filtered_out_when_it_exposes_the_king() {
	let mut pos = empty_position();

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let black_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };

	pos.board[4] = Some(white_king); // e1
	pos.board[36] = Some(white_pawn); // e5
	pos.board[35] = Some(black_pawn); // d5
	pos.board[56] = Some(black_king); // a8
	pos.board[60] = Some(black_rook); // e8
	pos.en_passant = Some(43); // d6
	pos.king_squares = [4, 56];

	let moves = pos.get_legal_moves(36, pos.side_to_move).expect("get_legal_moves returned Err");

	assert_eq!(moves.len(), 1);
	assert!(has_move(&moves, 36, 44, MoveKind::Quiet));
	assert!(!has_move(&moves, 36, 43, MoveKind::EnPassant { capture_square: 35 }));
}

#[test]
fn make_move_rejects_pinned_rook_sidestep() {
	let mut pos = empty_position();

	let white_king = ColoredPiece { piece: Piece::King, side: Side::White };
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let black_king = ColoredPiece { piece: Piece::King, side: Side::Black };
	let black_rook = ColoredPiece { piece: Piece::Rook, side: Side::Black };

	pos.board[4] = Some(white_king); // e1
	pos.board[12] = Some(white_rook); // e2
	pos.board[56] = Some(black_king); // a8
	pos.board[60] = Some(black_rook); // e8
	pos.king_squares = [4, 56];

	let illegal_move = mv(12, 13, MoveKind::Quiet, white_rook);

	assert!(matches!(pos.make_move(&illegal_move), Err(ChessError::NotAValidMove)));
}
