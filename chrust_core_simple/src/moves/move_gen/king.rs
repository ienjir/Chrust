use crate::{Piece, Square, file, moves::move_gen::MoveGenError, position::Position, rank};

impl Position {
    pub fn king_targets(&self, inital_square: Square) -> Result<Vec<Square>, MoveGenError> {
        let mut target_squares = Vec::with_capacity(8);

        let directions: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
        let king = match self.board[inital_square as usize] {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: inital_square }),
        };

        if king.piece != Piece::King {
            return Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::King, found_piece: king.piece, square: inital_square})
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

            if !(file_difference <= 1 && rank_difference <= 1) {
                continue;
            }

            let square_on_board = self.board[candidate_square_u as usize];
            match square_on_board {
                None => {
                    target_squares.push(candidate_square_u);
                    continue;
                },
                Some(colored_piece) => {
                    if colored_piece.side != king.side {
                        target_squares.push(candidate_square_u);
                    }
                    continue;
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
    fn king_c5_empty_board() {
        let mut pos = empty_position();

        pos.board[34] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::White,
        });

        let moves = pos.king_targets(34).expect("king_targets returned Err");

        assert_eq!(moves.len(), 8);

        assert!(moves.contains(&42));
        assert!(moves.contains(&27));
        assert!(moves.contains(&25));
        assert!(moves.contains(&35));
    }

    #[test]
    fn king_h1_corner_test() {
        let mut pos = empty_position();

        pos.board[7] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::Black,
        });

        let moves = pos.king_targets(7).expect("king_targets returned Err");

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&15));
        assert!(moves.contains(&14));
        assert!(moves.contains(&6));
        assert!(!moves.contains(&8));
        assert!(!moves.contains(&16));
    }

    #[test]
    fn king_d5_enemy_e6() {
        let mut pos = empty_position();

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::White,
        });

        pos.board[44] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.king_targets(35).expect("king_targets returned Err");

        assert_eq!(moves.len(), 8);

        assert!(moves.contains(&44));  
    }

    #[test]
    fn king_h5_friendly_g4() {
        let mut pos = empty_position();

        pos.board[39] = Some(ColoredPiece {
            piece: crate::Piece::King,
            side: crate::Side::Black,
        });

        pos.board[30] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.king_targets(39).expect("king_targets returned Err");

        assert_eq!(moves.len(), 4);

        assert!(!moves.contains(&30));  
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position(); 

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(pos.king_targets(60), Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::King, found_piece: Piece::Knight, square: 60 }));
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position(); 

        assert_eq!(pos.king_targets(35), Err(MoveGenError::NoPieceOnSquare { square: 35 }))
    }
}
