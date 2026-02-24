use crate::{Piece, Side, Square, errors::ChessError, helper::{file, file_rank, rank}, position::Position};

impl Position {
    pub fn is_square_attacked(&self, from_square: Square, side_to_attack: Side, ) -> Result<Option<Vec<Square>>, ChessError> {
        let mut attacking_squares: Vec<Square> = Vec::new();

        if !(0..=63).contains(&from_square) {
            return Err(ChessError::NotASquareOnBoard {
                square: from_square,
            });
        }

        // Pawns
        let pawn_attack_offsets: Vec<i16> = match side_to_attack {
            Side::White => vec![7, 9],
            Side::Black => vec![-7, -9],
        };

        for offset in pawn_attack_offsets {
            let attack_square = from_square as i16 + offset;

            if !(0..=63).contains(&attack_square) {
                continue;
            }

            let Some(target) = self.board[attack_square as usize] else {
                continue;
            };

            if target.side == side_to_attack {
                continue;
            }

            if target.piece == Piece::Pawn {
                attacking_squares.push(attack_square as u8);
            }
        }

        // King
        let king_attack_offsets: Vec<i16> = Vec::from([1, 7, 8, 9, -1, -7, -8, -9]);

        for offset in king_attack_offsets {
            let attack_square = from_square as i16 + offset;

            if !(0..=63).contains(&attack_square) {
                continue;
            }

            let (from_file, from_rank) = file_rank(from_square);
            let (from_file, from_rank) = (from_file as i16, from_rank as i16);
            let (attack_file, attack_rank) = file_rank(attack_square as u8);
            let (attack_file, attack_rank) = (attack_file as i16, attack_rank as i16);
            let file_diff = (attack_file - from_file).abs();
            let rank_diff = (attack_rank - from_rank).abs();

            if file_diff > 1 || rank_diff > 1 {
                continue;
            }

            let Some(target) = self.board[attack_square as usize] else {
                continue;
            };

            if target.side == side_to_attack {
                continue;
            }

            if target.piece == Piece::King {
                attacking_squares.push(attack_square as u8);
            }
        }

        // Knight
        let knight_attack_offsets: Vec<i16> = Vec::from([15, 17, 6, -10, -17, -15, -6, 10]);

        for offset in knight_attack_offsets {
            let attack_square = from_square as i16 + offset;

            if !(0..=63).contains(&attack_square) {
                continue;
            }

            let (from_file, from_rank) = file_rank(from_square);
            let (from_file, from_rank) = (from_file as i16, from_rank as i16);
            let (attack_file, attack_rank) = file_rank(attack_square as u8);
            let (attack_file, attack_rank) = (attack_file as i16, attack_rank as i16);
            let file_diff = (attack_file - from_file).abs();
            let rank_diff = (attack_rank - from_rank).abs();

            let is_knight_l =
                (file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1);
            if !is_knight_l {
                continue;
            }

            let Some(target) = self.board[attack_square as usize] else {
                continue;
            };

            if target.side == side_to_attack {
                continue;
            }

            if target.piece == Piece::Knight {
                attacking_squares.push(attack_square as u8);
            }
        }

        // Sliding
        let directions: [i16; 8] = [-8, 8, -1, 1, -7, 7, -9, 9];

        for direction in directions {
            let mut step_from_i: i16 = from_square as i16;

            loop {
                let step_to_i = step_from_i + direction;

                if !(0..=63).contains(&step_to_i) {
                    break;
                }

                let from_step = step_from_i as u8;
                let to_step = step_to_i as u8;

                let file_diff = (file(to_step) as i16 - file(from_step) as i16).abs();
                let rank_diff = (rank(to_step) as i16 - rank(from_step) as i16).abs();

                let is_rook_ray = direction.abs() == 8 || direction.abs() == 1;
                let is_bishop_ray = direction.abs() == 7 || direction.abs() == 9;

                if is_rook_ray {
                    if !((direction.abs() == 8 && file_diff == 0 && rank_diff == 1)
                        || (direction.abs() == 1 && file_diff == 1 && rank_diff == 0))
                    {
                        break;
                    }
                } else if is_bishop_ray {
                    if !(file_diff == 1 && rank_diff == 1) {
                        break;
                    }
                }

                match self.board[to_step as usize] {
                    None => {
                        step_from_i = step_to_i;
                    }
                    Some(occupant) => {
                        if occupant.side == side_to_attack {
                            break;
                        }

                        let attacks = if is_rook_ray {
                            occupant.piece == Piece::Rook || occupant.piece == Piece::Queen
                        } else {
                            occupant.piece == Piece::Bishop || occupant.piece == Piece::Queen
                        };

                        if attacks {
                            attacking_squares.push(to_step);
                        }
                        break;
                    }
                }
            }
        }

        if attacking_squares.is_empty() {
            return Ok(None);
        }

        Ok(Some(attacking_squares))
    }

