use crate::{
    errors::ChessError,
    helper::{file_rank, is_square_on_board},
    position::Position,
    Piece, Side, Square,
};

impl Position {
    pub fn is_king_in_check(&self, side: Side) -> Result<Option<Vec<Square>>, ChessError> {
        let (king_square, attack_side) = match side {
            Side::White => (self.king_squares[0], Side::Black),
            Side::Black => (self.king_squares[1], Side::White),
        };

        let attacking_squares = self.is_square_attacked(king_square, attack_side);

        attacking_squares
    }

    pub fn is_square_attacked(
        &self,
        from_square: Square,
        attacking_side: Side,
    ) -> Result<Option<Vec<Square>>, ChessError> {
        let mut attacking_squares: Vec<Square> = Vec::new();

        is_square_on_board(from_square)?;

        // Pawns
        let pawn_attack_offsets: Vec<i16> = match attacking_side {
            Side::White => vec![-7, -9],
            Side::Black => vec![7, 9],
        };

        for offset in pawn_attack_offsets {
            let attack_square = from_square as i16 + offset;

            if !(0..=63).contains(&attack_square) {
                continue;
            }

            let Some(target) = self.board[attack_square as usize] else {
                continue;
            };

            if target.side != attacking_side {
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

            if target.side != attacking_side {
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

            if target.side != attacking_side {
                continue;
            }

            if target.piece == Piece::Knight {
                attacking_squares.push(attack_square as u8);
            }
        }

        // Sliding
        self.is_square_attacked_sliding(&mut attacking_squares, from_square, attacking_side);

        if attacking_squares.is_empty() {
            return Ok(None);
        }

        Ok(Some(attacking_squares))
    }

    pub fn is_square_attacked_sliding(
        &self,
        attacking_squares: &mut Vec<Square>,
        from_square: Square,
        attacking_side: Side,
    ) {
        let directions: [i16; 8] = [-8, 8, -1, 1, -7, 7, -9, 9];

        for direction in directions {
            self.slide_ray(from_square, direction, |to_square, occupant| {
                match occupant {
                    None => true, // continue
                    Some(occ) => {
                        if occ.side == attacking_side {
                            let is_rook_ray = direction.abs() == 8 || direction.abs() == 1;
                            let attacks = if is_rook_ray {
                                occ.piece == Piece::Rook || occ.piece == Piece::Queen
                            } else {
                                occ.piece == Piece::Bishop || occ.piece == Piece::Queen
                            };
                            if attacks {
                                attacking_squares.push(to_square);
                            }
                        }
                        false // stop
                    }
                }
            });
        }
    }
}
