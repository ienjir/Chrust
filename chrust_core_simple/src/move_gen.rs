use crate::{Position, Square, file, rank};

const KNIGHT_OFFSET: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
const KING_OFFSET: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
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

// For emtpy board
pub fn king_targets(inital_square: Square) -> Vec<Square> {
    let mut target_squares = Vec::new();

    let current_file = file(inital_square) as i8;
    let current_rank = rank(inital_square) as i8;

    for offset in KING_OFFSET {
        let candidate_square_i = inital_square as i16 + offset as i16;
        if !(0..=63).contains(&candidate_square_i) {
            continue;
        }

        let candidate_square_u = candidate_square_i as u8;

        let file_difference = (file(candidate_square_u) as i8 - current_file).abs();
        let rank_difference = (rank(candidate_square_u) as i8 - current_rank).abs();

        if file_difference <= 1 && rank_difference <= 1 {
            target_squares.push(candidate_square_u);
        }
    }

    target_squares
}

// For emtpy board
pub fn knight_targets(inital_square: Square) -> Vec<Square> {
    let mut target_squares = Vec::new();

    let current_file = file(inital_square) as i8;
    let current_rank = rank(inital_square) as i8;

    for offset in KNIGHT_OFFSET {
        let candidate_square_i = inital_square as i16 + offset as i16;
        if !(0..=63).contains(&candidate_square_i) {
            continue;
        }

        let candidate_square_u = candidate_square_i as u8;

        let file_difference = (file(candidate_square_u) as i8 - current_file).abs();
        let rank_difference = (rank(candidate_square_u) as i8 - current_rank).abs();

        if (file_difference == 2 && rank_difference == 1) || (file_difference == 1 && rank_difference == 2) {
            target_squares.push(candidate_square_u);
        }
    }

    target_squares
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knight_c7() {
        let moves = knight_targets(50);

        assert_eq!(moves.len(), 6);

        let expected = [33, 35, 40, 44, 56, 60];
        for square in expected {
            assert!(moves.contains(&square));
        }
    }

    #[test]
    fn knight_h8() {
        let moves = knight_targets(7);

        assert_eq!(moves.len(), 2);

        let expected = [22, 13];
        for square in expected {
            assert!(moves.contains(&square));
        }
    }

    #[test]
    fn king_d4() {
        let moves = king_targets(27);

        assert_eq!(moves.len(), 8);

        let expected = [18, 19, 20, 26, 28, 34, 35, 36];
        for square in expected {
            assert!(moves.contains(&square));
        }
    }

    #[test]
    fn king_h8() {
        let moves = king_targets(63);

        assert_eq!(moves.len(), 3);

        let expected = [62, 54, 55];
        for square in expected {
            assert!(moves.contains(&square));
        }
    }
}
