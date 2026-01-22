use crate::{Piece, Square, file, moves::move_gen::MoveGenError, position::Position, rank};

impl Position {
    pub fn knight_targets(&self, inital_square: Square) -> Result<Vec<Square>, MoveGenError> {
        let mut target_squares = Vec::with_capacity(8);

        let directions: [i16; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
        let knight = match self.board[inital_square as usize] {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: inital_square }),
        };

        if knight.piece != Piece::Knight {
            return Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Knight, found_piece: knight.piece, square: inital_square})
        }

        let current_file = file(inital_square) as i8;
        let current_rank = rank(inital_square) as i8;

        for direction in directions {
            let candidate_square_i = inital_square as i16 + direction as i16;
            if !(0..=63).contains(&candidate_square_i) {
                continue;
            }

            let candidate_square_u = candidate_square_i as u8;

            let file_difference = (file(candidate_square_u) as i8 - current_file).abs();
            let rank_difference = (rank(candidate_square_u) as i8 - current_rank).abs();

            let is_allowed_jump = (file_difference == 2 && rank_difference == 1) || (file_difference == 1 && rank_difference == 2);

            if !is_allowed_jump {
                continue;
            }

            let square_on_board = self.board[candidate_square_u as usize];
            match square_on_board {
                None => {
                    target_squares.push(candidate_square_u);
                    continue;
                },
                Some(colored_piece) => {
                    if colored_piece.side != knight.side {
                        target_squares.push(candidate_square_u);
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
}
