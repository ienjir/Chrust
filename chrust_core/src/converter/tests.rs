use super::*;
use crate::errors::{ChessError, FenError};
use crate::moves::make_move::{Move, MoveKind};
use crate::test_common::game_from_fen;
use crate::{ColoredPiece, Piece, Side};

// ── convert_square_to_string ──────────────────────────────────────────────────

#[test]
fn square_to_string_a1() {
	assert_eq!(convert_square_to_string(0), "a1");
}

#[test]
fn square_to_string_h8() {
	assert_eq!(convert_square_to_string(63), "h8");
}

#[test]
fn square_to_string_e4() {
	assert_eq!(convert_square_to_string(28), "e4");
}

#[test]
fn square_to_string_d1() {
	assert_eq!(convert_square_to_string(3), "d1");
}

// ── convert_string_to_square ──────────────────────────────────────────────────

#[test]
fn string_to_square_a1() {
	assert_eq!(convert_string_to_square("a1"), Ok(0));
}

#[test]
fn string_to_square_h8() {
	assert_eq!(convert_string_to_square("h8"), Ok(63));
}

#[test]
fn string_to_square_e4() {
	assert_eq!(convert_string_to_square("e4"), Ok(28));
}

#[test]
fn string_to_square_is_case_insensitive() {
	assert_eq!(convert_string_to_square("E4"), Ok(28));
}

#[test]
fn string_to_square_too_short_returns_error() {
	assert!(matches!(convert_string_to_square("a"), Err(FenError::SquareLenghtIsnt2Wide(1))));
}

#[test]
fn string_to_square_too_long_returns_error() {
	assert!(matches!(convert_string_to_square("a1b"), Err(FenError::SquareLenghtIsnt2Wide(3))));
}

#[test]
fn string_to_square_invalid_file_returns_error() {
	assert!(matches!(convert_string_to_square("z1"), Err(FenError::InvalidFile('z'))));
}

#[test]
fn string_to_square_invalid_rank_zero_returns_error() {
	assert!(matches!(convert_string_to_square("a0"), Err(FenError::InvalidRank('0'))));
}

#[test]
fn string_to_square_invalid_rank_nine_returns_error() {
	assert!(matches!(convert_string_to_square("a9"), Err(FenError::InvalidRank('9'))));
}

// ── letter_to_piece ───────────────────────────────────────────────────────────

#[test]
fn letter_to_piece_k_is_king() {
	assert_eq!(letter_to_piece('k'), Ok(Piece::King));
}

#[test]
fn letter_to_piece_uppercase_k_is_king() {
	assert_eq!(letter_to_piece('K'), Ok(Piece::King));
}

#[test]
fn letter_to_piece_q_is_queen() {
	assert_eq!(letter_to_piece('q'), Ok(Piece::Queen));
}

#[test]
fn letter_to_piece_r_is_rook() {
	assert_eq!(letter_to_piece('r'), Ok(Piece::Rook));
}

#[test]
fn letter_to_piece_b_is_bishop() {
	assert_eq!(letter_to_piece('b'), Ok(Piece::Bishop));
}

#[test]
fn letter_to_piece_n_is_knight() {
	assert_eq!(letter_to_piece('n'), Ok(Piece::Knight));
}

#[test]
fn letter_to_piece_p_is_pawn() {
	assert_eq!(letter_to_piece('p'), Ok(Piece::Pawn));
}

#[test]
fn letter_to_piece_invalid_char_returns_error() {
	assert!(matches!(letter_to_piece('x'), Err(FenError::InvalidPieceChar('x'))));
}

// ── Piece::to_char ────────────────────────────────────────────────────────────

#[test]
fn piece_to_char_pawn() {
	assert_eq!(Piece::Pawn.to_char(), 'p');
}

#[test]
fn piece_to_char_knight() {
	assert_eq!(Piece::Knight.to_char(), 'n');
}

#[test]
fn piece_to_char_bishop() {
	assert_eq!(Piece::Bishop.to_char(), 'b');
}

