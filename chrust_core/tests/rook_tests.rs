mod common;

use chrust_core::errors::ChessError;
use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::{ColoredPiece, Piece, Side, Square};
use common::{empty_position, has_move, has_to_square};

#[test]
fn rook_h8_empty_board() {
    let mut pos = empty_position();

    pos.board[63] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    let moves = pos.rook_targets(63).expect("knight_targets returned Err");

    assert_eq!(moves.len(), 14);

    assert!(has_move(&moves, 63, 62, MoveKind::Quiet));
    assert!(has_move(&moves, 63, 56, MoveKind::Quiet));
    assert!(has_move(&moves, 63, 55, MoveKind::Quiet));
    assert!(has_move(&moves, 63, 7, MoveKind::Quiet));
}

#[test]
fn rook_d4_empty_board() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

    assert_eq!(moves.len(), 14);

    assert!(has_move(&moves, 27, 24, MoveKind::Quiet));
    assert!(has_move(&moves, 27, 31, MoveKind::Quiet));
    assert!(has_move(&moves, 27, 3, MoveKind::Quiet));
    assert!(has_move(&moves, 27, 26, MoveKind::Quiet));
}

#[test]
fn rook_d4_blocked_by_friendly_piece_f4() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    pos.board[29] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

    assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
    assert!(!has_to_square(&moves, 29));
    assert!(!has_to_square(&moves, 30));
}

#[test]
fn rook_d4_captures_enemy_f4_and_stops() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    pos.board[29] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::Black,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

    assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
    assert!(has_move(&moves, 27, 29, MoveKind::Capture));
    assert!(!has_to_square(&moves, 30));
}

#[test]
fn rook_a1_empty_board() {
    let mut pos = empty_position();

    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    let moves = pos.rook_targets(0).expect("knight_targets returned Err");

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

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    pos.board[35] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });
    pos.board[19] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });
    pos.board[26] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });
    pos.board[28] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

    assert!(moves.is_empty());
}

#[test]
fn rook_d4_captures_adjacent_enemy_pieces() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    pos.board[35] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });
    pos.board[19] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });
    pos.board[26] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });
    pos.board[28] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

    assert_eq!(moves.len(), 4);
    assert!(has_move(&moves, 27, 35, MoveKind::Capture));
    assert!(has_move(&moves, 27, 19, MoveKind::Capture));
    assert!(has_move(&moves, 27, 26, MoveKind::Capture));
    assert!(has_move(&moves, 27, 28, MoveKind::Capture));
}

#[test]
fn rook_d4_mixed_blockers() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });

    pos.board[43] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    });
    pos.board[25] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.rook_targets(27).expect("knight_targets returned Err");

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
fn wrong_piece_e8() {
    let mut pos = empty_position();

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    assert_eq!(
        pos.rook_targets(60),
        Err(ChessError::WrongPieceType {
            expected_piece: Piece::Rook,
            found_piece: Piece::Knight,
        })
    );
}

#[test]
fn no_piece_d5() {
    let pos = empty_position();

    assert_eq!(
        pos.rook_targets(35),
        Err(ChessError::NoPieceOnSquare { square: 35 })
    )
}

#[test]
fn try_move_on_non_existing_square() {
    let pos = empty_position();

    assert_eq!(
        pos.rook_targets(65),
        Err(ChessError::NotASquareOnBoard { square: 65 })
    )
}

#[test]
fn wrong_side_returns_wrong_side_error() {
    // Black rook on the board but it's White's turn.
    let mut pos = empty_position(); // side_to_move = White

    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    });

    assert_eq!(
        pos.rook_targets(0),
        Err(ChessError::WrongSide {
            expected_side: Side::White,
            found_side: Side::Black,
        })
    );
}

#[test]
fn black_rook_d4_empty_board() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    });

    let moves = pos.rook_targets(27).expect("rook_targets returned Err");

    assert_eq!(moves.len(), 14);
    assert!(has_move(&moves, 27, 3, MoveKind::Quiet)); // d1
    assert!(has_move(&moves, 27, 59, MoveKind::Quiet)); // d8
    assert!(has_move(&moves, 27, 24, MoveKind::Quiet)); // a4
    assert!(has_move(&moves, 27, 31, MoveKind::Quiet)); // h4
}
