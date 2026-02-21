use crate::{
    errors::ChessError,
    file,
    moves::make_move::{Move, MoveKind},
    position::Position,
    rank, Piece, Square,
};

impl Position {
    pub fn knight_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(8);

        if !(0..=63).contains(&from_square) {
            return Err(ChessError::NotASquareOnBoard {
                square: from_square,
            });
        }

        let knight = match self.board[from_square as usize] {
            Some(p) => p,
            None => {
                return Err(ChessError::NoPieceOnSquare {
                    square: from_square,
                })
            }
        };

        if knight.piece != Piece::Knight {
            return Err(ChessError::WrongPieceTypeOnSquare {
                expected_piece: Piece::Knight,
                found_piece: knight.piece,
                square: from_square,
            });
        }

        let from_file_i = file(from_square) as i16;
        let from_rank_i = rank(from_square) as i16;

        let directions: [i16; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

        for direction in directions {
            let candidate_square_i = from_square as i16 + direction;

            if !(0..=63).contains(&candidate_square_i) {
                continue;
            }

            let file_difference_i = (file(candidate_square_i as u8) as i16 - from_file_i).abs();
            let rank_difference_i = (rank(candidate_square_i as u8) as i16 - from_rank_i).abs();

            let is_allowed_jump = (file_difference_i == 2 && rank_difference_i == 1)
                || (file_difference_i == 1 && rank_difference_i == 2);

            if !is_allowed_jump {
                continue;
            }

            let candidate_occupant = self.board[candidate_square_i as usize];
            match candidate_occupant {
                None => {
                    target_moves.push(Move {
                        colored_piece: knight,
                        from_square: from_square,
                        to_square: candidate_square_i as u8,
                        move_kind: MoveKind::Quiet,
                    });
                }
                Some(colored_piece) => {
                    if colored_piece.side != knight.side {
                        target_moves.push(Move {
                            colored_piece: knight,
                            from_square: from_square,
                            to_square: candidate_square_i as u8,
                            move_kind: MoveKind::Capture,
                        });
                    }
                    continue;
                }
            };
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
    fn knight_e4_empty_board() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.knight_targets(28).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 8);

        assert!(has_move(&moves, 28, 43, MoveKind::Quiet));
        assert!(has_move(&moves, 28, 45, MoveKind::Quiet));
        assert!(has_move(&moves, 28, 11, MoveKind::Quiet));
        assert!(has_move(&moves, 28, 22, MoveKind::Quiet));
    }

    #[test]
    fn knight_a8_corner_test() {
        let mut pos = empty_position();

        pos.board[56] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.knight_targets(56).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 2);

        assert!(has_move(&moves, 56, 50, MoveKind::Quiet));
        assert!(has_move(&moves, 56, 41, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 39));
        assert!(!has_to_square(&moves, 16));
    }

    #[test]
    fn knight_g8_enemy_h6() {
        let mut pos = empty_position();

        pos.board[62] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        pos.board[47] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.knight_targets(62).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 3);

        assert!(has_move(&moves, 62, 47, MoveKind::Capture));
    }

    #[test]
    fn knight_d1_friendly_f2() {
        let mut pos = empty_position();

        pos.board[3] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        pos.board[13] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.knight_targets(3).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 3);

        assert!(!has_to_square(&moves, 13));
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position();

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::White,
        });

        assert_eq!(
            pos.knight_targets(60),
            Err(ChessError::WrongPieceTypeOnSquare {
                expected_piece: Piece::Knight,
                found_piece: Piece::King,
                square: 60
            })
        );
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position();

        assert_eq!(
            pos.knight_targets(35),
            Err(ChessError::NoPieceOnSquare { square: 35 })
        )
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(
            pos.knight_targets(65),
            Err(ChessError::NotASquareOnBoard { square: 65 })
        )
    }
}
