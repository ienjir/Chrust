use crate::{Piece, Square, file, moves::move_gen::MoveGenError, position::Position, rank};

impl Position {
    pub fn knight_targets(&self, from_square: Square) -> Result<Vec<Square>, MoveGenError> {
        let mut target_squares = Vec::with_capacity(8);

        if !(0..=63).contains(&from_square) {
            return Err(MoveGenError::NotASquareOnBoard { square: from_square })
        }

        let knight = match self.board[from_square as usize] {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: from_square }),
        };

        if knight.piece != Piece::Knight {
            return Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Knight, found_piece: knight.piece, square: from_square})
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

            let is_allowed_jump = (file_difference_i == 2 && rank_difference_i == 1) || (file_difference_i == 1 && rank_difference_i == 2);

            if !is_allowed_jump {
                continue;
            }

            let candidate_occupant = self.board[candidate_square_i as usize];
            match candidate_occupant {
                None => {
                    target_squares.push(candidate_square_i as u8);
                },
                Some(colored_piece) => {
                    if colored_piece.side != knight.side {
                        target_squares.push(candidate_square_i as u8);
                    }
                }
            };
        }

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
    fn knight_e4_empty_board() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.knight_targets(28).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 8);

        assert!(moves.contains(&43));
        assert!(moves.contains(&45));
        assert!(moves.contains(&11));
        assert!(moves.contains(&22));
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

        assert!(moves.contains(&50));
        assert!(moves.contains(&41));
        assert!(!moves.contains(&39));
        assert!(!moves.contains(&16));
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

        assert!(moves.contains(&47));  
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

        assert!(!moves.contains(&13));  
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position(); 

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::White,
        });

        assert_eq!(pos.knight_targets(60), Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Knight, found_piece: Piece::King, square: 60 }));
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position(); 

        assert_eq!(pos.knight_targets(35), Err(MoveGenError::NoPieceOnSquare { square: 35 }))
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(pos.pawn_targets(65), Err(MoveGenError::NotASquareOnBoard {square: 65}))
    } 
}
