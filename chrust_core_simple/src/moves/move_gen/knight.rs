use crate::{Square, file, rank};

const KNIGHT_OFFSET: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
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
}
