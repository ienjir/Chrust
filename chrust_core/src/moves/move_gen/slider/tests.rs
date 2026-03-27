use crate::errors::ChessError;
use crate::moves::make_move::MoveKind;
use crate::test_common::{empty_position, has_move, has_to_square};
use crate::{ColoredPiece, Piece, Side};

// ── Bishop tests ──────────────────────────────────────────────────────────────

#[test]
fn bishop_g7_empty_boad() {
	let mut pos = empty_position();

	pos.board[54] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	let piece = pos.board[54].unwrap();
	let moves = pos.slider_targets(piece, 54).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 9);

	assert!(has_move(&moves, 54, 63, MoveKind::Quiet));
	assert!(has_move(&moves, 54, 61, MoveKind::Quiet));
	assert!(has_move(&moves, 54, 47, MoveKind::Quiet));
	assert!(has_move(&moves, 54, 27, MoveKind::Quiet));
	assert!(has_move(&moves, 54, 0, MoveKind::Quiet));
}

#[test]
fn bishop_h7_corner_test() {
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	let piece = pos.board[7].unwrap();
	let moves = pos.slider_targets(piece, 7).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 7);

	assert!(has_move(&moves, 7, 14, MoveKind::Quiet));
	assert!(has_move(&moves, 7, 56, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 8));
	assert!(!has_to_square(&moves, 16));
	assert!(!has_to_square(&moves, 0));
}

#[test]
fn bishop_c7_enemy_f4() {
	let mut pos = empty_position();

	pos.board[50] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	pos.board[29] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let piece = pos.board[50].unwrap();
	let moves = pos.slider_targets(piece, 50).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 7);

	assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
	assert!(has_move(&moves, 50, 29, MoveKind::Capture));
	assert!(!has_to_square(&moves, 22));
}

#[test]
fn bishop_b3_friendly_e6() {
	let mut pos = empty_position();

	pos.board[17] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	pos.board[44] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let piece = pos.board[17].unwrap();
	let moves = pos.slider_targets(piece, 17).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 6);

	assert!(has_move(&moves, 17, 35, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 44));
}

#[test]
fn bishop_wrong_piece_e8() {
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	assert_eq!(
		pos.get_validated_colored_piece(60, Piece::Bishop),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Bishop,
			found_piece: Piece::Knight
		})
	);
}

#[test]
fn bishop_no_piece_d5() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(35), Err(ChessError::NoPieceOnSquare { square: 35 }))
}

#[test]
fn bishop_try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(65), Err(ChessError::NotASquareOnBoard { square: 65 }))
}

#[test]
fn bishop_wrong_side_returns_wrong_side_error() {
	// Black bishop on the board but it's White's turn.
	let mut pos = empty_position(); // side_to_move = White

	pos.board[0] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black });

	assert_eq!(
		pos.get_validated_colored_piece(0, Piece::Bishop),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn black_bishop_d4_empty_board() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[27] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	// d4 bishop on an open board has 13 diagonal moves.
	assert_eq!(moves.len(), 13);
	assert!(has_move(&moves, 27, 0, MoveKind::Quiet)); // a1
	assert!(has_move(&moves, 27, 54, MoveKind::Quiet)); // g7
	assert!(has_move(&moves, 27, 6, MoveKind::Quiet)); // g1
	assert!(has_move(&moves, 27, 48, MoveKind::Quiet)); // a7
}

#[test]
fn bishop_a3_does_not_wrap_around_board_edge() {
	// Bishop on a3 (sq 16) should not wrap to h2 or h4
	let mut pos = empty_position();

	pos.board[16] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	let piece = pos.board[16].unwrap();
	let moves = pos.slider_targets(piece, 16).expect("slider_targets returned Err");

	// Should not wrap to h-file
	assert!(!has_to_square(&moves, 15)); // h2
	assert!(!has_to_square(&moves, 23)); // h3

	// Should contain valid diagonal moves
	assert!(has_move(&moves, 16, 9, MoveKind::Quiet)); // b2
	assert!(has_move(&moves, 16, 25, MoveKind::Quiet)); // b4
}

#[test]
fn bishop_h6_does_not_wrap_around_board_edge() {
	// Bishop on h6 (sq 47) should not wrap to a5 or a7
	let mut pos = empty_position();

	pos.board[47] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	let piece = pos.board[47].unwrap();
	let moves = pos.slider_targets(piece, 47).expect("slider_targets returned Err");

	// Should not wrap to a-file
	assert!(!has_to_square(&moves, 32)); // a5
	assert!(!has_to_square(&moves, 48)); // a7

	// Should contain valid diagonal moves
	assert!(has_move(&moves, 47, 38, MoveKind::Quiet)); // g5
	assert!(has_move(&moves, 47, 54, MoveKind::Quiet)); // g7
}

// ── Rook tests ────────────────────────────────────────────────────────────────

