use super::*;
use crate::errors::ChessError;
use crate::moves::make_move::{Move, MoveKind};
use crate::position::Position;
use crate::test_common::{empty_position, has_move, has_to_square};
use crate::{ColoredPiece, Piece, Side, Square};

fn has_square(squares: &[Square], square: Square) -> bool {
	squares.iter().any(|&s| s == square)
}

#[test]
fn is_square_attacked_empty_square_none() {
	let pos = empty_position();

	assert_eq!(pos.is_square_attacked(35, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_out_of_bounds() {
	let pos = empty_position();

	assert_eq!(pos.is_square_attacked(65, Side::White), Err(ChessError::NotASquareOnBoard { square: 65 }));
}

#[test]
fn is_square_attacked_empty_board_none() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });

	assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_by_black_pawns() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[35] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });
	pos.board[37] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black });

	let mut attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();
	attacks.sort_unstable();

	assert_eq!(attacks, vec![35, 37]);
}

#[test]
fn is_square_attacked_by_white_pawns() {
	let mut pos = empty_position();

	pos.board[36] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.board[27] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });
	pos.board[29] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White });

	let mut attacks = pos.is_square_attacked(36, Side::White).expect("is_square_attacked returned Err").unwrap();
	attacks.sort_unstable();

	assert_eq!(attacks, vec![27, 29]);
}

#[test]
fn is_square_attacked_by_rook_and_bishop() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });
	pos.board[1] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black });

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 60));
	assert!(has_square(&attacks, 1));
	assert_eq!(attacks.len(), 2);
}

#[test]
fn is_square_attacked_by_king_adjacent() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[29] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 29));
}

#[test]
fn is_square_attacked_king_does_not_wrap_board_edge() {
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[8] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });

	assert_eq!(pos.is_square_attacked(7, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_by_knight_l_shape() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[45] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 45));
}

#[test]
fn is_square_attacked_knight_does_not_wrap_board_edge() {
	let mut pos = empty_position();

	pos.board[7] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[17] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	assert_eq!(pos.is_square_attacked(7, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_blocked_by_friendly_piece() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });
	pos.board[44] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::White });

	assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_by_queen_on_diagonal_ray() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[1] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black });

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 1));
}

#[test]
fn is_square_attacked_by_queen_on_orthogonal_ray() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[24] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // same rank, left

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 24));
}

#[test]
fn is_square_attacked_by_rook_on_horizontal_ray() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[31] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // h4, same rank

	let attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 31));
}

#[test]
fn is_square_attacked_by_two_knights_simultaneously() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[45] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // +17
	pos.board[43] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black }); // +15

	let mut attacks = pos.is_square_attacked(28, Side::Black).expect("is_square_attacked returned Err").unwrap();
	attacks.sort_unstable();

	assert_eq!(attacks, vec![43, 45]);
}

#[test]
fn is_square_attacked_pawn_does_not_wrap_from_a_file() {
	let mut pos = empty_position();

	pos.board[41] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // b6
	pos.board[32] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::Black }); // a5

	let attacks = pos.is_square_attacked(41, Side::White).expect("Err");
	assert_eq!(pos.is_square_attacked(47, Side::Black), Ok(None), "a-file pawn should not wrap to attack h6");
}

#[test]
fn is_square_attacked_pawn_does_not_wrap_from_h_file() {
	let mut pos = empty_position();

	pos.board[31] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // h4

	assert_eq!(pos.is_square_attacked(32, Side::White), Ok(None), "h-file pawn should not wrap to attack a5");
}

#[test]
fn is_square_attacked_king_attacks_from_all_8_directions() {
	let mut pos = empty_position();
	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });

	for neighbour in [19u8, 20, 21, 27, 29, 35, 36, 37] {
		let attacks = pos.is_square_attacked(neighbour, Side::Black).expect("Err");
		assert!(attacks.is_some() && has_square(attacks.as_ref().unwrap(), 28), "black king on e4 should attack sq {neighbour}");
	}
}

// ── is_king_in_check ──────────────────────────────────────────────────────────

