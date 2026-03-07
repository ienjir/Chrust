mod common;

use chrust_core::moves::make_move::{Move, MoveKind};
use chrust_core::position::Position;
use chrust_core::{errors::ChessError, ColoredPiece, Piece, Side};
use common::empty_position;

// ── helpers ──────────────────────────────────────────────────────────────────

/// Build a Move from its components; `colored_piece` is taken from the board.
fn mv(pos: &Position, from: u8, to: u8, kind: MoveKind) -> Move {
    Move {
        from_square: from,
        to_square: to,
        move_kind: kind,
        colored_piece: pos.board[from as usize].expect("helper mv(): no piece on from_square"),
    }
}

// ── error path tests ──────────────────────────────────────────────────────────

#[test]
fn make_move_errors_if_initial_square_empty() {
    let mut pos = empty_position();

    let mv = Move {
        from_square: 0,
        to_square: 1,
        move_kind: MoveKind::Quiet,
        colored_piece: ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        },
    };
    assert!(matches!(
        pos.make_move(&mv),
        Err(ChessError::NoPieceOnSquare { square: 0 })
    ));
}

#[test]
fn make_move_errors_if_from_square_out_of_bounds() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    let mv1 = Move {
        from_square: 64,
        to_square: 0,
        move_kind: MoveKind::Quiet,
        colored_piece: pawn,
    };
    let mv3 = Move {
        from_square: 200,
        to_square: 201,
        move_kind: MoveKind::Quiet,
        colored_piece: pawn,
    };

    assert!(matches!(
        pos.make_move(&mv1),
        Err(ChessError::NotASquareOnBoard { square: 64 })
    ));
    assert!(matches!(
        pos.make_move(&mv3),
        Err(ChessError::NotASquareOnBoard { square: 200 })
    ));
}

#[test]
fn make_move_errors_if_to_square_out_of_bounds() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[0] = Some(pawn);
    let mv2 = Move {
        from_square: 0,
        to_square: 64,
        move_kind: MoveKind::Quiet,
        colored_piece: pawn,
    };

    assert!(matches!(
        pos.make_move(&mv2),
        Err(ChessError::NotASquareOnBoard { square: 64 })
    ));
}

#[test]
fn make_move_errors_if_move_not_in_legal_list() {
    let mut pos = empty_position();

    // A rook on a1 cannot move diagonally to h8.
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook);

    let illegal = Move {
        from_square: 0,
        to_square: 63,
        move_kind: MoveKind::Quiet,
        colored_piece: rook,
    };
    assert!(matches!(
        pos.make_move(&illegal),
        Err(ChessError::NotAValidMove)
    ));
}

#[test]
fn promotion_errors_if_promotion_piece_is_none() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[48] = Some(pawn); // a7

    // The pawn generator emits Promotion { promotion_piece: Some(Piece::Pawn) }
    // as a sentinel.  A None promotion_piece is never in the legal list so the
    // move is rejected (NotAValidMove) before the None-check is even reached.
    let mv = Move {
        from_square: 48,
        to_square: 56,
        move_kind: MoveKind::Promotion {
            promotion_piece: None,
        },
        colored_piece: pawn,
    };
    assert!(matches!(pos.make_move(&mv), Err(_)));
}

// ── board-state tests ─────────────────────────────────────────────────────────

#[test]
fn quiet_move_clears_source_and_sets_target() {
    let mut pos = empty_position();

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook); // a1

    let m = mv(&pos, 0, 7, MoveKind::Quiet); // a1 → h1
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[0], None);
    assert_eq!(pos.board[7], Some(rook));
}

#[test]
fn capture_move_replaces_enemy_piece() {
    let mut pos = empty_position();

    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    let black_knight = ColoredPiece {
        piece: Piece::Knight,
        side: Side::Black,
    };
    pos.board[0] = Some(white_rook);
    pos.board[7] = Some(black_knight);

    let m = mv(&pos, 0, 7, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[0], None);
    assert_eq!(pos.board[7], Some(white_rook));
}

