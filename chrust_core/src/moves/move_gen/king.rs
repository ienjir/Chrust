use std::usize;

use crate::{
    Piece, Side, Square, errors::ChessError, file, moves::make_move::{Move, MoveKind}, position::Position, rank
};

impl Position {
    pub fn king_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
	let mut target_moves: Vec<Move> = Vec::with_capacity(8);

	let king = match self.get_validated_colored_piece(from_square, Piece::King) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	let from_file_i = file(from_square) as i16;
	let from_rank_i = rank(from_square) as i16;

	let directions: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

	// Check casteling
	let (king_from, k_empty, k_allowed, q_empty, q_allowed) = match king.side {
	    Side::White => (
		4u8,
		[5u8, 6u8],
		self.castle[0],
		[3u8, 2u8, 1u8],
		self.castle[1],
	    ),
	    Side::Black => (
		60u8,
		[61u8, 62u8],
		self.castle[2],
		[59u8, 58u8, 57u8],
		self.castle[3],
	    ),
	};

	if from_square == king_from {
	    let king_in_check = self.is_square_attacked(king_from, king.side)?.is_some();

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

#[cfg(test)]
mod tests {
    use crate::ColoredPiece;

    use super::*;

    fn empty_position() -> Position {
	Position {
	    board: [None; 64],
	    side_to_move: crate::Side::White,
	    castle: [false; 4],
	    en_passant: None,
	    king_squares: [4, 60],
	}
    }

    fn has_move(moves: &[Move], from: Square, to: Square, kind: MoveKind) -> bool {
	moves
	    .iter()
	    .any(|m| m.from_square == from && m.to_square == to && m.move_kind == kind)
    }

    fn has_to_square(moves: &[Move], to: Square) -> bool {
	moves.iter().any(|m| m.to_square == to)
    }

    #[test]
    fn king_c5_empty_board() {
	let mut pos = empty_position();

	pos.board[34] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::White,
	});

	let moves = pos.king_targets(34).expect("king_targets returned Err");

	assert_eq!(moves.len(), 8);

	assert!(has_move(&moves, 34, 42, MoveKind::Quiet));
	assert!(has_move(&moves, 34, 27, MoveKind::Quiet));
	assert!(has_move(&moves, 34, 25, MoveKind::Quiet));
	assert!(has_move(&moves, 34, 35, MoveKind::Quiet));
    }

