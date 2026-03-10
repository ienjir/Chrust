mod common;

use chrust_core::errors::ChessError;
use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::{ColoredPiece, Piece, Side, Square};
use common::{empty_position, has_move, has_to_square};

#[test]
fn queen_d4_empty_board() {
    let mut pos = empty_position();

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    });

    let moves = pos.queen_targets(27).expect("queen_targets returned Err");

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

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    });

    pos.board[29] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    let moves = pos.queen_targets(27).expect("queen_targets returned Err");

    assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
    assert!(!has_to_square(&moves, 29));
    assert!(!has_to_square(&moves, 30));
}

#[test]
fn queen_c7_enemy_f4() {
    let mut pos = empty_position();

    pos.board[50] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    });

    pos.board[29] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    });

    let moves = pos.queen_targets(50).expect("queen_targets returned Err");

    assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
    assert!(has_move(&moves, 50, 29, MoveKind::Capture));
    assert!(!has_to_square(&moves, 22));
}

#[test]
fn wrong_piece_e8() {
    let mut pos = empty_position();

    pos.board[60] = Some(ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    });

    assert_eq!(
        pos.queen_targets(60),
        Err(ChessError::WrongPieceType {
            expected_piece: Piece::Queen,
            found_piece: Piece::Knight,
        })
    );
}

#[test]
fn no_piece_d5() {
    let pos = empty_position();

    assert_eq!(
        pos.queen_targets(35),
        Err(ChessError::NoPieceOnSquare { square: 35 })
    )
}

#[test]
fn try_move_on_non_existing_square() {
    let pos = empty_position();

    assert_eq!(
        pos.queen_targets(65),
        Err(ChessError::NotASquareOnBoard { square: 65 })
    )
}

#[test]
fn wrong_side_returns_wrong_side_error() {
    // Black queen on the board but it's White's turn.
    let mut pos = empty_position(); // side_to_move = White

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::Black,
    });

    assert_eq!(
        pos.queen_targets(27),
        Err(ChessError::WrongSide {
            expected_side: Side::White,
            found_side: Side::Black,
        })
    );
}

#[test]
fn black_queen_d4_empty_board() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    pos.board[27] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::Black,
    });

    let moves = pos.queen_targets(27).expect("queen_targets returned Err");

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

    pos.board[0] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    });

    let moves = pos.queen_targets(0).expect("queen_targets returned Err");

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

    pos.board[28] = Some(ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    }); // e4

    // Add friendly blockers in some directions
    pos.board[36] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    }); // e5 (blocks north)

    pos.board[37] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    }); // f5 (blocks northeast)

    // Add enemy blockers in other directions
    pos.board[19] = Some(ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    }); // d3 (blocks southwest, capturable)

    let moves = pos.queen_targets(28).expect("queen_targets returned Err");

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
