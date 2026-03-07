mod common;

use chrust_core::errors::ChessError;
use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::{ColoredPiece, Piece, Side, Square};
use common::{empty_position, has_move, has_to_square};

#[test]
fn king_c5_empty_board() {
    let mut pos = empty_position();

    pos.board[34] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });

    let moves = pos.king_targets(34).expect("king_targets returned Err");

    assert_eq!(moves.len(), 8);

    assert!(has_move(&moves, 34, 42, MoveKind::Quiet));
    assert!(has_move(&moves, 34, 27, MoveKind::Quiet));
    assert!(has_move(&moves, 34, 25, MoveKind::Quiet));
    assert!(has_move(&moves, 34, 35, MoveKind::Quiet));
}

#[test]
fn king_h1_corner_test() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[7] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    });

    let moves = pos.king_targets(7).expect("king_targets returned Err");

    assert_eq!(moves.len(), 3);

    assert!(has_move(&moves, 7, 15, MoveKind::Quiet));
    assert!(has_move(&moves, 7, 14, MoveKind::Quiet));
    assert!(has_move(&moves, 7, 6, MoveKind::Quiet));
    assert!(!has_to_square(&moves, 8));
    assert!(!has_to_square(&moves, 16));
}

#[test]
fn king_d5_enemy_e6() {
    let mut pos = empty_position();

    pos.board[35] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });

    pos.board[44] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.king_targets(35).expect("king_targets returned Err");

    assert_eq!(moves.len(), 8);

    assert!(has_move(&moves, 35, 44, MoveKind::Capture));
}

#[test]
fn king_h5_friendly_g4() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[39] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    });

    pos.board[30] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.king_targets(39).expect("king_targets returned Err");

    assert_eq!(moves.len(), 4);

    assert!(!has_to_square(&moves, 30));
}

#[test]
fn wrong_piece_e8() {
    let mut pos = empty_position();

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    assert_eq!(
        pos.king_targets(60),
        Err(ChessError::WrongPieceType {
            expected_piece: Piece::King,
            found_piece: Piece::Knight,
        })
    );
}

#[test]
fn no_piece_d5() {
    let pos = empty_position();

    assert_eq!(
        pos.king_targets(35),
        Err(ChessError::NoPieceOnSquare { square: 35 })
    )
}

#[test]
fn try_move_on_non_existing_square() {
    let pos = empty_position();

    assert_eq!(
        pos.king_targets(65),
        Err(ChessError::NotASquareOnBoard { square: 65 })
    )
}

#[test]
fn king_castling_white_kingside_allowed_and_clear() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[7] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.castle[0] = true; // white kingside

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(has_move(
        &moves,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5
        }
    ));
}

#[test]
fn king_castling_white_queenside_blocked_by_piece() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.board[3] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    }); // block d1
    pos.castle[1] = true; // white queenside

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(!has_move(
        &moves,
        4,
        2,
        MoveKind::Castling {
            rook_from: 0,
            rook_to: 3
        }
    ));
}

#[test]
fn king_castling_black_queenside_allowed_and_clear() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    });
    pos.board[56] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    });
    pos.castle[3] = true; // black queenside

    let moves = pos.king_targets(60).expect("king_targets returned Err");

    assert!(has_move(
        &moves,
        60,
        58,
        MoveKind::Castling {
            rook_from: 56,
            rook_to: 59
        }
    ));
}

#[test]
fn king_castling_white_kingside_disallowed_when_in_check() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[7] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.board[60] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    }); // attacks e1
    pos.castle[0] = true;

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(!has_move(
        &moves,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5
        }
    ));
}

#[test]
fn king_castling_white_kingside_disallowed_when_path_square_attacked() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[7] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.board[61] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    }); // attacks f1
    pos.castle[0] = true;

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(!has_move(
        &moves,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5
        }
    ));
}

#[test]
fn king_castling_white_kingside_disallowed_when_destination_attacked() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[7] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.board[62] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    }); // attacks g1
    pos.castle[0] = true;

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(!has_move(
        &moves,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5
        }
    ));
}

#[test]
fn wrong_side_returns_wrong_side_error() {
    // Black king on the board but it's White's turn.
    let mut pos = empty_position(); // side_to_move = White

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    });

    assert_eq!(
        pos.king_targets(60),
        Err(ChessError::WrongSide {
            expected_side: Side::White,
            found_side: Side::Black,
        })
    );
}

#[test]
fn king_castling_white_queenside_allowed_and_clear() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    });
    pos.castle[1] = true; // white queenside

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(has_move(
        &moves,
        4,
        2,
        MoveKind::Castling {
            rook_from: 0,
            rook_to: 3
        }
    ));
}

#[test]
fn king_castling_black_kingside_allowed_and_clear() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    });
    pos.board[63] = Some(ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    });
    pos.castle[2] = true; // black kingside

    let moves = pos.king_targets(60).expect("king_targets returned Err");

    assert!(has_move(
        &moves,
        60,
        62,
        MoveKind::Castling {
            rook_from: 63,
            rook_to: 61
        }
    ));
}

#[test]
fn king_castling_white_kingside_disallowed_when_rook_missing() {
    let mut pos = empty_position();

    pos.board[4] = Some(ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    });
    pos.castle[0] = true;

    let moves = pos.king_targets(4).expect("king_targets returned Err");

    assert!(!has_move(
        &moves,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5
        }
    ));
}