    pub fn is_king_in_check(&self, side: Side) -> Result<Option<Vec<Square>>, ChessError>{
        let (king_square, attack_side) = match side {
            Side::White => { (self.king_squares[0], Side::Black) },
            Side::Black => { (self.king_squares[1], Side::White) },
        };

        let attacking_squares = self.is_square_attacked(king_square, attack_side);

        attacking_squares
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColoredPiece, Piece, Side};

    use super::*;

    fn empty_position() -> Position {
        Position {
            board: [None; 64],
            side_to_move: Side::White,
            castle: [false; 4],
            en_passant: None,
            king_squares: [4, 60],
        }
    }

    fn has_square(squares: &[Square], square: Square) -> bool {
        squares.iter().any(|&s| s == square)
    }

    #[test]
    fn is_square_attacked_empty_square_none() {
        let pos = empty_position();

        assert_eq!(pos.is_square_attacked(35, Side::White), Ok(None));
    }

    #[test]
    fn is_square_attacked_out_of_bounds() {
        let pos = empty_position();

        assert_eq!(
            pos.is_square_attacked(65, Side::White),
            Err(ChessError::NotASquareOnBoard { square: 65 })
        );
    }

    #[test]
    fn is_square_attacked_empty_board_none() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });

        assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
    }

    #[test]
    fn is_square_attacked_by_black_pawns() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[35] = Some(ColoredPiece {
            piece: Piece::Pawn,
            side: Side::Black,
        });
        pos.board[37] = Some(ColoredPiece {
            piece: Piece::Pawn,
            side: Side::Black,
        });

        let mut attacks = pos
            .is_square_attacked(28, Side::White)
            .expect("is_square_attacked returned Err")
            .unwrap();
        attacks.sort_unstable();

        assert_eq!(attacks, vec![35, 37]);
    }

    #[test]
    fn is_square_attacked_by_white_pawns() {
        let mut pos = empty_position();

        pos.board[36] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::Black,
        });
        pos.board[27] = Some(ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        });
        pos.board[29] = Some(ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        });

        let mut attacks = pos
            .is_square_attacked(36, Side::Black)
            .expect("is_square_attacked returned Err")
            .unwrap();
        attacks.sort_unstable();

        assert_eq!(attacks, vec![27, 29]);
    }

    #[test]
    fn is_square_attacked_by_rook_and_bishop() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[60] = Some(ColoredPiece {
            piece: Piece::Rook,
            side: Side::Black,
        });
        pos.board[1] = Some(ColoredPiece {
            piece: Piece::Bishop,
            side: Side::Black,
        });

        let attacks = pos
            .is_square_attacked(28, Side::White)
            .expect("is_square_attacked returned Err")
            .unwrap();

        assert!(has_square(&attacks, 60));
        assert!(has_square(&attacks, 1));
        assert_eq!(attacks.len(), 2);
    }

    #[test]
    fn is_square_attacked_by_king_adjacent() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[29] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::Black,
        });

        let attacks = pos
            .is_square_attacked(28, Side::White)
            .expect("is_square_attacked returned Err")
            .unwrap();

        assert!(has_square(&attacks, 29));
    }

    #[test]
    fn is_square_attacked_king_does_not_wrap_board_edge() {
        let mut pos = empty_position();

        pos.board[7] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[8] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::Black,
        });

        assert_eq!(pos.is_square_attacked(7, Side::White), Ok(None));
    }

    #[test]
    fn is_square_attacked_by_knight_l_shape() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[45] = Some(ColoredPiece {
            piece: Piece::Knight,
            side: Side::Black,
        });

        let attacks = pos
            .is_square_attacked(28, Side::White)
            .expect("is_square_attacked returned Err")
            .unwrap();

        assert!(has_square(&attacks, 45));
    }

    #[test]
    fn is_square_attacked_knight_does_not_wrap_board_edge() {
        let mut pos = empty_position();

        pos.board[7] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[17] = Some(ColoredPiece {
            piece: Piece::Knight,
            side: Side::Black,
        });

        assert_eq!(pos.is_square_attacked(7, Side::White), Ok(None));
    }

    #[test]
    fn is_square_attacked_blocked_by_friendly_piece() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[60] = Some(ColoredPiece {
            piece: Piece::Rook,
            side: Side::Black,
        });
        pos.board[44] = Some(ColoredPiece {
            piece: Piece::Bishop,
            side: Side::White,
        });

        assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
    }

    #[test]
    fn is_square_attacked_blocked_by_enemy_non_attacker() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });
        pos.board[60] = Some(ColoredPiece {
            piece: Piece::Rook,
            side: Side::Black,
        });
        pos.board[44] = Some(ColoredPiece {
            piece: Piece::Knight,
            side: Side::Black,
        });

        assert_eq!(pos.is_square_attacked(28, Side::White), Ok(None));
    }
}