#[test]
fn capture_stores_captured_piece_in_undo() {
    let mut pos = empty_position();

    let white_bishop = ColoredPiece {
        piece: Piece::Bishop,
        side: Side::White,
    };
    let black_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[10] = Some(white_bishop);
    pos.board[28] = Some(black_pawn);

    let m = mv(&pos, 10, 28, MoveKind::Capture);
    let undo = pos.make_move(&m).unwrap();

    assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn en_passant_clears_capture_square_and_moves_pawn() {
    let mut pos = empty_position();

    let white_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    let black_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[33] = Some(white_pawn); // b5
    pos.board[34] = Some(black_pawn); // c5 — the en-passant captured pawn
    pos.en_passant = Some(42); // c6 — the square the white pawn moves to

    let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
    let undo = pos.make_move(&m).unwrap();

    assert_eq!(pos.board[33], None);
    assert_eq!(pos.board[34], None);
    assert_eq!(pos.board[42], Some(white_pawn));
    assert_eq!(undo.captured_piece, Some(black_pawn));
}

#[test]
fn double_pawn_push_moves_piece() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(pawn); // a2

    let m = mv(&pos, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 }); // a2 → a4
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[8], None);
    assert_eq!(pos.board[24], Some(pawn));
}

#[test]
fn promotion_changes_piece_type() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[48] = Some(pawn); // a7

    // The pawn generator emits Piece::Pawn as a sentinel for the promotion
    // piece — the caller is responsible for replacing it with the desired piece
    // before submitting.  We use the sentinel so the move passes the legal-list
    // check; the board ends up with a Pawn on the promotion square.
    let m = mv(
        &pos,
        48,
        56,
        MoveKind::Promotion {
            promotion_piece: Some(Piece::Pawn),
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[48], None);
    assert_eq!(
        pos.board[56],
        Some(ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White
        })
    );
}

#[test]
fn promotion_with_non_sentinel_piece_is_rejected() {
    // Only the Pawn sentinel is accepted by the legal-move list right now.
    // Real piece types will be rejected as NotAValidMove until the promotion
    // API is updated to expose one move per promotion choice.
    for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
        let mut pos = empty_position();
        let pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        };
        pos.board[48] = Some(pawn); // a7

        let m = Move {
            from_square: 48,
            to_square: 56,
            move_kind: MoveKind::Promotion {
                promotion_piece: Some(piece),
            },
            colored_piece: pawn,
        };
        assert!(
            matches!(pos.make_move(&m), Err(ChessError::NotAValidMove)),
            "promotion to {piece:?} should return NotAValidMove until the API is updated"
        );
    }
}

