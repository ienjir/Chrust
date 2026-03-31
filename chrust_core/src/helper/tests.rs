use super::*;
use crate::test_common::empty_position;
use crate::{ColoredPiece, Piece, Side, errors::ChessError};

#[test]
fn is_square_on_board_positive_test() {
	assert_eq!(is_square_on_board(12), Ok(()));
}

#[test]
fn is_square_on_board_negative_test_with_positive_number_64() {
	assert_eq!(is_square_on_board(64), Err(ChessError::NotASquareOnBoard { square: 64 }));
}

#[test]
fn is_square_on_board_boundary_positives_0_and_63() {
	assert_eq!(is_square_on_board(0), Ok(()));
	assert_eq!(is_square_on_board(63), Ok(()));
}

#[test]
fn is_square_on_board_negative_test_with_negative_number() {
	let negative_square_as_u8 = -1i8 as u8;

	assert_eq!(is_square_on_board(negative_square_as_u8), Err(ChessError::NotASquareOnBoard { square: negative_square_as_u8 as i16 }));
}

#[test]
fn is_right_piece_type_positive_test_with_both_sides() {
	assert_eq!(is_right_piece_type(ColoredPiece { piece: Piece::King, side: Side::White }, Piece::King,), Ok(()));

	assert_eq!(is_right_piece_type(ColoredPiece { piece: Piece::Pawn, side: Side::Black }, Piece::Pawn,), Ok(()));
}

#[test]
fn is_right_piece_type_negative_test_with_both_sides() {
	assert_eq!(
		is_right_piece_type(ColoredPiece { piece: Piece::King, side: Side::White }, Piece::Queen,),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Queen,
			found_piece: Piece::King
		})
	);

	assert_eq!(
		is_right_piece_type(ColoredPiece { piece: Piece::Rook, side: Side::Black }, Piece::Bishop,),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Bishop,
			found_piece: Piece::Rook
		})
	);
}

#[test]
fn is_right_piece_side_positive_test_with_both_sides() {
	assert_eq!(is_right_piece_side(ColoredPiece { piece: Piece::Knight, side: Side::White }, Side::White,), Ok(()));

	assert_eq!(is_right_piece_side(ColoredPiece { piece: Piece::Knight, side: Side::Black }, Side::Black,), Ok(()));
}

#[test]
fn is_right_piece_side_negative_test_with_both_sides() {
	assert_eq!(
		is_right_piece_side(ColoredPiece { piece: Piece::Knight, side: Side::White }, Side::Black,),
		Err(ChessError::WrongSide {
			expected_side: Side::Black,
			found_side: Side::White
		})
	);

	assert_eq!(
		is_right_piece_side(ColoredPiece { piece: Piece::Knight, side: Side::Black }, Side::White,),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn get_colored_piece_from_square_positive_test_for_both_sides() {
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	pos.board[63] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });

	assert_eq!(pos.get_piece_from_square(0), Ok(ColoredPiece { piece: Piece::Rook, side: Side::White }));

	assert_eq!(pos.get_piece_from_square(63), Ok(ColoredPiece { piece: Piece::Rook, side: Side::Black }));
}

#[test]
fn get_colored_piece_from_square_negative_test_for_both_sides() {
	let pos_white = empty_position();

	assert_eq!(pos_white.get_piece_from_square(10), Err(ChessError::NoPieceOnSquare { square: 10 }));

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;

	assert_eq!(pos_black.get_piece_from_square(54), Err(ChessError::NoPieceOnSquare { square: 54 }));
}

#[test]
fn get_colored_piece_from_square_invalid_square_test() {
	let pos = empty_position();

	assert_eq!(pos.get_piece_from_square(64), Err(ChessError::NotASquareOnBoard { square: 64 }));
}

#[test]
fn get_validated_colored_piece_positive_test_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	assert_eq!(pos_white.get_validated_colored_piece(0, Piece::Rook), Ok(ColoredPiece { piece: Piece::Rook, side: Side::White }));

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[63] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	assert_eq!(pos_black.get_validated_colored_piece(63, Piece::Knight), Ok(ColoredPiece { piece: Piece::Knight, side: Side::Black }));
}

#[test]
fn get_validated_colored_piece_wrong_expected_piece_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });

	assert_eq!(
		pos_white.get_validated_colored_piece(4, Piece::Queen),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Queen,
			found_piece: Piece::King
		})
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });

	assert_eq!(
		pos_black.get_validated_colored_piece(60, Piece::Rook),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Rook,
			found_piece: Piece::King
		})
	);
}

#[test]
fn get_validated_colored_piece_wrong_expected_side_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.side_to_move = Side::White;
	pos_white.board[60] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	assert_eq!(
		pos_white.get_validated_colored_piece(60, Piece::Knight),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[4] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White });

	assert_eq!(
		pos_black.get_validated_colored_piece(4, Piece::Knight),
		Err(ChessError::WrongSide {
			expected_side: Side::Black,
			found_side: Side::White
		})
	);
}

#[test]
fn get_validated_colored_piece_error_precedence_invalid_square_short_circuits() {
	let pos = empty_position();

	assert_eq!(pos.get_validated_colored_piece(64, Piece::Queen), Err(ChessError::NotASquareOnBoard { square: 64 }));
}

#[test]
fn validate_colored_piece_correct_piece_and_side() {
	let mut pos = empty_position();
	pos.board[0] = Some(ColoredPiece { piece: Piece::Rook, side: Side::White });

	assert_eq!(pos.validate_colored_piece(ColoredPiece { piece: Piece::Rook, side: Side::White }, Piece::Rook,), Ok(()));
}

