use crate::{ColoredPiece, Square, helper::{file, rank}, moves::make_move::{Move, MoveKind}, position::Position};

impl Position {
    pub fn diagonal_slider(&self, from_square: Square, colored_piece: ColoredPiece, target_moves: &mut Vec<Move>) {
	let directions: [i16; 4] = [-7, 7, -9, 9];

	for direction in directions {
	    let mut step_from_i: i16 = from_square as i16;
	    loop {
		let step_to_i = step_from_i + direction;

		if !(0..=63).contains(&step_to_i) {
		    break;
		}

                let file_difference_i = (file(step_to_i as u8) as i16 - file(step_from_i as u8) as i16).abs();
                let rank_difference_i = (rank(step_to_i as u8) as i16 - rank(step_from_i as u8) as i16).abs();

		if rank_difference_i != 1 || file_difference_i != 1 {
		    break;
		}

		let candidate_occupant = self.board[step_to_i as usize];
		match candidate_occupant {
		    None => {
			target_moves.push(Move {
			    colored_piece: colored_piece,
			    from_square: from_square,
			    to_square: step_to_i as u8,
			    move_kind: MoveKind::Quiet,
			});
			step_from_i = step_to_i;
		    }
		    Some(occupant) => {
			if colored_piece.side != occupant.side {
			    target_moves.push(Move {
				from_square: from_square,
				to_square: step_to_i as u8,
				move_kind: MoveKind::Capture,
				colored_piece: colored_piece,
			    });
			}
			break;
		    }
		}
	    }
	}
    }

    pub fn horizontal_vertical_slider(&self, from_square: Square, colored_piece: ColoredPiece, target_moves: &mut Vec<Move>) {
	let directions: [i16; 4] = [-8, 8, -1, 1];

	for direction in directions {
	    let mut step_from_i: i16 = from_square as i16;
	    loop {
		let step_to_i = step_from_i + direction;

		if !(0..=63).contains(&step_to_i) {
		    break;
		}

                let file_difference_i = (file(step_to_i as u8) as i16 - file(step_from_i as u8) as i16).abs();
                let rank_difference_i = (rank(step_to_i as u8) as i16 - rank(step_from_i as u8) as i16).abs();

		if direction.abs() == 8 {
		    if file_difference_i != 0 || rank_difference_i != 1 {
			break;
		    }
		} else {
		    if file_difference_i != 1 || rank_difference_i != 0 {
			break;
		    }
		}

		let candidate_occupant = self.board[step_to_i as usize];
		match candidate_occupant {
		    None => {
			target_moves.push(Move {
			    colored_piece: colored_piece,
			    from_square: from_square,
			    to_square: step_to_i as u8,
			    move_kind: MoveKind::Quiet,
			});
			step_from_i = step_to_i;
		    }
		    Some(occupant) => {
			if colored_piece.side != occupant.side {
			    target_moves.push(Move {
				colored_piece: colored_piece,
				from_square: from_square,
				to_square: step_to_i as u8,
				move_kind: MoveKind::Capture,
			    });
			}
			break;
		    }
		}
	    }
	}
    }
}