#[test]
fn is_king_in_check_white_not_in_check() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];
	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });

	assert_eq!(pos.is_king_in_check(Side::White), Ok(None));
}

#[test]
fn is_king_in_check_white_in_check_from_rook() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];
	pos.board[4] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // attacks e1

	let result = pos.is_king_in_check(Side::White).expect("Err");
	assert!(result.is_some(), "white king should be in check");
	assert!(has_square(result.as_ref().unwrap(), 60));
}

#[test]
fn is_king_in_check_black_not_in_check() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });

	assert_eq!(pos.is_king_in_check(Side::Black), Ok(None));
}

#[test]
fn is_king_in_check_black_in_check_from_knight() {
	let mut pos = empty_position();
	pos.king_squares = [4, 60];
	pos.board[60] = Some(ColoredPiece { piece: Piece::King, side: Side::Black });
	pos.board[43] = Some(ColoredPiece { piece: Piece::Knight, side: Side::White }); // attacks e8

	let result = pos.is_king_in_check(Side::Black).expect("Err");
	assert!(result.is_some(), "black king should be in check");
	assert!(has_square(result.as_ref().unwrap(), 43));
}

#[test]
fn is_king_in_check_uses_king_squares_array() {
	let mut pos = empty_position();
	pos.king_squares = [27, 60];
	pos.board[27] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[3] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // d1 attacks d4

	let result = pos.is_king_in_check(Side::White).expect("Err");
	assert!(result.is_some(), "white king on d4 should be in check from rook on d1");
}

#[test]
fn is_square_attacked_blocked_by_enemy_non_attacker() {
	let mut pos = empty_position();

	pos.board[28] = Some(ColoredPiece { piece: Piece::King, side: Side::White });
	pos.board[60] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black });
	pos.board[44] = Some(ColoredPiece { piece: Piece::Knight, side: Side::Black });

	assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
}

#[test]
fn is_square_attacked_sliding_does_not_wrap_board_edge() {
	let mut pos = empty_position();

	pos.board[24] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a4
	pos.board[31] = Some(ColoredPiece { piece: Piece::Rook, side: Side::Black }); // h4

	let attacks = pos.is_square_attacked(24, Side::Black).expect("is_square_attacked returned Err").unwrap();

	assert!(has_square(&attacks, 31));

	assert_eq!(pos.is_square_attacked(16, Side::Black), Ok(None));
}

#[test]
fn is_square_attacked_bishop_does_not_wrap_diagonally() {
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece { piece: Piece::Bishop, side: Side::Black }); // a1

	let attacks_b2 = pos.is_square_attacked(9, Side::Black).expect("Err");
	assert!(attacks_b2.is_some()); // b2 is on valid diagonal

	assert_eq!(pos.is_square_attacked(15, Side::Black), Ok(None));
}

#[test]
fn is_square_attacked_queen_multiple_rays_with_blockers() {
	let mut pos = empty_position();

	pos.board[27] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // d4
	pos.board[0] = Some(ColoredPiece { piece: Piece::Queen, side: Side::Black }); // a1

	let attacks_d4 = pos.is_square_attacked(27, Side::Black).expect("Err");
	assert!(attacks_d4.is_some()); // d4 attacked by queen on a1 (diagonal)

	pos.board[18] = Some(ColoredPiece { piece: Piece::Pawn, side: Side::White }); // c3 blocks diagonal

	assert_eq!(pos.is_square_attacked(27, Side::Black), Ok(None));
}

#[test]
fn is_square_attacked_queen_on_h6_attacks_h8() {
	let mut pos = empty_position();

	pos.board[63] = Some(ColoredPiece { piece: Piece::King, side: Side::Black }); // h8
	pos.board[47] = Some(ColoredPiece { piece: Piece::Queen, side: Side::White }); // h6
	pos.board[0] = Some(ColoredPiece { piece: Piece::King, side: Side::White }); // a1

	let attacks = pos.is_square_attacked(63, Side::White).expect("is_square_attacked returned Err");

	assert!(attacks.is_some(), "h8 should be attacked by white queen on h6");
	assert!(has_square(attacks.as_ref().unwrap(), 47), "attacking square should be 47 (h6)");
}
