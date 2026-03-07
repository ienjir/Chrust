use std::i16;

use crate::{
    Piece, Side, Square, errors::ChessError, helper::{file, file_rank, rank}, moves::make_move::{Move, MoveKind}, position::Position
};

impl Position {
    // Without promotion
    pub fn pawn_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
	let mut target_moves: Vec<Move> = Vec::with_capacity(4);

	let pawn = match self.get_validated_colored_piece(from_square, Piece::Pawn) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	let (forward, start_rank, capture_offsets, last_rank): (i16, i16, [i16; 2], i16) = match pawn.side {
	    Side::White => (8, 1, [7, 9], 7),
	    Side::Black => (-8, 6, [-7, -9], 0),
	};

	let (from_file_i, from_rank_i) = file_rank(from_square);
	let from_file_i = from_file_i as i16;
	let from_rank_i = from_rank_i as i16;

	let mut forward_1_is_empty = false;
	let forward_1_candidate_i = from_square as i16 + forward;

	if (0..=63).contains(&forward_1_candidate_i) {
	    let file_difference_i = (file(forward_1_candidate_i as u8) as i16 - from_file_i).abs();

	    if file_difference_i == 0 {
		if self.board[forward_1_candidate_i as usize].is_none() {
		    // Promotion check
		    let to_rank_i = rank(forward_1_candidate_i as u8) as i16;
		    if to_rank_i == last_rank {
			target_moves.push(Move {
			    colored_piece: pawn,
			    from_square: from_square,
			    to_square: forward_1_candidate_i as u8,
			    move_kind: MoveKind::Promotion {
				promotion_piece: Some(Piece::Pawn),
			    },
			});
		    } else {
			let single_move = Move {
			    colored_piece: pawn,
			    from_square: from_square,
			    to_square: forward_1_candidate_i as u8,
			    move_kind: MoveKind::Quiet,
			};

			target_moves.push(single_move);
			forward_1_is_empty = true;
		    }
		}
	    }
	}

	if from_rank_i == start_rank && forward_1_is_empty {
	    let forward_2_candidate_i = from_square as i16 + (forward * 2);
	    if (0..=63).contains(&forward_2_candidate_i) {
		let file_difference_i =
		    (file(forward_2_candidate_i as u8) as i16 - from_file_i).abs();
		if file_difference_i == 0 {
		    if self.board[forward_2_candidate_i as usize].is_none() {
			let double_move = Move {
			    colored_piece: pawn,
			    from_square: from_square,
			    to_square: forward_2_candidate_i as u8,
			    move_kind: MoveKind::DoublePawnPush {
				passed_square: forward_1_candidate_i as u8,
			    },
			};
			target_moves.push(double_move);
		    }
		}
	    }
	}

	for capture_offset in capture_offsets {
	    let capture_candidate = from_square as i16 + capture_offset;

	    if !(0..=63).contains(&capture_candidate) {
		continue;
	    }

	    let file_difference_i = (file(capture_candidate as u8) as i16 - from_file_i).abs();
	    if file_difference_i != 1 {
		continue;
	    }

	    if let Some(en_passant_square) = self.en_passant {
		if en_passant_square as i16 == capture_candidate {
		    let captured_square = match pawn.side {
			Side::White => (en_passant_square as i16 - 8) as u8,
			Side::Black => (en_passant_square as i16 + 8) as u8,
		    };

		    let en_passant_move = Move {
			colored_piece: pawn,
			from_square: from_square,
			to_square: capture_candidate as u8,
			move_kind: MoveKind::EnPassant {
			    capture_square: captured_square,
			},
		    };
		    target_moves.push(en_passant_move);
		}
	    }

	    if let Some(piece) = self.board[capture_candidate as usize] {
		if piece.side != pawn.side {
		    let en_passant_move = Move {
			colored_piece: pawn,
			from_square: from_square,
			to_square: capture_candidate as u8,
			move_kind: MoveKind::Capture,
		    };
		    target_moves.push(en_passant_move);
		}
	    }
	}

	Ok(target_moves)
    }
}