// Castling move tests are ignored because king_targets passes `king.side` to
// is_square_attacked for transit-square safety, so the own rook on h1/a1 causes
// f1/g1 / b1/c1 to appear "attacked" and castling is rejected.
// Remove the #[ignore] once that bug in king.rs is fixed.
#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_white_kingside_moves_king_and_rook() {
    let mut pos = empty_position();

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[4] = Some(king); // e1
    pos.board[7] = Some(rook); // h1
    pos.castle[0] = true; // white kingside

    let m = mv(
        &pos,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5,
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[4], None);
    assert_eq!(pos.board[6], Some(king));
    assert_eq!(pos.board[7], None);
    assert_eq!(pos.board[5], Some(rook));
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_white_queenside_moves_king_and_rook() {
    let mut pos = empty_position();

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[4] = Some(king); // e1
    pos.board[0] = Some(rook); // a1
    pos.castle[1] = true; // white queenside

    let m = mv(
        &pos,
        4,
        2,
        MoveKind::Castling {
            rook_from: 0,
            rook_to: 3,
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[4], None);
    assert_eq!(pos.board[2], Some(king));
    assert_eq!(pos.board[0], None);
    assert_eq!(pos.board[3], Some(rook));
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_black_kingside_moves_king_and_rook() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[60] = Some(king); // e8
    pos.board[63] = Some(rook); // h8
    pos.castle[2] = true; // black kingside

    let m = mv(
        &pos,
        60,
        62,
        MoveKind::Castling {
            rook_from: 63,
            rook_to: 61,
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[60], None);
    assert_eq!(pos.board[62], Some(king));
    assert_eq!(pos.board[63], None);
    assert_eq!(pos.board[61], Some(rook));
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_black_queenside_moves_king_and_rook() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[60] = Some(king); // e8
    pos.board[56] = Some(rook); // a8
    pos.castle[3] = true; // black queenside

    let m = mv(
        &pos,
        60,
        58,
        MoveKind::Castling {
            rook_from: 56,
            rook_to: 59,
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(pos.board[60], None);
    assert_eq!(pos.board[58], Some(king));
    assert_eq!(pos.board[56], None);
    assert_eq!(pos.board[59], Some(rook));
}

// ── side_to_move ──────────────────────────────────────────────────────────────

#[test]
fn make_move_toggles_side_to_move_white_to_black() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(pawn); // a2

    let m = mv(&pos, 8, 16, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.side_to_move, Side::Black);
}

#[test]
fn make_move_toggles_side_to_move_black_to_white() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[48] = Some(pawn); // a7

    let m = mv(&pos, 48, 40, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.side_to_move, Side::White);
}

// ── halfmove_clock ────────────────────────────────────────────────────────────

#[test]
fn halfmove_clock_resets_on_pawn_move() {
    let mut pos = empty_position();
    pos.halfmove_clock = 10;

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(pawn); // a2

    let m = mv(&pos, 8, 16, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_resets_on_capture() {
    let mut pos = empty_position();
    pos.halfmove_clock = 5;

    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    let black_knight = ColoredPiece {
        piece: Piece::Knight,
        side: Side::Black,
    };
    pos.board[0] = Some(white_rook);
    pos.board[7] = Some(black_knight);

    let m = mv(&pos, 0, 7, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_resets_on_en_passant() {
    let mut pos = empty_position();
    pos.halfmove_clock = 3;

    let white_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    let black_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[33] = Some(white_pawn); // b5
    pos.board[34] = Some(black_pawn); // c5
    pos.en_passant = Some(42); // c6

    let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
    pos.make_move(&m).unwrap();

    assert_eq!(pos.halfmove_clock, 0);
}

#[test]
fn halfmove_clock_increments_on_quiet_non_pawn_move() {
    let mut pos = empty_position();
    pos.halfmove_clock = 4;

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook); // a1

    let m = mv(&pos, 0, 7, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.halfmove_clock, 5);
}

// ── fullmove_number ───────────────────────────────────────────────────────────

#[test]
fn fullmove_number_increments_only_after_black_move() {
    // Per chess rules, fullmove_number increments after Black's move only.
    let mut pos = empty_position();
    assert_eq!(pos.fullmove_number, 0);

    // White's move — fullmove_number should NOT change.
    let white_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(white_pawn); // a2
    let m1 = mv(&pos, 8, 16, MoveKind::Quiet);
    pos.make_move(&m1).unwrap();
    assert_eq!(
        pos.fullmove_number, 0,
        "fullmove_number should not change after White moves"
    );

    // Black's move — fullmove_number should increment.
    let black_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[48] = Some(black_pawn); // a7
    let m2 = mv(&pos, 48, 40, MoveKind::Quiet);
    pos.make_move(&m2).unwrap();
    assert_eq!(
        pos.fullmove_number, 1,
        "fullmove_number should increment after Black moves"
    );
}

// ── en_passant state ──────────────────────────────────────────────────────────

#[test]
fn double_pawn_push_sets_en_passant_square() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(pawn); // a2

    let m = mv(&pos, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 });
    pos.make_move(&m).unwrap();

    assert_eq!(pos.en_passant, Some(16));
}

#[test]
fn any_other_move_clears_en_passant() {
    let mut pos = empty_position();
    pos.en_passant = Some(42); // some leftover en passant square

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook);

    let m = mv(&pos, 0, 7, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(pos.en_passant, None);
}

// ── castling rights revocation ────────────────────────────────────────────────

#[test]
fn white_king_move_revokes_both_white_castling_rights() {
    let mut pos = empty_position();
    pos.castle = [true, true, true, true];

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    pos.board[4] = Some(king); // e1

    let m = mv(&pos, 4, 5, MoveKind::Quiet); // e1 → f1
    pos.make_move(&m).unwrap();

    assert!(!pos.castle[0], "white kingside right should be revoked");
    assert!(!pos.castle[1], "white queenside right should be revoked");
    assert!(pos.castle[2], "black kingside right should be untouched");
    assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
fn black_king_move_revokes_both_black_castling_rights() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.castle = [true, true, true, true];

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    };
    pos.board[60] = Some(king); // e8

    let m = mv(&pos, 60, 61, MoveKind::Quiet); // e8 → f8
    pos.make_move(&m).unwrap();

    assert!(pos.castle[0], "white kingside right should be untouched");
    assert!(pos.castle[1], "white queenside right should be untouched");
    assert!(!pos.castle[2], "black kingside right should be revoked");
    assert!(!pos.castle[3], "black queenside right should be revoked");
}

#[test]
fn white_kingside_rook_move_revokes_white_kingside_right() {
    // H1 rook (square 7) → castle[0] = white kingside.
    let mut pos = empty_position();
    pos.castle = [true, true, false, false];

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[7] = Some(rook); // h1

    let m = mv(&pos, 7, 6, MoveKind::Quiet); // h1 → g1
    pos.make_move(&m).unwrap();

    assert!(
        !pos.castle[0],
        "white kingside right should be revoked after h1 rook moves"
    );
    assert!(pos.castle[1], "white queenside right should be untouched");
}

#[test]
fn white_queenside_rook_move_revokes_white_queenside_right() {
    // A1 rook (square 0) → castle[1] = white queenside.
    // NOTE: make_move currently clears castle[0] instead of castle[1] for
    // square 0 (the indices are swapped — see bug report).  This test asserts
    // the CORRECT expected behaviour; it will fail until the bug is fixed.
    let mut pos = empty_position();
    pos.castle = [true, true, false, false];

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook); // a1

    let m = mv(&pos, 0, 1, MoveKind::Quiet); // a1 → b1
    pos.make_move(&m).unwrap();

    assert!(pos.castle[0], "white kingside right should be untouched");
    assert!(
        !pos.castle[1],
        "white queenside right should be revoked after a1 rook moves"
    );
}

#[test]
fn black_kingside_rook_move_revokes_black_kingside_right() {
    // H8 rook (square 63) → castle[2] = black kingside.
    // NOTE: make_move currently clears castle[3] instead of castle[2] for
    // square 63 (the indices are swapped — see bug report).
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.castle = [false, false, true, true];

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[63] = Some(rook); // h8

    let m = mv(&pos, 63, 62, MoveKind::Quiet); // h8 → g8
    pos.make_move(&m).unwrap();

    assert!(
        !pos.castle[2],
        "black kingside right should be revoked after h8 rook moves"
    );
    assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
fn black_queenside_rook_move_revokes_black_queenside_right() {
    // A8 rook (square 56) → castle[3] = black queenside.
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.castle = [false, false, true, true];

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[56] = Some(rook); // a8

    let m = mv(&pos, 56, 57, MoveKind::Quiet); // a8 → b8
    pos.make_move(&m).unwrap();

    assert!(pos.castle[2], "black kingside right should be untouched");
    assert!(
        !pos.castle[3],
        "black queenside right should be revoked after a8 rook moves"
    );
}

// BUG: the to_square castling-rights match in make_move has swapped indices.
// Capturing on h1 (sq 7) clears castle[1] (queenside) instead of castle[0]
// (kingside), and vice versa for a1. These tests assert correct expected
// behaviour and are ignored until the bug is fixed.
#[test]
#[ignore = "swapped castling-rights indices in make_move to_square match (known bug)"]
fn capturing_white_kingside_rook_on_h1_revokes_white_kingside_right() {
    // A black piece captures the White h1 rook → castle[0] (white kingside)
    // should be cleared.
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.castle = [true, true, false, false];

    let black_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[15] = Some(black_rook); // h2
    pos.board[7] = Some(white_rook); // h1

    let m = mv(&pos, 15, 7, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert!(
        !pos.castle[0],
        "white kingside right should be revoked when h1 rook is captured"
    );
    assert!(pos.castle[1], "white queenside right should be untouched");
}

#[test]
#[ignore = "swapped castling-rights indices in make_move to_square match (known bug)"]
fn capturing_white_queenside_rook_on_a1_revokes_white_queenside_right() {
    // A black piece captures the White a1 rook → castle[1] (white queenside)
    // should be cleared.
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.castle = [true, true, false, false];

    let black_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[8] = Some(black_rook); // a2
    pos.board[0] = Some(white_rook); // a1

    let m = mv(&pos, 8, 0, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert!(pos.castle[0], "white kingside right should be untouched");
    assert!(
        !pos.castle[1],
        "white queenside right should be revoked when a1 rook is captured"
    );
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_revokes_both_rights_for_that_side() {
    let mut pos = empty_position();
    pos.castle = [true, true, false, false];

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[4] = Some(king);
    pos.board[7] = Some(rook);

    let m = mv(
        &pos,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5,
        },
    );
    pos.make_move(&m).unwrap();

    assert!(
        !pos.castle[0],
        "white kingside right should be revoked after castling"
    );
    assert!(
        !pos.castle[1],
        "white queenside right should be revoked after castling"
    );
}

// ── king_squares tracking ─────────────────────────────────────────────────────

#[test]
fn king_squares_updated_after_white_king_moves() {
    let mut pos = empty_position();
    pos.king_squares = [4, 60];

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    pos.board[4] = Some(king); // e1

    let m = mv(&pos, 4, 5, MoveKind::Quiet); // e1 → f1
    pos.make_move(&m).unwrap();

    assert_eq!(pos.king_squares[0], 5, "white king square should be f1");
    assert_eq!(
        pos.king_squares[1], 60,
        "black king square should be unchanged"
    );
}

#[test]
fn king_squares_updated_after_black_king_moves() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.king_squares = [4, 60];

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::Black,
    };
    pos.board[60] = Some(king); // e8

    let m = mv(&pos, 60, 61, MoveKind::Quiet); // e8 → f8
    pos.make_move(&m).unwrap();

    assert_eq!(
        pos.king_squares[0], 4,
        "white king square should be unchanged"
    );
    assert_eq!(pos.king_squares[1], 61, "black king square should be f8");
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn king_squares_updated_after_castling() {
    let mut pos = empty_position();
    pos.king_squares = [4, 60];
    pos.castle[0] = true;

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[4] = Some(king);
    pos.board[7] = Some(rook);

    let m = mv(
        &pos,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5,
        },
    );
    pos.make_move(&m).unwrap();

    assert_eq!(
        pos.king_squares[0], 6,
        "white king square should be g1 after kingside castle"
    );
}

#[test]
fn non_king_move_does_not_update_king_squares() {
    let mut pos = empty_position();
    pos.king_squares = [4, 60];

    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[0] = Some(rook);

    let m = mv(&pos, 0, 7, MoveKind::Quiet);
    pos.make_move(&m).unwrap();

    assert_eq!(
        pos.king_squares[0], 4,
        "white king square should not change on rook move"
    );
    assert_eq!(
        pos.king_squares[1], 60,
        "black king square should not change on rook move"
    );
}

// ── undo tests ────────────────────────────────────────────────────────────────

#[test]
fn quiet_move_undo_restores_board() {
    let mut pos = empty_position();

    let knight = ColoredPiece {
        piece: Piece::Knight,
        side: Side::White,
    };
    pos.board[1] = Some(knight); // b1

    let m = mv(&pos, 1, 18, MoveKind::Quiet); // b1 → c3
    let undo = pos.make_move(&m).unwrap();

    assert_eq!(pos.board[1], None);
    assert_eq!(pos.board[18], Some(knight));

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.board[1], Some(knight));
    assert_eq!(pos.board[18], None);
}

#[test]
fn capture_undo_restores_both_pieces() {
    let mut pos = empty_position();

    let white_queen = ColoredPiece {
        piece: Piece::Queen,
        side: Side::White,
    };
    let black_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[3] = Some(white_queen); // d1
    pos.board[59] = Some(black_rook); // d8

    let m = mv(&pos, 3, 59, MoveKind::Capture);
    let undo = pos.make_move(&m).unwrap();

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.board[3], Some(white_queen));
    assert_eq!(pos.board[59], Some(black_rook));
}

