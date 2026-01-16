use crate::{Square, file, rank};

// For emtpy board
const KING_OFFSET: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
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

#[cfg(test)]
mod tests {
    use super::*;

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
