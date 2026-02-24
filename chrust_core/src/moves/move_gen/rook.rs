use crate::{ Piece, Square, errors::ChessError, moves::make_move::{Move}, position::Position };

impl Position {
    pub fn rook_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(14);

	let rook = match self.get_validated_colored_piece(from_square, Piece::Rook) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	self.horizontal_vertical_slider(from_square, rook, &mut target_moves);

        Ok(target_moves)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColoredPiece, moves::make_move::MoveKind};

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
    fn rook_h8_empty_board() {
        let mut pos = empty_position();

        pos.board[63] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(63).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 14);

        assert!(has_move(&moves, 63, 62, MoveKind::Quiet));
        assert!(has_move(&moves, 63, 56, MoveKind::Quiet));
        assert!(has_move(&moves, 63, 55, MoveKind::Quiet));
        assert!(has_move(&moves, 63, 7, MoveKind::Quiet));
    }

    #[test]
    fn rook_d4_empty_board() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 14);

        assert!(has_move(&moves, 27, 24, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 31, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 3, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 26, MoveKind::Quiet));
    }

    #[test]
    fn rook_d4_blocked_by_friendly_piece_f4() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 29));
        assert!(!has_to_square(&moves, 30));
    }

    #[test]
    fn rook_d4_captures_enemy_f4_and_stops() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::Black,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 29, MoveKind::Capture));
        assert!(!has_to_square(&moves, 30));
    }

    #[test]
    fn rook_a1_empty_board() {
        let mut pos = empty_position();

        pos.board[0] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(0).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 14);

        assert!(has_move(&moves, 0, 1, MoveKind::Quiet));
        assert!(has_move(&moves, 0, 7, MoveKind::Quiet));
        assert!(has_move(&moves, 0, 8, MoveKind::Quiet));
        assert!(has_move(&moves, 0, 56, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 63));
    }

    #[test]
    fn rook_d4_blocked_by_adjacent_friendly_pieces() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[19] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[26] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert!(moves.is_empty());
    }

    #[test]
    fn rook_d4_captures_adjacent_enemy_pieces() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[35] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[19] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[26] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });
        pos.board[28] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 4);
        assert!(has_move(&moves, 27, 35, MoveKind::Capture));
        assert!(has_move(&moves, 27, 19, MoveKind::Capture));
        assert!(has_move(&moves, 27, 26, MoveKind::Capture));
        assert!(has_move(&moves, 27, 28, MoveKind::Capture));
    }

    #[test]
    fn rook_d4_mixed_blockers() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Rook,
            side: crate::Side::White,
        });

        pos.board[43] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });
        pos.board[25] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.rook_targets(27).expect("knight_targets returned Err");

        assert_eq!(moves.len(), 10);

        assert!(has_move(&moves, 27, 35, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 43));
        assert!(has_move(&moves, 27, 19, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 11, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 3, MoveKind::Quiet));

        assert!(has_move(&moves, 27, 26, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 25, MoveKind::Capture));
        assert!(!has_to_square(&moves, 24));

        assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
        assert!(has_move(&moves, 27, 31, MoveKind::Quiet));
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position();

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(
            pos.rook_targets(60),
            Err(ChessError::WrongPieceType {
                expected_piece: Piece::Rook,
                found_piece: Piece::Knight,
            })
        );
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position();

        assert_eq!(
            pos.rook_targets(35),
            Err(ChessError::NoPieceOnSquare { square: 35 })
        )
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(
            pos.rook_targets(65),
            Err(ChessError::NotASquareOnBoard { square: 65 })
        )
    }
}
