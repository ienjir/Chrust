use std::i16;

use crate::{Piece, Side, Square, errors::MoveGenError, file, moves::make_move::{Move, MoveKind}, position::{Position}, rank};

impl Position {
    // Without promotion
    pub fn pawn_targets(&self, from_square: Square)  -> Result<Vec<Move>, MoveGenError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(4);

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

        let (forward, start_rank, capture_offsets, last_rank): (i16, i16, [i16; 2], i16) = match pawn.side {
            Side::White => (8, 1, [7, 9], 7),
            Side::Black => (-8, 6, [-7, -9], 0),
        };

        let from_file_i = file(from_square) as i16;
        let from_rank_i = rank(from_square) as i16;

        let mut forward_1_is_empty = false;
        let forward_1_candidate_i = from_square as i16 + forward; 

        if (0..=63).contains(&forward_1_candidate_i) {
            let file_difference_i = (file(forward_1_candidate_i as u8) as i16 - from_file_i).abs();

            if file_difference_i == 0 {
                if self.board[forward_1_candidate_i as usize].is_none() {
                    if from_rank_i == last_rank {
                        target_moves.push(Move {
                            from_square: from_square,
                            to_square: forward_1_candidate_i as u8,
                            move_kind: MoveKind::Promotion { promotion_piece: Piece::Pawn },
                        });    
                    } else {
                        let single_move = Move {
                            from_square: from_square,
                            to_square: forward_1_candidate_i as u8,
                            move_kind: MoveKind::Quiet,
                        };

                        target_moves.push(single_move);
                        forward_1_is_empty = true;
                    }
                }
            }
        }

        if from_rank_i == start_rank && forward_1_is_empty {
            let forward_2_candidate_i = from_square as i16 + (forward * 2);
            if (0..=63).contains(&forward_2_candidate_i) {
                let file_difference_i = (file(forward_2_candidate_i as u8) as i16 - from_file_i).abs();
                if file_difference_i == 0 {
                    if self.board[forward_2_candidate_i as usize].is_none() {
                        let double_move = Move {
                            from_square: from_square,
                            to_square: forward_2_candidate_i as u8,
                            move_kind: MoveKind::DoublePawnPush { passed_square: forward_1_candidate_i as u8 },
                        };
                        target_moves.push(double_move);
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

            if let Some(en_passant_square) = self.en_passant {
                if en_passant_square as i16 == capture_candidate {
                    let captured_square = match pawn.side {
                        Side::White => (en_passant_square as i16 - 8) as u8,
                        Side::Black => (en_passant_square as i16 + 8) as u8,
                    };

                    let en_passant_move = Move {
                        from_square: from_square,
                        to_square: capture_candidate as u8,
                        move_kind: MoveKind::EnPassant { capture_square: captured_square }
                    };
                    target_moves.push(en_passant_move);
                }
            }

            if let Some(piece) = self.board[capture_candidate as usize] {
                if piece.side != pawn.side {
                    let en_passant_move = Move {
                        from_square: from_square,
                        to_square: capture_candidate as u8,
                        move_kind: MoveKind::Capture,
                    };
                    target_moves.push(en_passant_move);
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
        }
    }

    fn has_move(moves: &[Move], from: Square, to: Square, kind: MoveKind) -> bool {
        moves.iter().any(|m| {
            m.from_square == from && m.to_square == to && m.move_kind == kind
        })
    }

    fn has_to_square(moves: &[Move], to: Square) -> bool {
        moves.iter().any(|m| m.to_square == to)
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

        assert!(has_move(&moves, 10, 18, MoveKind::Quiet));
        assert!(has_move(&moves, 10, 26, MoveKind::DoublePawnPush { passed_square: 18 }));
        assert!(!has_to_square(&moves, 17));
        assert!(!has_to_square(&moves, 19));
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

        assert!(has_move(&moves, 50, 42, MoveKind::Quiet));
        assert!(has_move(&moves, 50, 34, MoveKind::DoublePawnPush { passed_square: 42 }));
        assert!(!has_to_square(&moves, 41));
        assert!(!has_to_square(&moves, 43));
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

        assert!(has_move(&moves, 28, 36, MoveKind::Quiet));
        assert!(has_move(&moves, 28, 37, MoveKind::Capture));
        assert!(!has_to_square(&moves, 35));
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
        assert!(has_move(&moves, 11, 19, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 27));
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
        assert!(has_move(&moves, 8, 16, MoveKind::Quiet));
        assert!(has_move(&moves, 8, 24, MoveKind::DoublePawnPush { passed_square: 16 }));
        assert!(has_move(&moves, 8, 17, MoveKind::Capture));
        assert!(!has_to_square(&moves, 15));
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
        assert!(has_move(&moves, 55, 47, MoveKind::Quiet));
        assert!(has_move(&moves, 55, 39, MoveKind::DoublePawnPush { passed_square: 47 }));
        assert!(has_move(&moves, 55, 46, MoveKind::Capture));
        assert!(!has_to_square(&moves, 48));
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

    #[test]
    fn w_pawn_e5_en_passant_d6() {
        let mut pos = empty_position();

        pos.board[36] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.en_passant = Some(43); 

        let moves = pos.pawn_targets(36).expect("pawn_targets returned Err");

        assert!(has_move(&moves, 36, 44, MoveKind::Quiet));
        assert!(has_move(&moves, 36, 43, MoveKind::EnPassant { capture_square: 35 }));
    }
}
