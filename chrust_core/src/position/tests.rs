use super::*;
use crate::errors::{ChessError, FenError};
use crate::game_status::GameStatus;
use crate::{ColoredPiece, Piece, Side};

// ── convert_square_string_to_square ──────────────────────────────────────────

#[test]
fn square_string_a1_is_square_0() {
	assert_eq!(convert_square_string_to_square("a1").unwrap(), 0);
}

#[test]
fn square_string_h1_is_square_7() {
	assert_eq!(convert_square_string_to_square("h1").unwrap(), 7);
}

#[test]
fn square_string_a8_is_square_56() {
	assert_eq!(convert_square_string_to_square("a8").unwrap(), 56);
}

#[test]
fn square_string_h8_is_square_63() {
	assert_eq!(convert_square_string_to_square("h8").unwrap(), 63);
}

#[test]
fn square_string_e4_is_square_28() {
	// e4 = file 4 (e=4), rank 3 (4-1) → 3*8+4 = 28
	assert_eq!(convert_square_string_to_square("e4").unwrap(), 28);
}

#[test]
fn square_string_accepts_uppercase() {
	// The function lowercases the input.
	assert_eq!(convert_square_string_to_square("E4").unwrap(), 28);
	assert_eq!(convert_square_string_to_square("H8").unwrap(), 63);
}

#[test]
fn square_string_too_short_returns_error() {
	assert!(matches!(convert_square_string_to_square("a"), Err(FenError::SquareLenghtIsnt2Wide(1))));
}

#[test]
fn square_string_too_long_returns_error() {
	assert!(matches!(convert_square_string_to_square("a12"), Err(FenError::SquareLenghtIsnt2Wide(3))));
}

#[test]
fn square_string_empty_returns_error() {
	assert!(matches!(convert_square_string_to_square(""), Err(FenError::SquareLenghtIsnt2Wide(0))));
}

#[test]
fn square_string_invalid_file_returns_error() {
	assert!(matches!(convert_square_string_to_square("i4"), Err(FenError::InvalidFile('i'))));
	assert!(matches!(convert_square_string_to_square("z1"), Err(FenError::InvalidFile('z'))));
}

#[test]
fn square_string_rank_zero_returns_error() {
	// Rank 0 is not a valid chess rank (1-8 only).
	assert!(matches!(convert_square_string_to_square("a0"), Err(FenError::InvalidRank('0'))));
}

#[test]
fn square_string_rank_nine_returns_error() {
	assert!(matches!(convert_square_string_to_square("a9"), Err(FenError::InvalidRank('9'))));
}

#[test]
fn square_string_non_digit_rank_returns_error() {
	assert!(matches!(convert_square_string_to_square("ax"), Err(FenError::InvalidRank('x'))));
}

// ── load_position_from_fen — error paths ─────────────────────────────────────

#[test]
fn fen_with_too_few_parts_returns_error() {
	// Standard FEN has 6 space-separated fields.
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"), Err(FenError::MissingFenParts)));
}

#[test]
fn fen_with_too_many_parts_returns_error() {
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 extra"), Err(FenError::MissingFenParts)));
}

#[test]
fn fen_with_invalid_piece_char_returns_error() {
	// 'X' is not a valid piece character.
	assert!(matches!(load_position_from_fen("rnbqkXnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), Err(FenError::InvalidPieceChar('X'))));
}

#[test]
fn fen_with_invalid_side_to_move_returns_error() {
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1"), Err(FenError::NotAValideSide)));
}

#[test]
fn fen_with_invalid_castling_char_returns_error() {
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w XQkq - 0 1"), Err(FenError::InvalidCastlingRights('X'))));
}

#[test]
fn fen_with_invalid_en_passant_square_returns_error() {
	// "z9" is not a valid square string.
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq z9 0 1"), Err(_)));
}

// ── load_position_from_fen — side to move ────────────────────────────────────

#[test]
fn fen_side_to_move_white() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.side_to_move, Side::White);
}

#[test]
fn fen_side_to_move_black() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
	assert_eq!(pos.side_to_move, Side::Black);
}

// ── load_position_from_fen — castling rights ─────────────────────────────────

#[test]
fn fen_castling_all_rights() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.castle, [true, true, true, true]);
}

#[test]
fn fen_castling_no_rights() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap();
	assert_eq!(pos.castle, [false, false, false, false]);
}

