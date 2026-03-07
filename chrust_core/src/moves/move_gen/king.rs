use std::usize;

use crate::{
    Piece, Side, Square, errors::ChessError, helper::{file, file_rank, rank}, moves::make_move::{Move, MoveKind}, position::Position
};

impl Position {
    pub fn king_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
	let mut target_moves: Vec<Move> = Vec::with_capacity(8);

	let king = match self.get_validated_colored_piece(from_square, Piece::King) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	let (from_file_i, from_rank_i) = file_rank(from_square);
	let from_file_i = from_file_i as i16;
	let from_rank_i = from_rank_i as i16;

	let directions: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

	// Check casteling
	let (king_from, k_empty, k_allowed, q_empty, q_allowed) = match king.side {
	    Side::White => (4u8, [5u8, 6u8], self.castle[0], [3u8, 2u8, 1u8], self.castle[1]),
	    Side::Black => (60u8, [61u8, 62u8], self.castle[2], [59u8, 58u8, 57u8], self.castle[3]),
	};

	if from_square == king_from {
	    let king_in_check = self.is_square_attacked(king_from, match king.side {Side::Black => Side::White, Side::White => Side::Black})?.is_some();

	    let k_clear = k_empty.iter().all(|&sq| self.board[sq as usize].is_none());
	    let k_rook_ok = matches!(
		self.board[(king_from + 3) as usize],
		Some(rook) if rook.piece == Piece::Rook && rook.side == king.side
	    );
	    let k_safe = !self.is_square_attacked(king_from + 1, king.side)?.is_some()
		&& !self.is_square_attacked(king_from + 2, king.side)?.is_some();

	    if k_allowed && k_clear && k_rook_ok && !king_in_check && k_safe {
		target_moves.push(Move {
		    colored_piece: king,
		    from_square,
		    to_square: king_from + 2,
		    move_kind: MoveKind::Castling {
			rook_from: king_from + 3,
			rook_to: king_from + 1,
		    },
		});
	    }

	    let q_clear = q_empty.iter().all(|&sq| self.board[sq as usize].is_none());
	    let q_rook_ok = matches!(
		self.board[(king_from - 4) as usize],
		Some(rook) if rook.piece == Piece::Rook && rook.side == king.side
	    );
	    let q_safe = !self.is_square_attacked(king_from - 1, king.side)?.is_some()
		&& !self.is_square_attacked(king_from - 2, king.side)?.is_some();

	    if q_allowed && q_clear && q_rook_ok && !king_in_check && q_safe {
		target_moves.push(Move {
		    colored_piece: king,
		    from_square,
		    to_square: king_from - 2,
		    move_kind: MoveKind::Castling {
			rook_from: king_from - 4,
			rook_to: king_from - 1,
		    },
		});
	    }
	}

	for direction in directions {
	    let candidate_square_i = from_square as i16 + direction;

	    if !(0..=63).contains(&candidate_square_i) {
		continue;
	    }

	    let file_difference_i = (file(candidate_square_i as u8) as i16 - from_file_i).abs();
	    let rank_difference_i = (rank(candidate_square_i as u8) as i16 - from_rank_i).abs();

	    if !(file_difference_i <= 1 && rank_difference_i <= 1) {
		continue;
	    }

	    let candidate_occupant = self.board[candidate_square_i as usize];
	    match candidate_occupant {
		None => {
		    let check_check = self.is_square_attacked(candidate_square_i as u8, king.side);
		    match check_check {
			Err(x) => return Err(x),
			Ok(x) => {
			    if x.is_some() {
				continue;
			    }
			}
		    }

		    target_moves.push(Move {
			colored_piece: king,
			from_square: from_square,
			to_square: candidate_square_i as u8,
			move_kind: MoveKind::Quiet,
		    });
		    continue;
		}
		Some(colored_piece) => {
		    let check_check = self.is_square_attacked(candidate_square_i as u8, king.side);
		    match check_check {
			Err(x) => return Err(x),
			Ok(x) => {
			    if x.is_some() {
				continue;
			    }
			}
		    }

		    if colored_piece.side != king.side {
			target_moves.push(Move {
			    colored_piece: king,
			    from_square: from_square,
			    to_square: candidate_square_i as u8,
			    move_kind: MoveKind::Capture,
			});
		    }
		    continue;
		}
	    };
	}

	Ok(target_moves)
    }
}

