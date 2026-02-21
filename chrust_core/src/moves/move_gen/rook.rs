use crate::{
    errors::ChessError,
    file,
    moves::make_move::{Move, MoveKind},
    position::Position,
    rank, Piece, Square,
};

impl Position {
    pub fn rook_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(14);

        if !(0..=63).contains(&from_square) {
            return Err(ChessError::NotASquareOnBoard {
                square: from_square,
            });
        }

        let rook = match self.board[from_square as usize] {
            Some(p) => p,
            None => {
                return Err(ChessError::NoPieceOnSquare {
                    square: from_square,
                })
            }
        };

        if rook.piece != Piece::Rook {
            return Err(ChessError::WrongPieceTypeOnSquare {
                expected_piece: Piece::Rook,
                found_piece: rook.piece,
                square: from_square,
            });
        }

        let directions: [i16; 4] = [-8, 8, -1, 1];

        for direction in directions {
            let mut step_from_i: i16 = from_square as i16;
            loop {
                let step_to_i = step_from_i + direction;

                if !(0..=63).contains(&step_to_i) {
                    break;
                }

                let file_difference_i =
                    (file(step_to_i as u8) as i16 - file(step_from_i as u8) as i16).abs();
                let rank_difference_i =
                    (rank(step_to_i as u8) as i16 - rank(step_from_i as u8) as i16).abs();

                if direction.abs() == 8 {
                    if file_difference_i != 0 || rank_difference_i != 1 {
                        break;
                    }
                } else {
                    if file_difference_i != 1 || rank_difference_i != 0 {
                        break;
                    }
                }

                let candidate_occupant = self.board[step_to_i as usize];
                match candidate_occupant {
                    None => {
                        target_moves.push(Move {
                            colored_piece: rook,
                            from_square: from_square,
                            to_square: step_to_i as u8,
                            move_kind: MoveKind::Quiet,
                        });
                        step_from_i = step_to_i;
                    }
                    Some(colored_piece) => {
                        if colored_piece.side != rook.side {
                            target_moves.push(Move {
                                colored_piece: rook,
                                from_square: from_square,
                                to_square: step_to_i as u8,
                                move_kind: MoveKind::Capture,
                            });
                        }
                        break;
                    }
                }
            }
        }

        Ok(target_moves)
    }
}

#[cfg(test)]
mod tests {
    use crate::ColoredPiece;

    use super::*;

    fn empty_position() -> Position {
        Position {
            board: [None; 64],
            side_to_move: crate::Side::White,
            castle: [false; 4],
            en_passant: None,
            king_squares: [4, 60],
        }
    }

    fn has_move(moves: &[Move], from: Square, to: Square, kind: MoveKind) -> bool {
        moves
            .iter()
            .any(|m| m.from_square == from && m.to_square == to && m.move_kind == kind)
    }

    fn has_to_square(moves: &[Move], to: Square) -> bool {
        moves.iter().any(|m| m.to_square == to)
    }

    #[test]
    fn rook_h8_empty_board() {
        let mut pos = empty_position();

        pos.board[63] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[19] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[26] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert!(moves.is_empty());
    }

    #[test]
    fn rook_d4_captures_adjacent_enemy_pieces() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[19] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[26] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
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
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[43] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[25] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
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
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(
            pos.rook_targets(60),
            Err(ChessError::WrongPieceTypeOnSquare {
                expected_piece: Piece::Rook,
                found_piece: Piece::Knight,
                square: 60
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
}