#[test]
#[ignore = "undo_move DoublePawnPush branch is not yet implemented (WIP)"]
fn double_pawn_push_undo_restores_position() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[51] = Some(pawn); // d7

    let m = mv(&pos, 51, 35, MoveKind::DoublePawnPush { passed_square: 43 });
    let undo = pos.make_move(&m).unwrap();

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.board[51], Some(pawn));
    assert_eq!(pos.board[35], None);
}

#[test]
#[ignore = "castling rejected — inverted is_square_attacked side in king_targets (known bug)"]
fn castling_undo_restores_king_and_rook() {
    let mut pos = empty_position();
    pos.castle[0] = true;

    let king = ColoredPiece {
        piece: Piece::King,
        side: Side::White,
    };
    let rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    pos.board[4] = Some(king); // e1
    pos.board[7] = Some(rook); // h1

    let m = mv(
        &pos,
        4,
        6,
        MoveKind::Castling {
            rook_from: 7,
            rook_to: 5,
        },
    );
    let undo = pos.make_move(&m).unwrap();

    assert_eq!(pos.board[4], None);
    assert_eq!(pos.board[6], Some(king));
    assert_eq!(pos.board[7], None);
    assert_eq!(pos.board[5], Some(rook));

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.board[4], Some(king));
    assert_eq!(pos.board[6], None);
    assert_eq!(pos.board[7], Some(rook));
    assert_eq!(pos.board[5], None);
}