#[test]
fn piece_to_char_rook() {
	assert_eq!(Piece::Rook.to_char(), 'r');
}

#[test]
fn piece_to_char_queen() {
	assert_eq!(Piece::Queen.to_char(), 'q');
}

#[test]
fn piece_to_char_king() {
	assert_eq!(Piece::King.to_char(), 'k');
}

// ── ColoredPiece::to_char ─────────────────────────────────────────────────────

#[test]
fn colored_piece_to_char_white_pawn_is_uppercase() {
	let cp = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	assert_eq!(cp.to_char(), 'P');
}

#[test]
fn colored_piece_to_char_black_pawn_is_lowercase() {
	let cp = ColoredPiece { piece: Piece::Pawn, side: Side::Black };
	assert_eq!(cp.to_char(), 'p');
}

#[test]
fn colored_piece_to_char_white_king() {
	let cp = ColoredPiece { piece: Piece::King, side: Side::White };
	assert_eq!(cp.to_char(), 'K');
}

#[test]
fn colored_piece_to_char_black_queen() {
	let cp = ColoredPiece { piece: Piece::Queen, side: Side::Black };
	assert_eq!(cp.to_char(), 'q');
}

#[test]
fn colored_piece_to_char_white_rook() {
	let cp = ColoredPiece { piece: Piece::Rook, side: Side::White };
	assert_eq!(cp.to_char(), 'R');
}

// ── Move::to_uci ──────────────────────────────────────────────────────────────

#[test]
fn to_uci_quiet_move_does_not_panic() {
	let white_rook = ColoredPiece { piece: Piece::Rook, side: Side::White };
	let mv = Move {
		from_square: 0,
		to_square: 8,
		move_kind: MoveKind::Quiet,
		colored_piece: white_rook,
	};
	mv.to_uci();
}

#[test]
fn to_uci_promotion_does_not_panic() {
	let white_pawn = ColoredPiece { piece: Piece::Pawn, side: Side::White };
	let mv = Move {
		from_square: 48,
		to_square: 56,
		move_kind: MoveKind::Promotion { promotion_piece: Piece::Queen },
		colored_piece: white_pawn,
	};
	mv.to_uci();
}

// ── Game::convert_uci_to_move ─────────────────────────────────────────────────

#[test]
fn convert_uci_to_move_quiet_pawn_push() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let mv = game.convert_uci_to_move("e2e3").unwrap();
	assert_eq!(mv.from_square, 12);
	assert_eq!(mv.to_square, 20);
	assert_eq!(mv.move_kind, MoveKind::Quiet);
	assert_eq!(mv.colored_piece, ColoredPiece { piece: Piece::Pawn, side: Side::White });
}

#[test]
fn convert_uci_to_move_white_double_pawn_push() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let mv = game.convert_uci_to_move("e2e4").unwrap();
	assert_eq!(mv.from_square, 12);
	assert_eq!(mv.to_square, 28);
	assert_eq!(mv.move_kind, MoveKind::DoublePawnPush { passed_square: 20 });
}

#[test]
fn convert_uci_to_move_black_double_pawn_push() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
	let mv = game.convert_uci_to_move("e7e5").unwrap();
	assert_eq!(mv.from_square, 52);
	assert_eq!(mv.to_square, 36);
	assert_eq!(mv.move_kind, MoveKind::DoublePawnPush { passed_square: 44 });
}

#[test]
fn convert_uci_to_move_capture() {
	let game = game_from_fen("4k3/p7/8/8/8/8/8/R3K3 w - - 0 1");
	let mv = game.convert_uci_to_move("a1a7").unwrap();
	assert_eq!(mv.from_square, 0);
	assert_eq!(mv.to_square, 48);
	assert_eq!(mv.move_kind, MoveKind::Capture);
}

#[test]
fn convert_uci_to_move_white_kingside_castling() {
	let game = game_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
	let mv = game.convert_uci_to_move("e1g1").unwrap();
	assert_eq!(mv.from_square, 4);
	assert_eq!(mv.to_square, 6);
	assert_eq!(mv.move_kind, MoveKind::Castling { rook_from: 7, rook_to: 5 });
}