#[test]
fn validate_colored_piece_wrong_side_returns_wrong_side_before_piece_type() {
	// side_to_move is White, but piece is Black — WrongSide should come first.
	let pos = empty_position(); // side_to_move = White

	assert_eq!(
		pos.validate_colored_piece(ColoredPiece { piece: Piece::Rook, side: Side::Black }, Piece::Rook,),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}

#[test]
fn validate_colored_piece_wrong_piece_type() {
	let pos = empty_position(); // side_to_move = White

	assert_eq!(
		pos.validate_colored_piece(ColoredPiece { piece: Piece::Knight, side: Side::White }, Piece::Rook,),
		Err(ChessError::WrongPieceType {
			expected_piece: Piece::Rook,
			found_piece: Piece::Knight
		})
	);
}

// ── geometry helpers ──────────────────────────────────────────────────────────

#[test]
fn file_of_corners_and_edges() {
	assert_eq!(file(0), 0, "a1 is file 0");
	assert_eq!(file(7), 7, "h1 is file 7");
	assert_eq!(file(8), 0, "a2 is file 0");
	assert_eq!(file(15), 7, "h2 is file 7");
	assert_eq!(file(56), 0, "a8 is file 0");
	assert_eq!(file(63), 7, "h8 is file 7");
}

#[test]
fn rank_of_corners_and_edges() {
	assert_eq!(rank(0), 0, "a1 is rank 0");
	assert_eq!(rank(7), 0, "h1 is rank 0");
	assert_eq!(rank(8), 1, "a2 is rank 1");
	assert_eq!(rank(56), 7, "a8 is rank 7");
	assert_eq!(rank(63), 7, "h8 is rank 7");
}

#[test]
fn square_from_file_and_rank() {
	assert_eq!(square(0, 0), 0, "a1 = 0");
	assert_eq!(square(7, 0), 7, "h1 = 7");
	assert_eq!(square(0, 1), 8, "a2 = 8");
	assert_eq!(square(4, 3), 28, "e4 = 28");
	assert_eq!(square(7, 7), 63, "h8 = 63");
}

#[test]
fn file_rank_round_trips() {
	for f in 0u8..8 {
		for r in 0u8..8 {
			let sq = square(f, r);
			assert_eq!(file_rank(sq), (f, r), "file_rank(square({f},{r})) should round-trip");
		}
	}
}

// ── ColoredPiece::to_char ─────────────────────────────────────────────────────

#[test]
fn to_char_white_pieces_are_uppercase() {
	assert_eq!(ColoredPiece { piece: Piece::King, side: Side::White }.to_char(), 'K');
	assert_eq!(ColoredPiece { piece: Piece::Queen, side: Side::White }.to_char(), 'Q');
	assert_eq!(ColoredPiece { piece: Piece::Rook, side: Side::White }.to_char(), 'R');
	assert_eq!(ColoredPiece { piece: Piece::Bishop, side: Side::White }.to_char(), 'B');
	assert_eq!(ColoredPiece { piece: Piece::Knight, side: Side::White }.to_char(), 'N');
	assert_eq!(ColoredPiece { piece: Piece::Pawn, side: Side::White }.to_char(), 'P');
}

#[test]
fn to_char_black_pieces_are_lowercase() {
	assert_eq!(ColoredPiece { piece: Piece::King, side: Side::Black }.to_char(), 'k');
	assert_eq!(ColoredPiece { piece: Piece::Queen, side: Side::Black }.to_char(), 'q');
	assert_eq!(ColoredPiece { piece: Piece::Rook, side: Side::Black }.to_char(), 'r');
	assert_eq!(ColoredPiece { piece: Piece::Bishop, side: Side::Black }.to_char(), 'b');
	assert_eq!(ColoredPiece { piece: Piece::Knight, side: Side::Black }.to_char(), 'n');
	assert_eq!(ColoredPiece { piece: Piece::Pawn, side: Side::Black }.to_char(), 'p');
}

// ── letter_to_piece ───────────────────────────────────────────────────────────

#[test]
fn letter_to_piece_all_valid_lowercase() {
	assert_eq!(letter_to_piece('k'), Ok(Piece::King));
	assert_eq!(letter_to_piece('p'), Ok(Piece::Pawn));
	assert_eq!(letter_to_piece('n'), Ok(Piece::Knight));
	assert_eq!(letter_to_piece('b'), Ok(Piece::Bishop));
	assert_eq!(letter_to_piece('r'), Ok(Piece::Rook));
	assert_eq!(letter_to_piece('q'), Ok(Piece::Queen));
}

#[test]
fn letter_to_piece_all_valid_uppercase() {
	assert_eq!(letter_to_piece('K'), Ok(Piece::King));
	assert_eq!(letter_to_piece('P'), Ok(Piece::Pawn));
	assert_eq!(letter_to_piece('N'), Ok(Piece::Knight));
	assert_eq!(letter_to_piece('B'), Ok(Piece::Bishop));
	assert_eq!(letter_to_piece('R'), Ok(Piece::Rook));
	assert_eq!(letter_to_piece('Q'), Ok(Piece::Queen));
}

#[test]
fn letter_to_piece_invalid_char_returns_error() {
	assert!(letter_to_piece('x').is_err());
	assert!(letter_to_piece('1').is_err());
	assert!(letter_to_piece('z').is_err());
}

// ── integration ───────────────────────────────────────────────────────────────

#[test]
fn integration_get_possible_moves_respects_side_to_move() {
	let mut pos = empty_position();
	pos.side_to_move = Side::White;
	pos.board[63] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });

	assert_eq!(
		pos.get_pseduo_legal_moves(63, ColoredPiece { piece: Piece::Rook, side: Side::Black }),
		Err(ChessError::WrongSide {
			expected_side: Side::White,
			found_side: Side::Black
		})
	);
}
