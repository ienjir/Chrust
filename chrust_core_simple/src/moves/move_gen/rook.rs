use crate::{Square, file, position::Position, rank};

impl Position {
    pub fn rook_targets(&self, inital_square: Square) -> Vec<Square> {
        let mut target_squares = Vec::with_capacity(14);

        let directions: [i16; 4] = [-8, 8, -1, 1];
        let rook = match self.board[inital_square as usize] {
            Some(p) => p,
            None => return Vec::new(),
        };

        for direction_increment in directions {
            let mut current_square: i16 = inital_square as i16;
            loop {
                let next = current_square + direction_increment;

                if !(0..=63).contains(&next) {
                    break;
                }

                let file_difference = (file(next as u8) as i8 - file(current_square as u8) as i8).abs(); 
                let rank_difference = (rank(next as u8) as i8 - rank(current_square as u8) as i8).abs();

                if direction_increment.abs() == 8 { 
                    if file_difference != 0 || rank_difference != 1 {
                        break;
                    }
                } else { 
                    if rank_difference != 0 || file_difference != 1 {
                        break;
                    }

                }

                let square_on_board = self.board[next as usize];
                match square_on_board {
                    None => {
                        target_squares.push(next as u8);
                        current_square = next;
                        continue;
                    },
                    Some(colored_piece) => {
                        if colored_piece.side != rook.side {
                            target_squares.push(next as u8);
                        }
                        break;
                    }
                }
            }
        }

        target_squares
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
    fn rook_h8_empty_board() {
        let mut pos = empty_position();

        pos.board[63] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(63);

        assert_eq!(moves.len(), 14);

        assert!(moves.contains(&62)); 
        assert!(moves.contains(&56)); 
        assert!(moves.contains(&55)); 
        assert!(moves.contains(&7)); 
    }

    #[test]
    fn rook_d4_empty_board() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27);

        assert_eq!(moves.len(), 14);

        assert!(moves.contains(&24)); 
        assert!(moves.contains(&31));
        assert!(moves.contains(&3));  
        assert!(moves.contains(&26));
    }

    #[test]
    fn rook_d4_blocked_by_friendly_piece_f4() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27);

        assert!(moves.contains(&28)); 
        assert!(!moves.contains(&29)); 
        assert!(!moves.contains(&30));
    }

    #[test]
    fn rook_d4_captures_enemy_f4_and_stops() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        // enemy piece on f4
        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.rook_targets(27);

        assert!(moves.contains(&28)); 
        assert!(moves.contains(&29)); 
        assert!(!moves.contains(&30)); 
    }
}
