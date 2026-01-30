use crate::{Piece, Side, Square, errors::{MoveError}, position::Position};

#[derive(PartialEq, Debug, Clone)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
    pub move_kind: MoveKind, 
}

#[derive(PartialEq, Debug, Clone)]
pub enum MoveKind {
    Quiet,
    Capture,
    DoublePawnPush { passed_square: Square },
    EnPassant { capture_square: Square }, 
    Promotion { promotion_piece: Option<Piece> },
}

impl Position {
    pub fn make_move_validated(&mut self, mv: &Move) -> Result<Position, MoveError> {
        match self.en_passant {
            Some(x) => println!("Test: {x}"),
            None => println!("none")
        }
        let mut position = match self.make_move_unvalidated(mv) {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if let MoveKind::DoublePawnPush { passed_square } = mv.move_kind {
            println!("Passed: {passed_square}");
            position.en_passant = Some(passed_square);
        } else {
            position.en_passant = None;
        }

        Ok(position)
    }

    pub fn make_move_unvalidated(&self, mv: &Move) -> Result<Position, MoveError> {
        if mv.from_square > 63 || mv.to_square > 63 {
            return Err(MoveError::OutOfBounds);
        }

        let potential_piece = self.board[mv.from_square as usize];
        let mut piece = match potential_piece {
            Some(x) => x,
            None => return Err(MoveError::NoPieceOnInitalSquare(mv.from_square))
        };

        let mut next_position = self.clone();

        match mv.move_kind {
            MoveKind::Quiet => {
                next_position.board[mv.from_square as usize] = None;
                next_position.board[mv.to_square as usize] = Some(piece);
            },
            MoveKind::Capture => {
                next_position.board[mv.from_square as usize] = None;
                next_position.board[mv.to_square as usize] = Some(piece);
            },
            MoveKind::EnPassant { capture_square } =>  {
                next_position.board[capture_square as usize] = None;
                next_position.board[mv.from_square as usize] = None;
                next_position.board[mv.to_square as usize] = Some(piece);
            },
            MoveKind::DoublePawnPush { passed_square: _ } => {
                next_position.board[mv.from_square as usize] = None;
                next_position.board[mv.to_square as usize] = Some(piece);
            }
            MoveKind::Promotion { promotion_piece } =>  {
                if promotion_piece.is_none() {
                    return Err(MoveError::PromotionPieceCantBeEmpty);
                }

                piece.piece = promotion_piece.expect("Promotion piece is somehow none"); 
                next_position.board[mv.from_square as usize]  = None;
                next_position.board[mv.to_square as usize] = Some(piece);
            } 
        };



        next_position.side_to_move = match next_position.side_to_move {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        Ok(next_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColoredPiece, Piece};

    use super::*;

    fn empty_position() -> Position {
        Position {
            board: [None; 64],
            side_to_move: Side::White,
            castle: [false; 4],
            en_passant: None,
        }
    }

    #[test]
    fn make_move_errors_if_initial_square_empty() {
        let pos = empty_position();

        let mv = Move {
            from_square: 0,
            to_square: 1,
            move_kind: MoveKind::Quiet,
        };
        let err = pos.make_move_unvalidated(&mv).unwrap_err();
        match err {
            MoveError::NoPieceOnInitalSquare(sq) => assert_eq!(sq, 0),
            _ => panic!("expected NoPieceOnInitalSquare"),
        }
    }

    #[test]
    fn make_move_errors_if_out_of_bounds() {
        let pos = empty_position();

        let mv1 = Move {
            from_square: 64,
            to_square: 0,
            move_kind: MoveKind::Quiet,
        };
        let mv2 = Move {
            from_square: 0,
            to_square: 64,
            move_kind: MoveKind::Quiet,
        };
        let mv3 = Move {
            from_square: 200,
            to_square: 201,
            move_kind: MoveKind::Quiet,
        };

        assert!(matches!(pos.make_move_unvalidated(&mv1), Err(MoveError::OutOfBounds)));
        assert!(matches!(pos.make_move_unvalidated(&mv2), Err(MoveError::OutOfBounds)));
        assert!(matches!(pos.make_move_unvalidated(&mv3), Err(MoveError::OutOfBounds)));
    }

    #[test]
    fn make_move_moves_piece_clears_source_and_sets_target() {
        let mut pos = empty_position();

        let rook = ColoredPiece {
            piece: Piece::Rook,
            side: Side::White,
        };

        pos.board[0] = Some(rook); // a1

        let mv = Move {
            from_square: 0,
            to_square: 7,
            move_kind: MoveKind::Quiet,
        };
        let next = pos.make_move_unvalidated(&mv).unwrap(); // a1 -> h1

        assert_eq!(next.board[0], None);
        assert_eq!(next.board[7], Some(rook));

        assert_eq!(pos.board[0], Some(rook));
        assert_eq!(pos.board[7], None);
    }

    #[test]
    fn make_move_overwrites_target_piece_capture_like() {
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

        let mv = Move {
            from_square: 0,
            to_square: 7,
            move_kind: MoveKind::Capture,
        };
        let next = pos.make_move_unvalidated(&mv).unwrap();

        assert_eq!(next.board[0], None);
        assert_eq!(next.board[7], Some(white_rook));
    }

    #[test]
    fn make_move_toggles_side_to_move() {
        let mut pos = empty_position();

        let pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        };
        pos.board[0] = Some(pawn);

        pos.side_to_move = Side::White;
        let mv1 = Move {
            from_square: 0,
            to_square: 1,
            move_kind: MoveKind::Quiet,
        };
        let next = pos.make_move_unvalidated(&mv1).unwrap();
        assert_eq!(next.side_to_move, Side::Black);

        pos.side_to_move = Side::Black;
        let mv2 = Move {
            from_square: 0,
            to_square: 1,
            move_kind: MoveKind::Quiet,
        };
        let next2 = pos.make_move_unvalidated(&mv2).unwrap();
        assert_eq!(next2.side_to_move, Side::White);
    }

    #[test]
    fn make_move_en_passant_clears_capture_square() {
        let mut pos = empty_position();

        let white_pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        };
        let black_pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::Black,
        };

        pos.board[12] = Some(white_pawn); // e2
        pos.board[20] = Some(black_pawn); // e3 capture square for en passant

        let mv = Move {
            from_square: 12,
            to_square: 21, // f3
            move_kind: MoveKind::EnPassant { capture_square: 20 },
        };
        let next = pos.make_move_unvalidated(&mv).unwrap();

        assert_eq!(next.board[20], None);
        assert_eq!(next.board[12], None);
        assert_eq!(next.board[21], Some(white_pawn));
    }

    #[test]
    fn make_move_double_pawn_push_moves_piece() {
        let mut pos = empty_position();

        let pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        };
        pos.board[8] = Some(pawn); // a2

        let mv = Move {
            from_square: 8,
            to_square: 24, // a4
            move_kind: MoveKind::DoublePawnPush { passed_square: 16 },
        };
        let next = pos.make_move_unvalidated(&mv).unwrap();

        assert_eq!(next.board[8], None);
        assert_eq!(next.board[24], Some(pawn));
    }
}