    #[test]
    fn king_h1_corner_test() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[7] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::Black,
	});

	let moves = pos.king_targets(7).expect("king_targets returned Err");

	assert_eq!(moves.len(), 3);

	assert!(has_move(&moves, 7, 15, MoveKind::Quiet));
	assert!(has_move(&moves, 7, 14, MoveKind::Quiet));
	assert!(has_move(&moves, 7, 6, MoveKind::Quiet));
	assert!(!has_to_square(&moves, 8));
	assert!(!has_to_square(&moves, 16));
    }

    #[test]
    fn king_d5_enemy_e6() {
	let mut pos = empty_position();

	pos.board[35] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::White,
	});

	pos.board[44] = Some(ColoredPiece {
	    piece: crate::Piece::Pawn,
	    side: crate::Side::Black,
	});

	let moves = pos.king_targets(35).expect("king_targets returned Err");

	assert_eq!(moves.len(), 8);

	assert!(has_move(&moves, 35, 44, MoveKind::Capture));
    }

    #[test]
    fn king_h5_friendly_g4() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[39] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::Black,
	});

	pos.board[30] = Some(ColoredPiece {
	    piece: crate::Piece::Pawn,
	    side: crate::Side::Black,
	});

	let moves = pos.king_targets(39).expect("king_targets returned Err");

	assert_eq!(moves.len(), 4);

	assert!(!has_to_square(&moves, 30));
    }

    #[test]
    fn wrong_piece_e8() {
	let mut pos = empty_position();

	pos.board[60] = Some(ColoredPiece {
	    piece: crate::Piece::Knight,
	    side: crate::Side::White,
	});

	assert_eq!(
	    pos.king_targets(60),
	    Err(ChessError::WrongPieceType {
		expected_piece: Piece::King,
		found_piece: Piece::Knight,
	    })
	);
    }

    #[test]
    fn no_piece_d5() {
	let pos = empty_position();

	assert_eq!(
	    pos.king_targets(35),
	    Err(ChessError::NoPieceOnSquare { square: 35 })
	)
    }

    #[test]
    fn try_move_on_non_existing_square() {
	let pos = empty_position();

	assert_eq!(
	    pos.king_targets(65),
	    Err(ChessError::NotASquareOnBoard { square: 65 })
	)
    }

    #[test]
    fn king_castling_white_kingside_allowed_and_clear() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::White,
	});
	pos.board[7] = Some(ColoredPiece {
	    piece: crate::Piece::Rook,
	    side: crate::Side::White,
	});
	pos.castle[0] = true; // white kingside

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(has_move(
		&moves,
		4,
		6,
		MoveKind::Castling {
		    rook_from: 7,
		    rook_to: 5
		}
	));
    }

    #[test]
    fn king_castling_white_queenside_blocked_by_piece() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::White,
	});
	pos.board[0] = Some(ColoredPiece {
	    piece: crate::Piece::Rook,
	    side: crate::Side::White,
	});
	pos.board[3] = Some(ColoredPiece {
	    piece: crate::Piece::Knight,
	    side: crate::Side::White,
	}); // block d1
	pos.castle[1] = true; // white queenside

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(!has_move(
		&moves,
		4,
		2,
		MoveKind::Castling {
		    rook_from: 0,
		    rook_to: 3
		}
	));
    }

    #[test]
    fn king_castling_black_queenside_allowed_and_clear() {
	let mut pos = empty_position();
	pos.side_to_move = Side::Black;

	pos.board[60] = Some(ColoredPiece {
	    piece: crate::Piece::King,
	    side: crate::Side::Black,
	});
	pos.board[56] = Some(ColoredPiece {
	    piece: crate::Piece::Rook,
	    side: crate::Side::Black,
	});
	pos.castle[3] = true; // black queenside

	let moves = pos.king_targets(60).expect("king_targets returned Err");

	assert!(has_move(&moves, 60, 58, MoveKind::Castling {
		    rook_from: 56,
		    rook_to: 59
		}
	));
    }

    #[test]
    fn king_castling_white_kingside_disallowed_when_in_check() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::White,
	});
	pos.board[7] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::White,
	});
	pos.board[60] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::Black,
	}); // attacks e1
	pos.castle[0] = true;

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(!has_move(
		&moves,
		4,
		6,
		MoveKind::Castling {
		    rook_from: 7,
		    rook_to: 5
		}
	));
    }

    #[test]
    fn king_castling_white_kingside_disallowed_when_path_square_attacked() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::White,
	});
	pos.board[7] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::White,
	});
	pos.board[61] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::Black,
	}); // attacks f1
	pos.castle[0] = true;

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(!has_move(
		&moves,
		4,
		6,
		MoveKind::Castling {
		    rook_from: 7,
		    rook_to: 5
		}
	));
    }

    #[test]
    fn king_castling_white_kingside_disallowed_when_destination_attacked() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::White,
	});
	pos.board[7] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::White,
	});
	pos.board[62] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::Black,
	}); // attacks g1
	pos.castle[0] = true;

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(!has_move(
		&moves,
		4,
		6,
		MoveKind::Castling {
		    rook_from: 7,
		    rook_to: 5
		}
	));
    }

    #[test]
    fn king_castling_white_kingside_disallowed_when_rook_missing() {
	let mut pos = empty_position();

	pos.board[4] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::White,
	});
	pos.castle[0] = true;

	let moves = pos.king_targets(4).expect("king_targets returned Err");

	assert!(!has_move(
		&moves,
		4,
		6,
		MoveKind::Castling {
		    rook_from: 7,
		    rook_to: 5
		}
	));
    }
}
