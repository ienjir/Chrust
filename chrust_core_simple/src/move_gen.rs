use crate::{Square, file, rank};

const KNIGHT_OFFSET: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
const KING_OFFSET: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

pub fn king_offset(current_square: Square) -> Vec<Square> {
    let mut target_squares = Vec::new();

    let current_file = file(current_square) as i8;
    let current_rank = rank(current_square) as i8;

    for offset in KING_OFFSET {
        let candidate_square_i = current_square as i16 + offset as i16;
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

pub fn knight_targets(current_square: Square) -> Vec<Square> {
    let mut target_squares = Vec::new();
    
    let current_file = file(current_square) as i8;
    let current_rank = rank(current_square) as i8;

    for offset in KNIGHT_OFFSET {
        let candidate_square_i = current_square as i16 + offset as i16;
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