#[test]
fn rook_h8_empty_board() {
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	let piece = pos.board[63].unwrap();
	let moves = pos.slider_targets(piece, 63).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14);

	assert!(has_move(&moves, 63, 62, MoveKind::Quiet));
	assert!(has_move(&moves, 63, 56, MoveKind::Quiet));
	assert!(has_move(&moves, 63, 55, MoveKind::Quiet));
	assert!(has_move(&moves, 63, 7, MoveKind::Quiet));
}

#[test]
fn rook_d4_empty_board() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14);

	assert!(has_move(&moves, 27, 24, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 31, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 3, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 26, MoveKind::Quiet));
}

#[test]
fn rook_d4_blocked_by_friendly_piece_f4() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[29] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 29));
	assert!(!has_to_square(&moves, 30));
}

#[test]
fn rook_d4_captures_enemy_f4_and_stops() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[29] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 29, MoveKind::Capture));
	assert!(!has_to_square(&moves, 30));
}

#[test]
fn rook_a1_empty_board() {
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	let piece = pos.board[0].unwrap();
	let moves = pos.slider_targets(piece, 0).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14);

	assert!(has_move(&moves, 0, 1, MoveKind::Quiet));
	assert!(has_move(&moves, 0, 7, MoveKind::Quiet));
	assert!(has_move(&moves, 0, 8, MoveKind::Quiet));
	assert!(has_move(&moves, 0, 56, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 63));
}

#[test]
fn rook_d4_blocked_by_adjacent_friendly_pieces() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[19] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[26] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[28] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert!(moves.is_empty());
}

#[test]
fn rook_d4_captures_adjacent_enemy_pieces() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.board[19] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.board[26] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.board[28] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 4);
	assert!(has_move(&moves, 27, 35, MoveKind::Capture));
	assert!(has_move(&moves, 27, 19, MoveKind::Capture));
	assert!(has_move(&moves, 27, 26, MoveKind::Capture));
	assert!(has_move(&moves, 27, 28, MoveKind::Capture));
}

#[test]
fn rook_d4_mixed_blockers() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[43] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[25] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 10);

	assert!(has_move(&moves, 27, 35, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 43));
	assert!(has_move(&moves, 27, 19, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 11, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 3, MoveKind::Quiet));

	assert!(has_move(&moves, 27, 26, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 25, MoveKind::Capture));
	assert!(!has_to_square(&moves, 24));

	assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
	assert!(has_move(&moves, 27, 31, MoveKind::Quiet));
}

#[test]
fn rook_wrong_piece_e8() {
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	assert_eq!(
		pos.get_validated_colored_piece(60, Piece::Rook),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Rook,
			found_piece: Piece::Knight
		})
	);
}

#[test]
fn rook_no_piece_d5() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(35), Err(ChessError::NoPieceOnSquare { square: 35 }))
}

#[test]
fn rook_try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(65), Err(ChessError::NotASquareOnBoard { square: 65 }))
}

#[test]
fn rook_wrong_side_returns_wrong_side_error() {
	// Black rook on the board but it's White's turn.
	let mut pos = empty_position(); // side_to_move = White

	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });

	assert_eq!(
		pos.get_validated_colored_piece(0, Piece::Rook),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn black_rook_d4_empty_board() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[27] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14);
	assert!(has_move(&moves, 27, 3, MoveKind::Quiet)); // d1
	assert!(has_move(&moves, 27, 59, MoveKind::Quiet)); // d8
	assert!(has_move(&moves, 27, 24, MoveKind::Quiet)); // a4
	assert!(has_move(&moves, 27, 31, MoveKind::Quiet)); // h4
}

#[test]
fn rook_a4_does_not_wrap_to_h_file() {
	// Rook on a4 (sq 24) moving left should wrap check prevent going to h3
	let mut pos = empty_position();

	pos.board[24] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	let piece = pos.board[24].unwrap();
	let moves = pos.slider_targets(piece, 24).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14); // 7 vertical + 7 horizontal

	// Should not wrap to previous rank
	assert!(!has_to_square(&moves, 23)); // h3 - wrong rank
	assert!(!has_to_square(&moves, 15)); // h2 - wrong rank

	// Should contain valid moves along the rank
	assert!(has_move(&moves, 24, 25, MoveKind::Quiet)); // b4
	assert!(has_move(&moves, 24, 31, MoveKind::Quiet)); // h4 - valid same rank
}

#[test]
fn rook_h4_does_not_wrap_to_a_file() {
	// Rook on h4 (sq 31) moving right should wrap check prevent going to a5
	let mut pos = empty_position();

	pos.board[31] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	let piece = pos.board[31].unwrap();
	let moves = pos.slider_targets(piece, 31).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 14); // 7 vertical + 7 horizontal

	// Should not wrap to next rank
	assert!(!has_to_square(&moves, 32)); // a5 - next rank, would be wrap
	assert!(!has_to_square(&moves, 40)); // a6 - next rank

	// Should contain valid moves along the rank
	assert!(has_move(&moves, 31, 30, MoveKind::Quiet)); // g4
	assert!(has_move(&moves, 31, 24, MoveKind::Quiet)); // a4 - valid same rank
}