#[test]
fn convert_uci_to_move_white_queenside_castling() {
	let game = game_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
	let mv = game.convert_uci_to_move("e1c1").unwrap();
	assert_eq!(mv.from_square, 4);
	assert_eq!(mv.to_square, 2);
	assert_eq!(mv.move_kind, MoveKind::Castling { rook_from: 0, rook_to: 3 });
}

#[test]
fn convert_uci_to_move_black_kingside_castling() {
	let game = game_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1");
	let mv = game.convert_uci_to_move("e8g8").unwrap();
	assert_eq!(mv.from_square, 60);
	assert_eq!(mv.to_square, 62);
	assert_eq!(mv.move_kind, MoveKind::Castling { rook_from: 63, rook_to: 61 });
}

#[test]
fn convert_uci_to_move_black_queenside_castling() {
	let game = game_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1");
	let mv = game.convert_uci_to_move("e8c8").unwrap();
	assert_eq!(mv.from_square, 60);
	assert_eq!(mv.to_square, 58);
	assert_eq!(mv.move_kind, MoveKind::Castling { rook_from: 56, rook_to: 59 });
}

#[test]
fn convert_uci_to_move_promotion_to_queen() {
	let game = game_from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1");
	let mv = game.convert_uci_to_move("a7a8q").unwrap();
	assert_eq!(mv.from_square, 48);
	assert_eq!(mv.to_square, 56);
	assert_eq!(mv.move_kind, MoveKind::Promotion { promotion_piece: Piece::Queen });
}

#[test]
fn convert_uci_to_move_promotion_to_knight() {
	let game = game_from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1");
	let mv = game.convert_uci_to_move("a7a8n").unwrap();
	assert_eq!(mv.move_kind, MoveKind::Promotion { promotion_piece: Piece::Knight });
}

#[test]
fn convert_uci_to_move_promotion_to_rook() {
	let game = game_from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1");
	let mv = game.convert_uci_to_move("a7a8r").unwrap();
	assert_eq!(mv.move_kind, MoveKind::Promotion { promotion_piece: Piece::Rook });
}

#[test]
fn convert_uci_to_move_promotion_to_bishop() {
	let game = game_from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1");
	let mv = game.convert_uci_to_move("a7a8b").unwrap();
	assert_eq!(mv.move_kind, MoveKind::Promotion { promotion_piece: Piece::Bishop });
}

#[test]
fn convert_uci_to_move_en_passant() {
	let game = game_from_fen("4k3/ppp1pppp/8/3pP3/8/8/PPP2PPP/4K3 w - d6 0 1");
	let mv = game.convert_uci_to_move("e5d6").unwrap();
	assert_eq!(mv.from_square, 36);
	assert_eq!(mv.to_square, 43);
	assert_eq!(mv.move_kind, MoveKind::EnPassant { capture_square: 35 });
}

#[test]
fn convert_uci_to_move_quiet_knight_move() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let mv = game.convert_uci_to_move("g1f3").unwrap();
	assert_eq!(mv.from_square, 6);
	assert_eq!(mv.to_square, 21);
	assert_eq!(mv.move_kind, MoveKind::Quiet);
	assert_eq!(mv.colored_piece, ColoredPiece { piece: Piece::Knight, side: Side::White });
}

#[test]
fn convert_uci_to_move_no_piece_returns_error() {
	let game = game_from_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1");
	let result = game.convert_uci_to_move("a1a2");
	assert!(matches!(result, Err(ChessError::NoPieceOnSquare { square: 0 })));
}

#[test]
fn convert_uci_to_move_invalid_from_file_returns_error() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let result = game.convert_uci_to_move("z1e4");
	assert!(result.is_err());
}

#[test]
fn convert_uci_to_move_invalid_to_rank_returns_error() {
	let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	let result = game.convert_uci_to_move("e2e9");
	assert!(result.is_err());
}