#[test]
fn fen_castling_white_only() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1").unwrap();
	// castle[0]=white kingside, castle[1]=white queenside
	assert!(pos.castle[0], "white kingside should be set");
	assert!(pos.castle[1], "white queenside should be set");
	assert!(!pos.castle[2], "black kingside should be unset");
	assert!(!pos.castle[3], "black queenside should be unset");
}

#[test]
fn fen_castling_black_only() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w kq - 0 1").unwrap();
	assert!(!pos.castle[0], "white kingside should be unset");
	assert!(!pos.castle[1], "white queenside should be unset");
	assert!(pos.castle[2], "black kingside should be set");
	assert!(pos.castle[3], "black queenside should be set");
}

#[test]
fn fen_castling_white_kingside_only() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K - 0 1").unwrap();
	assert!(pos.castle[0]);
	assert!(!pos.castle[1]);
	assert!(!pos.castle[2]);
	assert!(!pos.castle[3]);
}

#[test]
fn fen_castling_black_queenside_only() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q - 0 1").unwrap();
	assert!(!pos.castle[0]);
	assert!(!pos.castle[1]);
	assert!(!pos.castle[2]);
	assert!(pos.castle[3]);
}

// ── load_position_from_fen — en passant ──────────────────────────────────────

#[test]
fn fen_en_passant_dash_means_none() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.en_passant, None);
}

#[test]
fn fen_en_passant_e3_is_set() {
	// After 1. e4, the en passant square is e3 = square 20.
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
	assert_eq!(pos.en_passant, Some(20)); // e3 = file 4, rank 2 → 2*8+4 = 20
}

#[test]
fn fen_en_passant_d6_is_set() {
	// After 1...d5, the en passant square is d6 = square 43.
	let pos = load_position_from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2").unwrap();
	assert_eq!(pos.en_passant, Some(43)); // d6 = file 3, rank 5 → 5*8+3 = 43
}

// ── load_position_from_fen — board piece placement ───────────────────────────

#[test]
fn fen_starting_position_white_pieces_on_rank_1() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

	// Rank 1 (squares 0-7): R N B Q K B N R
	assert_eq!(pos.board[0], Some(ColoredPiece { piece: Piece::Rook, side: Side::White }));
	assert_eq!(pos.board[1], Some(ColoredPiece { piece: Piece::Knight, side: Side::White }));
	assert_eq!(pos.board[2], Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }));
	assert_eq!(pos.board[3], Some(ColoredPiece { piece: Piece::Queen, side: Side::White }));
	assert_eq!(pos.board[4], Some(ColoredPiece { piece: Piece::King, side: Side::White }));
	assert_eq!(pos.board[5], Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }));
	assert_eq!(pos.board[6], Some(ColoredPiece { piece: Piece::Knight, side: Side::White }));
	assert_eq!(pos.board[7], Some(ColoredPiece { piece: Piece::Rook, side: Side::White }));
}

#[test]
fn fen_starting_position_black_pieces_on_rank_8() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

	// Rank 8 (squares 56-63): r n b q k b n r
	assert_eq!(pos.board[56], Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }));
	assert_eq!(pos.board[57], Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }));
	assert_eq!(pos.board[58], Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }));
	assert_eq!(pos.board[59], Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }));
	assert_eq!(pos.board[60], Some(ColoredPiece { piece: Piece::King, side: Side::Black }));
	assert_eq!(pos.board[61], Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }));
	assert_eq!(pos.board[62], Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }));
	assert_eq!(pos.board[63], Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }));
}

#[test]
fn fen_starting_position_white_pawns_on_rank_2() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	for sq in 8..16u8 {
		assert_eq!(pos.board[sq as usize], Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }), "expected white pawn on square {sq}");
	}
}

#[test]
fn fen_starting_position_black_pawns_on_rank_7() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	for sq in 48..56u8 {
		assert_eq!(pos.board[sq as usize], Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }), "expected black pawn on square {sq}");
	}
}

#[test]
fn fen_starting_position_middle_ranks_are_empty() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	for sq in 16..48u8 {
		assert_eq!(pos.board[sq as usize], None, "square {sq} should be empty");
	}
}

#[test]
fn fen_digit_skips_empty_squares_correctly() {
	// "8" in a rank means 8 empty squares.
	let pos = load_position_from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
	assert!(pos.board.iter().all(|sq| sq.is_none()), "all squares should be empty");
}

