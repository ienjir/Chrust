use crate::{
    errors::ChessError,
    position::{Position},
    ColoredPiece, Piece, Side, Square,
};

pub fn file(square: Square) -> u8 {
    square % 8
}

pub fn rank(square: Square) -> u8 {
    square / 8
}

pub fn square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

impl ColoredPiece {
    pub fn to_char(&self) -> char {
        let mut piece_char = match self.piece {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };

        if self.side == Side::White {
            piece_char = piece_char.to_ascii_uppercase();
        }

        piece_char
    }
}


/// Checks if a `Square` is in the 64 squares of a chessboard
pub fn is_square_on_board(from_square: Square) -> Result<(), ChessError> {
    if !(0..=63).contains(&from_square) {
        return Err(ChessError::NotASquareOnBoard {
            square: from_square,
        });
    } else {
        return Ok(());
    }
}

pub fn is_right_piece_type(from_piece: ColoredPiece, expected_piece: Piece) -> Result<(), ChessError> {
    if from_piece.piece != expected_piece {
        return Err(ChessError::WrongPieceType {
            expected_piece,
            found_piece: from_piece.piece,
        });
    } 

    Ok(())
}

pub fn is_right_piece_side(from_piece: ColoredPiece, expected_side: Side) -> Result<(), ChessError> {
    if from_piece.side != expected_side {
	return Err(ChessError::WrongSide {
	    expected_side,
	    found_side: from_piece.side,
	});
    } else {
	Ok(())
    }
}

impl Position {
    /// Gets a colored piece that is validated so that it acutually exists. Also validates the from_square
    pub fn get_validated_colored_piece(&self, from_square: Square, expected_piece: Piece) -> Result<ColoredPiece, ChessError> {
	let col_piece = match self.get_colored_piece_from_square(from_square) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	if let Err(x) = is_right_piece_side(col_piece, self.side_to_move) {
	    return Err(x);
	}

	if let Err(x) = is_right_piece_type(col_piece, expected_piece) {
	    return Err(x);
	}

	Ok(col_piece)
    }

    pub fn get_colored_piece_from_square(&self, from_square: Square) -> Result<ColoredPiece, ChessError> {
	if let Err(x) = is_square_on_board(from_square) {
	    return Err(x);
	}


	match self.board[from_square as usize] {
	    Some(p) => return Ok(p),
	    None => {
		return Err(ChessError::NoPieceOnSquare {
		    square: from_square,
		})
	    }
	};
    }
}

#[cfg(test)]
mod tests {
    use crate::moves::move_gen::move_gen::get_possible_moves;

    use super::*;

    fn empty_position() -> Position {
	Position {
	    board: [None; 64],
	    side_to_move: Side::White,
	    castle: [false; 4],
	    en_passant: None,
	    king_squares: [4, 60],
	}
    }

    #[test]
    fn is_square_on_board_positive_test() {
	assert_eq!(is_square_on_board(12), Ok(()));
    }

    #[test]
    fn is_square_on_board_negative_test_with_positive_number_64() {
	assert_eq!(
	    is_square_on_board(64),
	    Err(ChessError::NotASquareOnBoard { square: 64 })
	);
    }

    #[test]
    fn is_square_on_board_boundary_positives_0_and_63() {
	assert_eq!(is_square_on_board(0), Ok(()));
	assert_eq!(is_square_on_board(63), Ok(()));
    }

    #[test]
    fn is_square_on_board_negative_test_with_negative_number() {
	let negative_square_as_u8 = -1i8 as u8;

	assert_eq!(
	    is_square_on_board(negative_square_as_u8),
	    Err(ChessError::NotASquareOnBoard {
		square: negative_square_as_u8
	    })
	);
    }

    #[test]
    fn is_right_piece_type_positive_test_with_both_sides() {
	assert_eq!(
	    is_right_piece_type(
		ColoredPiece {
		    piece: Piece::King,
		    side: Side::White,
		},
		Piece::King,
	    ),
	    Ok(())
	);

	assert_eq!(
	    is_right_piece_type(
		ColoredPiece {
		    piece: Piece::Pawn,
		    side: Side::Black,
		},
		Piece::Pawn,
	    ),
	    Ok(())
	);
    }

    #[test]
    fn is_right_piece_type_negative_test_with_both_sides() {
	assert_eq!(
	    is_right_piece_type(
		ColoredPiece {
		    piece: Piece::King,
		    side: Side::White,
		},
		Piece::Queen,
	    ),
	    Err(ChessError::WrongPieceType {
		expected_piece: Piece::Queen,
		found_piece: Piece::King,
	    })
	);

	assert_eq!(
	    is_right_piece_type(
		ColoredPiece {
		    piece: Piece::Rook,
		    side: Side::Black,
		},
		Piece::Bishop,
	    ),
	    Err(ChessError::WrongPieceType {
		expected_piece: Piece::Bishop,
		found_piece: Piece::Rook,
	    })
	);
    }

    #[test]
    fn is_right_piece_side_positive_test_with_both_sides() {
	assert_eq!(
	    is_right_piece_side(
		ColoredPiece {
		    piece: Piece::Knight,
		    side: Side::White,
		},
		Side::White,
	    ),
	    Ok(())
	);

	assert_eq!(
	    is_right_piece_side(
		ColoredPiece {
		    piece: Piece::Knight,
		    side: Side::Black,
		},
		Side::Black,
	    ),
	    Ok(())
	);
    }

