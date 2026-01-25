use crate::{Piece, Side, Square, file, moves::move_gen::move_gen::MoveGenError, position::Position, rank};

impl Position {
    // Without en passant / promotion
    pub fn pawn_targets(&self, from_square: Square)  -> Result<Vec<Square>, MoveGenError> {
        let mut target_squares = Vec::with_capacity(4);

        if !(0..=63).contains(&from_square) {
            return Err(MoveGenError::NotASquareOnBoard {square: from_square})
        }

        let pawn = match self.board[from_square as usize]  {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: from_square })
        };

        if pawn.piece != Piece::Pawn {
            return Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Pawn, found_piece: pawn.piece, square: from_square})
        }

        let (forward, start_rank, capture_offsets): (i16, i16, [i16; 2]) = match pawn.side {
            Side::White => (8, 1, [7, 9]),
            Side::Black => (-8, 6, [-7, -9]),
        };

        let from_file_i = file(from_square) as i16;
        let from_rank_i = rank(from_square) as i16;

        let mut forward_1_is_empty = false;
        let forward_1_candidate_i = from_square as i16 + forward; 

        if (0..=63).contains(&forward_1_candidate_i) {
            let file_difference_i = (file(forward_1_candidate_i as u8) as i16 - from_file_i).abs();

            if file_difference_i == 0 {
                if self.board[forward_1_candidate_i as usize].is_none() {
                    target_squares.push(forward_1_candidate_i as u8);
                    forward_1_is_empty = true;
                }
            }
        }

        if from_rank_i == start_rank && forward_1_is_empty {
            let forward_2_candidate_i = from_square as i16 + (forward * 2);
            if (0..=63).contains(&forward_2_candidate_i) {
                let file_difference_i = (file(forward_2_candidate_i as u8) as i16 - from_file_i).abs();
                if file_difference_i == 0 {
                    if self.board[forward_2_candidate_i as usize].is_none() {
                        target_squares.push(forward_2_candidate_i as u8);
                    }
                }
            }
        }

        for capture_offset in capture_offsets {
            let capture_candidate = from_square as i16 + capture_offset;

            if !(0..=63).contains(&capture_candidate) {
                continue;
            }

            let file_difference_i = (file(capture_candidate as u8) as i16 - from_file_i).abs();
            if file_difference_i != 1 {
                continue;
            }

            if let Some(piece) = self.board[capture_candidate as usize] {
                if piece.side != pawn.side {
                    target_squares.push(capture_candidate as u8);
                }
            }
        }

        // Check for en passent 

        Ok(target_squares)
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
            en_passent: None,
        }
    }


    #[test]
    fn w_pawn_c2() {
        let mut pos = empty_position();

        pos.board[10] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.pawn_targets(10).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&18));
        assert!(moves.contains(&26));
        assert!(!moves.contains(&17));
        assert!(!moves.contains(&19));
    }

    #[test]
    fn b_pawn_c7() {
        let mut pos = empty_position();

        pos.board[50] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.pawn_targets(50).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&42));
        assert!(moves.contains(&34));
        assert!(!moves.contains(&41));
        assert!(!moves.contains(&43));
    }

    #[test]
    fn w_pawn_e4_enemy_f5_friendly_d5() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        pos.board[37] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.pawn_targets(28).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&36));
        assert!(moves.contains(&37));
        assert!(!moves.contains(&35));
    }

    #[test]
    fn w_pawn_d2_blocked_by_piece_d3() {
        let mut pos = empty_position();

        pos.board[11] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        pos.board[19] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.pawn_targets(11).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn w_pawn_d2_blocked_on_double_move() {
        let mut pos = empty_position();

        pos.board[11] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.pawn_targets(11).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&19));
        assert!(!moves.contains(&27));
    }

    #[test]
    fn w_pawn_a2_edge_capture_b3() {
        let mut pos = empty_position();

        pos.board[8] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        pos.board[17] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.pawn_targets(8).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&16));
        assert!(moves.contains(&24));
        assert!(moves.contains(&17));
        assert!(!moves.contains(&15));
    }

    #[test]
    fn b_pawn_h7_edge_capture_g6() {
        let mut pos = empty_position();

        pos.board[55] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        pos.board[46] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.pawn_targets(55).expect("pawn_targets returned Err");

        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&47));
        assert!(moves.contains(&39));
        assert!(moves.contains(&46));
        assert!(!moves.contains(&48));
    }

    #[test]
    fn wrong_piece_e2() {
        let mut pos = empty_position(); 

        pos.board[12] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::White,
        });

        assert_eq!(pos.pawn_targets(12), Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Pawn, found_piece: Piece::King, square: 12 }));
    }

    #[test]
    fn no_piece_e2() {
        let pos = empty_position(); 

        assert_eq!(pos.pawn_targets(12), Err(MoveGenError::NoPieceOnSquare { square: 12 }))
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(pos.pawn_targets(65), Err(MoveGenError::NotASquareOnBoard { square: 65 }))
    } 
}
