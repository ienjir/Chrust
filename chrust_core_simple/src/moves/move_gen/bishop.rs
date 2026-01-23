use crate::{Piece, Square, file, moves::move_gen::MoveGenError, position::Position, rank};

impl Position {
    pub fn bishop_targets(&self, from_square: Square) -> Result<Vec<Square>, MoveGenError>  {
        let mut target_squares = Vec::with_capacity(13);

        if !(0..=63).contains(&from_square) {
            return Err(MoveGenError::NotASquareOnBoard { square: from_square })
        }

        let bishop = match self.board[from_square as usize] {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: from_square }),
        };

        if bishop.piece != Piece::Bishop {
            return Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Bishop, found_piece: bishop.piece, square: from_square})
        }

        let directions: [i16; 4] = [-7, 7, -9, 9];

        for direction in directions {
            let mut step_from_i: i16 = from_square as i16;
            loop {
                let step_to_i = step_from_i + direction;

                if !(0..=63).contains(&step_to_i) {
                    break;
                }

                let file_difference_i = (file(step_to_i as u8) as i16 - file(step_from_i as u8) as i16).abs();
                let rank_difference_i = (rank(step_to_i as u8) as i16 - rank(step_from_i as u8) as i16).abs();

                if rank_difference_i != 1 || file_difference_i != 1 {
                    break;
                }

                let square_on_board = self.board[step_to_i as usize];
                match square_on_board {
                    None => {
                        target_squares.push(step_to_i as u8);
                        step_from_i = step_to_i;
                    },
                    Some(colored_piece) => {
                        if colored_piece.side != bishop.side {
                            target_squares.push(step_to_i as u8);
                        }

                        break;
                    }
                }
            }
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
    fn bishop_g7_empty_boad() {
        let mut pos = empty_position();

        pos.board[54] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(54).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 9);

        assert!(moves.contains(&63));
        assert!(moves.contains(&61));
        assert!(moves.contains(&47));
        assert!(moves.contains(&27));
        assert!(moves.contains(&0));
    }

    #[test]
    fn bishop_h7_corner_test() {
        let mut pos = empty_position();

        pos.board[7] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(7).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 7);

        assert!(moves.contains(&14));
        assert!(moves.contains(&56));
        assert!(!moves.contains(&8));
        assert!(!moves.contains(&16));
        assert!(!moves.contains(&0));
    }

    #[test]
    fn bishop_c7_enemy_f4() {
        let mut pos = empty_position();

        pos.board[50] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.bishop_targets(50).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 7);

        assert!(moves.contains(&36)); 
        assert!(moves.contains(&29));
        assert!(!moves.contains(&22));  
    }

    #[test]
    fn bishop_b3_friendly_e6() {
        let mut pos = empty_position();

        pos.board[17] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        pos.board[44] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(17).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 6);

        assert!(moves.contains(&35)); 
        assert!(!moves.contains(&44));
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position(); 

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(pos.bishop_targets(60), Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Bishop, found_piece: Piece::Knight, square: 60 }));
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position(); 

        assert_eq!(pos.bishop_targets(35), Err(MoveGenError::NoPieceOnSquare { square: 35 }))
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(pos.pawn_targets(65), Err(MoveGenError::NotASquareOnBoard {square: 65}))
    } 
}