// ── Queen tests ───────────────────────────────────────────────────────────────

#[test]
fn queen_d4_empty_board() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 27);

	assert!(has_move(&moves, 27, 31, MoveKind::Quiet)); // h4
	assert!(has_move(&moves, 27, 3, MoveKind::Quiet)); // d1
	assert!(has_move(&moves, 27, 63, MoveKind::Quiet)); // h8
	assert!(has_move(&moves, 27, 18, MoveKind::Quiet)); // c3
	assert!(has_move(&moves, 27, 36, MoveKind::Quiet)); // e5
}

#[test]
fn queen_d4_blocked_by_friendly_piece_f4() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White });

	pos.board[29] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 29));
	assert!(!has_to_square(&moves, 30));
}

#[test]
fn queen_c7_enemy_f4() {
	let mut pos = empty_position();

	pos.board[50] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White });

	pos.board[29] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let piece = pos.board[50].unwrap();
	let moves = pos.slider_targets(piece, 50).expect("slider_targets returned Err");

	assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
	assert!(has_move(&moves, 50, 29, MoveKind::Capture));
	assert!(!has_to_square(&moves, 22));
}

#[test]
fn queen_wrong_piece_e8() {
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	assert_eq!(
		pos.get_validated_colored_piece(60, Piece::Queen),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Queen,
			found_piece: Piece::Knight
		})
	);
}

#[test]
fn queen_no_piece_d5() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(35), Err(ChessError::NoPieceOnSquare { square: 35 }))
}

#[test]
fn queen_try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(65), Err(ChessError::NotASquareOnBoard { square: 65 }))
}

#[test]
fn queen_wrong_side_returns_wrong_side_error() {
	// Black queen on the board but it's White's turn.
	let mut pos = empty_position(); // side_to_move = White

	pos.board[27] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black });

	assert_eq!(
		pos.get_validated_colored_piece(27, Piece::Queen),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn black_queen_d4_empty_board() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[27] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black });

	let piece = pos.board[27].unwrap();
	let moves = pos.slider_targets(piece, 27).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 27);
	assert!(has_move(&moves, 27, 3, MoveKind::Quiet)); // d1 orthogonal
	assert!(has_move(&moves, 27, 31, MoveKind::Quiet)); // h4 orthogonal
	assert!(has_move(&moves, 27, 0, MoveKind::Quiet)); // a1 diagonal
	assert!(has_move(&moves, 27, 63, MoveKind::Quiet)); // h8 diagonal
}

#[test]
fn queen_a1_corner_all_directions() {
	// Queen in corner should move along rank, file, and one diagonal
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White });

	let piece = pos.board[0].unwrap();
	let moves = pos.slider_targets(piece, 0).expect("slider_targets returned Err");

	assert_eq!(moves.len(), 21);

	// Orthogonal moves along rank
	assert!(has_move(&moves, 0, 7, MoveKind::Quiet)); // a1-h1

	// Orthogonal moves along file
	assert!(has_move(&moves, 0, 56, MoveKind::Quiet)); // a1-a8

	// Diagonal moves
	assert!(has_move(&moves, 0, 63, MoveKind::Quiet)); // a1-h8 diagonal

	// Verify we have moves in all available directions (7 + 7 + 7 = 21)
	assert!(has_move(&moves, 0, 1, MoveKind::Quiet)); // b1
	assert!(has_move(&moves, 0, 8, MoveKind::Quiet)); // a2
	assert!(has_move(&moves, 0, 9, MoveKind::Quiet)); // b2 (diagonal)
}

#[test]
fn queen_e4_blocked_in_multiple_directions() {
	// Queen with blockers in different directions
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // e4

	// Add friendly blockers in some directions
	pos.board[36] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // e5 (blocks north)

	pos.board[37] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // f5 (blocks northeast)

	// Add enemy blockers in other directions
	pos.board[19] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // d3 (blocks southwest, capturable)

	let piece = pos.board[28].unwrap();
	let moves = pos.slider_targets(piece, 28).expect("slider_targets returned Err");

	// Should not move past friendly blockers
	assert!(!has_to_square(&moves, 36)); // e5 blocked
	assert!(!has_to_square(&moves, 37)); // f5 blocked
	assert!(!has_to_square(&moves, 44)); // e6 (past e5)

	// Should capture enemy blocker but not move past it
	assert!(has_move(&moves, 28, 19, MoveKind::Capture)); // d3 capture
	assert!(!has_to_square(&moves, 10)); // c2 (past d3)

	// Other directions should be open
	assert!(has_move(&moves, 28, 4, MoveKind::Quiet)); // e1
	assert!(has_move(&moves, 28, 31, MoveKind::Quiet)); // h4
}