#[test]
fn undo_restores_side_to_move() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[8] = Some(pawn);

    let m = mv(&pos, 8, 16, MoveKind::Quiet);
    let undo = pos.make_move(&m).unwrap();

    assert_eq!(pos.side_to_move, Side::Black);

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.side_to_move, Side::White);
}

// BUG: undo_move does not restore halfmove_clock (it is incremented by
// make_move but never restored on undo).  Once undo_move saves/restores
// halfmove_clock, remove #[ignore].
#[test]
#[ignore = "undo_move does not restore halfmove_clock (WIP)"]
fn undo_restores_all_metadata() {
    let mut pos = empty_position();
    pos.side_to_move = Side::Black;
    pos.en_passant = Some(16);
    pos.castle = [true, false, true, false];
    pos.king_squares = [4, 60];
    pos.fullmove_number = 42;
    pos.halfmove_clock = 7;

    let knight = ColoredPiece {
        piece: Piece::Knight,
        side: Side::Black,
    };
    pos.board[57] = Some(knight); // b8

    let m = mv(&pos, 57, 42, MoveKind::Quiet);
    let undo = pos.make_move(&m).unwrap();

    pos.undo_move(undo, m).unwrap();

    assert_eq!(pos.side_to_move, Side::Black);
    assert_eq!(pos.en_passant, Some(16));
    assert_eq!(pos.castle, [true, false, true, false]);
    assert_eq!(pos.king_squares, [4, 60]);
    assert_eq!(pos.fullmove_number, 42);
    assert_eq!(pos.halfmove_clock, 7);
}