    #[test]
    fn is_right_piece_side_negative_test_with_both_sides() {
	assert_eq!(
	    is_right_piece_side(
		ColoredPiece {
		    piece: Piece::Knight,
		    side: Side::White,
		},
		Side::Black,
	    ),
	    Err(ChessError::WrongSide {
		expected_side: Side::Black,
		found_side: Side::White,
	    })
	);

	assert_eq!(
	    is_right_piece_side(
		ColoredPiece {
		    piece: Piece::Knight,
		    side: Side::Black,
		},
		Side::White,
	    ),
	    Err(ChessError::WrongSide {
		expected_side: Side::White,
		found_side: Side::Black,
	    })
	);
    }

    #[test]
    fn get_colored_piece_from_square_positive_test_for_both_sides() {
	let mut pos = empty_position();

	pos.board[0] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::White,
	});

	pos.board[63] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::Black,
	});

	assert_eq!(
	    pos.get_colored_piece_from_square(0),
	    Ok(ColoredPiece {
		piece: Piece::Rook,
		side: Side::White,
	    })
	);

	assert_eq!(
	    pos.get_colored_piece_from_square(63),
	    Ok(ColoredPiece {
		piece: Piece::Rook,
		side: Side::Black,
	    })
	);
    }

    #[test]
    fn get_colored_piece_from_square_negative_test_for_both_sides() {
	let pos_white = empty_position();

	assert_eq!(
	    pos_white.get_colored_piece_from_square(10),
	    Err(ChessError::NoPieceOnSquare { square: 10 })
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;

	assert_eq!(
	    pos_black.get_colored_piece_from_square(54),
	    Err(ChessError::NoPieceOnSquare { square: 54 })
	);
    }

    #[test]
    fn get_colored_piece_from_square_invalid_square_test() {
	let pos = empty_position();

	assert_eq!(
	    pos.get_colored_piece_from_square(64),
	    Err(ChessError::NotASquareOnBoard { square: 64 })
	);
    }

    #[test]
    fn get_validated_colored_piece_positive_test_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.board[0] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::White,
	});

	assert_eq!(
	    pos_white.get_validated_colored_piece(0, Piece::Rook),
	    Ok(ColoredPiece {
		piece: Piece::Rook,
		side: Side::White,
	    })
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[63] = Some(ColoredPiece {
	    piece: Piece::Knight,
	    side: Side::Black,
	});

	assert_eq!(
	    pos_black.get_validated_colored_piece(63, Piece::Knight),
	    Ok(ColoredPiece {
		piece: Piece::Knight,
		side: Side::Black,
	    })
	);
    }

    #[test]
    fn get_validated_colored_piece_wrong_expected_piece_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.board[4] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::White,
	});

	assert_eq!(
	    pos_white.get_validated_colored_piece(4, Piece::Queen),
	    Err(ChessError::WrongPieceType {
		expected_piece: Piece::Queen,
		found_piece: Piece::King,
	    })
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[60] = Some(ColoredPiece {
	    piece: Piece::King,
	    side: Side::Black,
	});

	assert_eq!(
	    pos_black.get_validated_colored_piece(60, Piece::Rook),
	    Err(ChessError::WrongPieceType {
		expected_piece: Piece::Rook,
		found_piece: Piece::King,
	    })
	);
    }

    #[test]
    fn get_validated_colored_piece_wrong_expected_side_for_both_sides() {
	let mut pos_white = empty_position();
	pos_white.side_to_move = Side::White;
	pos_white.board[60] = Some(ColoredPiece {
	    piece: Piece::Knight,
	    side: Side::Black,
	});

	assert_eq!(
	    pos_white.get_validated_colored_piece(60, Piece::Knight),
	    Err(ChessError::WrongSide {
		expected_side: Side::White,
		found_side: Side::Black,
	    })
	);

	let mut pos_black = empty_position();
	pos_black.side_to_move = Side::Black;
	pos_black.board[4] = Some(ColoredPiece {
	    piece: Piece::Knight,
	    side: Side::White,
	});

	assert_eq!(
	    pos_black.get_validated_colored_piece(4, Piece::Knight),
	    Err(ChessError::WrongSide {
		expected_side: Side::Black,
		found_side: Side::White,
	    })
	);
    }

    #[test]
    fn get_validated_colored_piece_error_precedence_invalid_square_short_circuits() {
	let pos = empty_position();

	assert_eq!(
	    pos.get_validated_colored_piece(64, Piece::Queen),
	    Err(ChessError::NotASquareOnBoard { square: 64 })
	);
    }

    #[test]
    fn integration_get_possible_moves_respects_side_to_move() {
	let mut pos = empty_position();
	pos.side_to_move = Side::White;
	pos.board[63] = Some(ColoredPiece {
	    piece: Piece::Rook,
	    side: Side::Black,
	});

	assert_eq!(
	    get_possible_moves(&pos, 63),
	    Err(ChessError::WrongSide {
		expected_side: Side::White,
		found_side: Side::Black,
	    })
	);
    }
}
