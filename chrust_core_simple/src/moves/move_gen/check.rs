use std::usize;

use crate::{Piece, Side, Square, errors::MoveGenError, file, moves::{make_move::Move, move_gen::pawn}, position::{self, Position}, rank};

impl Position {
    pub fn is_square_attacked(&self, from_square: Square) -> Result<Option<Vec<Square>>, MoveGenError>{
        let mut attacking_squares: Vec<Square> = Vec::new();

        if !(0..=63).contains(&from_square) {
            return Err(MoveGenError::NotASquareOnBoard { square: from_square });
        }

        let piece = match self.board[from_square as usize] {
            Some(p) => p,
            None => return Err(MoveGenError::NoPieceOnSquare { square: from_square }),
        };

        // Pawns
        let pawn_attack_offsets: Vec<i16> = match piece.side {
            Side::White => vec![7, 9],
            Side::Black => vec![-7, -9],
        };

        for offset in pawn_attack_offsets {
            let attack_square = from_square as i16 + offset;

            let Some(target) = self.board[attack_square as usize] else {
                continue;
            };

            if target.side == piece.side {
                continue;
            }

            if target.piece == Piece::Pawn {
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
                    if !( (direction.abs() == 8 && file_diff == 0 && rank_diff == 1)
                        || (direction.abs() == 1 && file_diff == 1 && rank_diff == 0) )
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
                        // TODO: replace this with `by_side` logic, not `piece.side`
                        if occupant.side == piece.side {
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
        }
    }

    fn has_square(squares: &[Square], square: Square) -> bool {
        squares.iter().any(|&s| s == square)
    }

    #[test]
    fn is_square_attacked_no_piece_on_square() {
        let pos = empty_position();

        assert_eq!(
            pos.is_square_attacked(35),
            Err(MoveGenError::NoPieceOnSquare { square: 35 })
        );
    }

    #[test]
    fn is_square_attacked_out_of_bounds() {
        let pos = empty_position();

        assert_eq!(
            pos.is_square_attacked(65),
            Err(MoveGenError::NotASquareOnBoard { square: 65 })
        );
    }

    #[test]
    fn is_square_attacked_empty_board_none() {
        let mut pos = empty_position();

        pos.board[28] = Some(ColoredPiece {
            piece: Piece::King,
            side: Side::White,
        });

        assert_eq!(pos.is_square_attacked(28), Ok(None));
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

        let mut attacks = pos.is_square_attacked(28).expect("is_square_attacked returned Err").unwrap();
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

        let mut attacks = pos.is_square_attacked(36).expect("is_square_attacked returned Err").unwrap();
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

        let attacks = pos.is_square_attacked(28).expect("is_square_attacked returned Err").unwrap();

        assert!(has_square(&attacks, 60));
        assert!(has_square(&attacks, 1));
        assert_eq!(attacks.len(), 2);
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

        assert_eq!(pos.is_square_attacked(28), Ok(None));
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

        assert_eq!(pos.is_square_attacked(28), Ok(None));
    }
}
