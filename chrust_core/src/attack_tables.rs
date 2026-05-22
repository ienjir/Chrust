use crate::{
	Square,
	helper::{file_diff, in_bounds, rank_diff},
	moves::move_gen::king::get_file_and_rank_difference,
};
use std::{array, sync::OnceLock};

pub(crate) static KNIGHT_TARGETS: OnceLock<[Vec<Square>; 64]> = OnceLock::new();
pub(crate) static KING_TARGETS: OnceLock<[Vec<Square>; 64]> = OnceLock::new();

pub fn init_attack_tables() {
	KNIGHT_TARGETS.get_or_init(precompute_knight_targets);
	KING_TARGETS.get_or_init(precompute_king_targets);
}

pub(crate) fn precompute_knight_targets() -> [Vec<Square>; 64] {
	let mut moves: [Vec<Square>; 64] = array::from_fn(|_| Vec::with_capacity(8));
	let directions: [i16; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

	for compute_square in 0u8..64 {
		for direction in directions {
			let candidate_square_i = compute_square as i16 + direction;

			if !in_bounds(candidate_square_i) {
				continue;
			}

			if !matches!((file_diff(candidate_square_i, compute_square), rank_diff(candidate_square_i, compute_square)), (2, 1) | (1, 2)) {
				continue;
			}

			moves[compute_square as usize].push(candidate_square_i as Square);
		}
	}

	moves
}

pub(crate) fn precompute_king_targets() -> [Vec<Square>; 64] {
	let mut moves: [Vec<Square>; 64] = array::from_fn(|_| Vec::with_capacity(8));
	let directions: [i16; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

	for compute_square in 0u8..64 {
		for direction in directions {
			let candidate_square_i = compute_square as i16 + direction;

			if !in_bounds(candidate_square_i) {
				continue;
			}

			let (file_difference_i, rank_difference_i) = get_file_and_rank_difference(compute_square, candidate_square_i as u8);

			if !(file_difference_i <= 1 && rank_difference_i <= 1) {
				continue;
			}
			moves[compute_square as usize].push(candidate_square_i as Square);
		}
	}

	moves
}
