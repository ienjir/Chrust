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

