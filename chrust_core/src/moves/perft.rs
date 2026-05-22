use crate::position::Position;

impl Position {
	/// Count leaf nodes at the given depth (standard perft).
	/// Uses make/undo on the raw position — no game-level rules (50-move,
	/// repetition, etc.) which is correct for perft.
	pub fn perft(&mut self, depth: u32) -> u64 {
		if depth == 0 {
			return 1;
		}

		let moves = match self.get_all_legal_moves_for_side(self.side_to_move) {
			Ok(m) => m,
			Err(_) => return 0,
		};

		if depth == 1 {
			return moves.len() as u64;
		}

		let mut count = 0;
		for mv in moves {
			let undo = self.make_move_unvalidated(mv).unwrap();
			count += self.perft(depth - 1);
			self.undo_move(undo, mv).unwrap();
		}

		count
	}

	/// Perft with per-move breakdown printed to stdout (useful for debugging).
	pub fn perft_divide(&mut self, depth: u32) -> u64 {
		if depth == 0 {
			return 1;
		}

		let moves = match self.get_all_legal_moves_for_side(self.side_to_move) {
			Ok(m) => m,
			Err(_) => return 0,
		};

		let mut total = 0;
		for mv in moves {
			let undo = self.make_move_unvalidated(mv).unwrap();
			let nodes = self.perft(depth - 1);
			self.undo_move(undo, mv).unwrap();

			let from = mv.from_square;
			let to = mv.to_square;
			println!("{}{}: {}", crate::converter::convert_square_to_string(from), crate::converter::convert_square_to_string(to), nodes);
			total += nodes;
		}

		println!("\nTotal: {total}");
		total
	}
}

#[cfg(test)]
mod tests;