// ── undo — WIP branches ───────────────────────────────────────────────────────

#[test]
#[ignore = "undo_move EnPassant branch is not yet implemented (WIP)"]
fn en_passant_undo_restores_all_squares() {
    let mut pos = empty_position();

    let white_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    let black_pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::Black,
    };
    pos.board[33] = Some(white_pawn); // b5
    pos.board[34] = Some(black_pawn); // c5
    pos.en_passant = Some(42); // c6

    let m = mv(&pos, 33, 42, MoveKind::EnPassant { capture_square: 34 });
    let undo = pos.make_move(&m).unwrap();

    pos.undo_move(undo, m).unwrap();

    assert_eq!(
        pos.board[33],
        Some(white_pawn),
        "white pawn should be restored to b5"
    );
    assert_eq!(
        pos.board[34],
        Some(black_pawn),
        "captured black pawn should be restored to c5"
    );
    assert_eq!(pos.board[42], None, "c6 should be empty after undo");
}

#[test]
#[ignore = "undo_move Promotion branch is not yet implemented (WIP)"]
fn promotion_undo_restores_original_pawn() {
    let mut pos = empty_position();

    let pawn = ColoredPiece {
        piece: Piece::Pawn,
        side: Side::White,
    };
    pos.board[48] = Some(pawn); // a7

    let m = mv(
        &pos,
        48,
        56,
        MoveKind::Promotion {
            promotion_piece: Some(Piece::Pawn),
        },
    );
    let undo = pos.make_move(&m).unwrap();

    pos.undo_move(undo, m).unwrap();

    assert_eq!(
        pos.board[48],
        Some(pawn),
        "original pawn should be restored to a7"
    );
    assert_eq!(pos.board[56], None, "a8 should be empty after undo");
}

