mod common;

use chrust_core::errors::ChessError;
use chrust_core::moves::make_move::MoveKind;
use chrust_core::{ColoredPiece, Piece, Side};
use common::{empty_position, has_move, has_to_square};

#[test]
fn bishop_g7_empty_boad() {
    let mut pos = empty_position();

    pos.board[54] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    let moves = pos.bishop_targets(54).expect("bishop_targets returned Err");

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

    pos.board[7] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    let moves = pos.bishop_targets(7).expect("bishop_targets returned Err");

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

    pos.board[50] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    pos.board[29] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.bishop_targets(50).expect("bishop_targets returned Err");

    assert_eq!(moves.len(), 7);

    assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
    assert!(has_move(&moves, 50, 29, MoveKind::Capture));
    assert!(!has_to_square(&moves, 22));
}

#[test]
fn bishop_b3_friendly_e6() {
    let mut pos = empty_position();

    pos.board[17] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    pos.board[44] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });

    let moves = pos.bishop_targets(17).expect("bishop_targets returned Err");

    assert_eq!(moves.len(), 6);

    assert!(has_move(&moves, 17, 35, MoveKind::Quiet));
    assert!(!has_to_square(&moves, 44));
}

#[test]
fn wrong_piece_e8() {
    let mut pos = empty_position();

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    assert_eq!(
        pos.bishop_targets(60),
        Err(ChessError::WrongPieceType {
            expected_piece: Piece::Bishop,
            found_piece: Piece::Knight,
        })
    );
}

#[test]
fn no_piece_d5() {
    let pos = empty_position();

    assert_eq!(
        pos.bishop_targets(35),
        Err(ChessError::NoPieceOnSquare { square: 35 })
    )
}

#[test]
fn try_move_on_non_existing_square() {
    let pos = empty_position();

    assert_eq!(
        pos.bishop_targets(65),
        Err(ChessError::NotASquareOnBoard { square: 65 })
    )
}

#[test]
fn wrong_side_returns_wrong_side_error() {
    // Black bishop on the board but it's White's turn.
    let mut pos = empty_position(); // side_to_move = White

    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::Black,
    });

    assert_eq!(
        pos.bishop_targets(0),
        Err(ChessError::WrongSide {
            expected_side: Side::White,
            found_side: Side::Black,
        })
    );
}

#[test]
fn black_bishop_d4_empty_board() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::Black,
    });

    let moves = pos.bishop_targets(27).expect("bishop_targets returned Err");

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

    pos.board[16] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    let moves = pos.bishop_targets(16).expect("bishop_targets returned Err");

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

    pos.board[47] = Some(ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    });

    let moves = pos.bishop_targets(47).expect("bishop_targets returned Err");

    // Should not wrap to a-file
    assert!(!has_to_square(&moves, 32)); // a5
    assert!(!has_to_square(&moves, 48)); // a7

    // Should contain valid diagonal moves
    assert!(has_move(&moves, 47, 38, MoveKind::Quiet)); // g5
    assert!(has_move(&moves, 47, 54, MoveKind::Quiet)); // g7
}