#[test]
fn fen_mixed_digits_and_pieces_placed_correctly() {
	// "4P3" on rank 1 means 4 empty, white pawn on e1 (sq 4), then 3 empty.
	let pos = load_position_from_fen("8/8/8/8/8/8/8/4P3 w - - 0 1").unwrap();
	assert_eq!(pos.board[4], Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }));
	assert_eq!(pos.board[0], None);
	assert_eq!(pos.board[7], None);
}

#[test]
fn fen_all_six_piece_types_parsed_correctly() {
	// One of each piece type for both sides in a minimal FEN.
	// Board: white pieces on rank 1, black pieces on rank 8.
	let pos = load_position_from_fen("kqrbnp/8/8/8/8/8/8/KQRBNP w - - 0 1").unwrap();

	// White pieces (uppercase): K Q R B N P on a1–f1 (squares 4,3,2,1,6,8... no)
	// "KQRBNP" on rank 1: K=sq0, Q=sq1, R=sq2, B=sq3, N=sq4, P=sq5
	assert_eq!(pos.board[0], Some(ColoredPiece { piece: Piece::King, side: Side::White }));
	assert_eq!(pos.board[1], Some(ColoredPiece { piece: Piece::Queen, side: Side::White }));
	assert_eq!(pos.board[2], Some(ColoredPiece { piece: Piece::Rook, side: Side::White }));
	assert_eq!(pos.board[3], Some(ColoredPiece { piece: Piece::Bishop, side: Side::White }));
	assert_eq!(pos.board[4], Some(ColoredPiece { piece: Piece::Knight, side: Side::White }));
	assert_eq!(pos.board[5], Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }));

	// Black pieces (lowercase): k q r b n p on a8–f8 (squares 56–61)
	assert_eq!(pos.board[56], Some(ColoredPiece { piece: Piece::King, side: Side::Black }));
	assert_eq!(pos.board[57], Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }));
	assert_eq!(pos.board[58], Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }));
	assert_eq!(pos.board[59], Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }));
	assert_eq!(pos.board[60], Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }));
	assert_eq!(pos.board[61], Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }));
}

#[test]
fn fen_after_e4_board_state_correct() {
	// rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();

	// e4 = sq 28 has a white pawn
	assert_eq!(pos.board[28], Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }));
	// e2 = sq 12 is now empty
	assert_eq!(pos.board[12], None);
	// side to move is Black
	assert_eq!(pos.side_to_move, Side::Black);
	// en passant is e3 = sq 20
	assert_eq!(pos.en_passant, Some(20));
}

// ── load_position_from_fen — king_squares default ────────────────────────────

#[test]
fn fen_king_squares_default_on_starting_position() {
	// king_squares is not parsed from FEN — it keeps its default [4, 60].
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.king_squares, [4, 60]);
}

// ── load_position_from_fen — halfmove_clock / fullmove_counter ──────────────

#[test]
fn fen_halfmove_clock_parsed_correctly() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 7 42").unwrap();
	assert_eq!(pos.halfmove_clock, 7);
}

#[test]
fn fen_fullmove_counter_parsed_correctly() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 7 42").unwrap();
	assert_eq!(pos.fullmove_counter, 42);
}

#[test]
fn fen_halfmove_clock_zero() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn fen_fullmove_counter_starting_position() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert_eq!(pos.fullmove_counter, 1);
}

#[test]
fn fen_invalid_halfmove_clock_returns_error() {
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - abc 1"), Err(FenError::InvalidNumber(_))));
}