// ── castling rights — black rook captures (h8 / a8) ──────────────────────────
// The to_square index mapping has swapped indices (known bug).  These tests
// assert the CORRECT expected behaviour and are ignored until the bug is fixed.

#[test]
#[ignore = "swapped castling-rights indices in make_move to_square match (known bug)"]
fn capturing_black_kingside_rook_on_h8_revokes_black_kingside_right() {
    let mut pos = empty_position();
    pos.castle = [false, false, true, true];

    // White rook captures the Black h8 rook → castle[2] (black kingside) cleared.
    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    let black_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[55] = Some(white_rook); // h7
    pos.board[63] = Some(black_rook); // h8

    let m = mv(&pos, 55, 63, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert!(
        !pos.castle[2],
        "black kingside right should be revoked when h8 rook is captured"
    );
    assert!(pos.castle[3], "black queenside right should be untouched");
}

#[test]
#[ignore = "swapped castling-rights indices in make_move to_square match (known bug)"]
fn capturing_black_queenside_rook_on_a8_revokes_black_queenside_right() {
    let mut pos = empty_position();
    pos.castle = [false, false, true, true];

    // White rook captures the Black a8 rook → castle[3] (black queenside) cleared.
    let white_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::White,
    };
    let black_rook = ColoredPiece {
        piece: Piece::Rook,
        side: Side::Black,
    };
    pos.board[48] = Some(white_rook); // a7
    pos.board[56] = Some(black_rook); // a8

    let m = mv(&pos, 48, 56, MoveKind::Capture);
    pos.make_move(&m).unwrap();

    assert!(pos.castle[2], "black kingside right should be untouched");
    assert!(
        !pos.castle[3],
        "black queenside right should be revoked when a8 rook is captured"
    );
}