#[test]
fn fen_invalid_fullmove_counter_returns_error() {
	assert!(matches!(load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 xyz"), Err(FenError::InvalidNumber(_))));
}

#[test]
fn fen_large_halfmove_clock_parsed_correctly() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 99 1").unwrap();
	assert_eq!(pos.halfmove_clock, 99);
}

#[test]
fn fen_large_fullmove_counter_parsed_correctly() {
	let pos = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 999").unwrap();
	assert_eq!(pos.fullmove_counter, 999);
}

// ── load_position_from_fen — complete position tests ─────────────────────────

#[test]
fn fen_complex_middlegame_position() {
	// A more complex position from a real game
	let fen = "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
	let pos = load_position_from_fen(fen).unwrap();

	assert_eq!(pos.side_to_move, Side::White);
	assert_eq!(pos.castle, [true, true, true, true]);
	assert_eq!(pos.en_passant, None);
	assert_eq!(pos.halfmove_clock, 4);
	assert_eq!(pos.fullmove_counter, 4);
}

#[test]
fn fen_position_with_partial_castling_rights() {
	// Position where only some castling rights remain
	let fen = "r3k2r/8/8/8/8/8/8/R3K2R w Kq - 0 1";
	let pos = load_position_from_fen(fen).unwrap();

	assert!(pos.castle[0], "white kingside should be set");
	assert!(!pos.castle[1], "white queenside should be unset");
	assert!(!pos.castle[2], "black kingside should be unset");
	assert!(pos.castle[3], "black queenside should be set");
}

#[test]
fn fen_endgame_position() {
	// King and pawn endgame
	let fen = "8/5k2/8/8/3K4/8/4P3/8 w - - 10 50";
	let pos = load_position_from_fen(fen).unwrap();

	assert_eq!(pos.side_to_move, Side::White);
	assert_eq!(pos.castle, [false, false, false, false]);
	assert_eq!(pos.en_passant, None);
	assert_eq!(pos.halfmove_clock, 10);
	assert_eq!(pos.fullmove_counter, 50);

	// Check that pieces are placed correctly
	assert_eq!(pos.board[27], Some(ColoredPiece { piece: Piece::King, side: Side::White })); // d4
	assert_eq!(pos.board[12], Some(ColoredPiece { piece: Piece::Pawn, side: Side::White })); // e2
	assert_eq!(pos.board[53], Some(ColoredPiece { piece: Piece::King, side: Side::Black }));
	// f7
}

// ── Game::try_from_fen ────────────────────────────────────────────────────────

#[test]
fn try_from_fen_position_matches_load_position_from_fen() {
	let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
	let game = Game::try_from_fen(fen).unwrap();
	let expected_position = load_position_from_fen(fen).unwrap();
	assert_eq!(game.position, expected_position);
}

#[test]
fn try_from_fen_all_vecs_are_empty() {
	let game = Game::try_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert!(game.hash_history.is_empty(), "hash_history should be empty");
	assert!(game.move_history.is_empty(), "move_history should be empty");
	assert!(game.undo_history.is_empty(), "undo_history should be empty");
}

#[test]
fn try_from_fen_invalid_fen_returns_err() {
	let result = Game::try_from_fen("not a valid fen string");
	assert!(matches!(result, Err(ChessError::FenError { .. })));
}

#[test]
fn try_from_fen_status_playing() {
	let game = Game::try_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	assert!(matches!(game.game_status, GameStatus::Playing));
}

#[test]
fn try_from_fen_status_in_check() {
	// White king on e1, black rook on e8 giving check, black king on a8
	let game = Game::try_from_fen("k7/8/8/8/8/8/8/4K2r w - - 0 1").unwrap();
	assert!(matches!(game.game_status, GameStatus::InCheck));
}

#[test]
fn try_from_fen_status_checkmate() {
	// Back-rank mate: white king on h1 trapped by own pawns on g2/h2, black rooks on a1 and h8
	let game = Game::try_from_fen("6k1/8/8/8/8/8/6PP/r6K w - - 0 1").unwrap();
	assert!(matches!(game.game_status, GameStatus::CheckmateForSide(Side::Black)));
}

#[test]
fn try_from_fen_status_stalemate() {
	// White king on a1 stalemated: black queen on b3, black king on c2
	let game = Game::try_from_fen("8/8/8/8/8/1q6/2k5/K7 w - - 0 1").unwrap();
	assert!(matches!(game.game_status, GameStatus::Stalemate));
}

#[test]
fn try_from_fen_status_draw_by_fifty_moves() {
	// Halfmove clock at 100: draw by fifty-move rule
	let game = Game::try_from_fen("8/5k2/8/8/3K4/8/8/8 w - - 100 80").unwrap();
	assert!(matches!(game.game_status, GameStatus::DrawByFiftyMoves));
}

#[test]
fn try_from_fen_status_draw_by_insufficient_material() {
	// Kings only: neither side can mate
	let game = Game::try_from_fen("8/5k2/8/8/3K4/8/8/8 w - - 0 1").unwrap();
	assert!(matches!(game.game_status, GameStatus::DrawByInsufficientMaterial));
}
