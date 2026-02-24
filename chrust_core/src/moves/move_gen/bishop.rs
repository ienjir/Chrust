use crate::{ Piece, Square, errors::ChessError, moves::make_move::{Move}, position::Position };

impl Position {
    pub fn bishop_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut to_moves: Vec<Move> = Vec::with_capacity(13);

	let bishop = match self.get_validated_colored_piece(from_square, Piece::Bishop) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	self.diagonal_slider(from_square, bishop, &mut to_moves);

        Ok(to_moves)
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
    fn bishop_g7_empty_boad() {
        let mut pos = empty_position();

        pos.board[54] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(54).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 9);

        assert!(has_move(&moves, 54, 63, MoveKind::Quiet));
        assert!(has_move(&moves, 54, 61, MoveKind::Quiet));
        assert!(has_move(&moves, 54, 47, MoveKind::Quiet));
        assert!(has_move(&moves, 54, 27, MoveKind::Quiet));
        assert!(has_move(&moves, 54, 0, MoveKind::Quiet));
    }

    #[test]
    fn bishop_h7_corner_test() {
        let mut pos = empty_position();

        pos.board[7] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(7).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 7);

        assert!(has_move(&moves, 7, 14, MoveKind::Quiet));
        assert!(has_move(&moves, 7, 56, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 8));
        assert!(!has_to_square(&moves, 16));
        assert!(!has_to_square(&moves, 0));
    }

    #[test]
    fn bishop_c7_enemy_f4() {
        let mut pos = empty_position();

        pos.board[50] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.bishop_targets(50).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 7);

        assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
        assert!(has_move(&moves, 50, 29, MoveKind::Capture));
        assert!(!has_to_square(&moves, 22));
    }

    #[test]
    fn bishop_b3_friendly_e6() {
        let mut pos = empty_position();

        pos.board[17] = Some(ColoredPiece {
            piece: crate::Piece::Bishop,
            side: crate::Side::White,
        });

        pos.board[44] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::White,
        });

        let moves = pos.bishop_targets(17).expect("bishop_targets returned Err");

        assert_eq!(moves.len(), 6);

        assert!(has_move(&moves, 17, 35, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 44));
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position();

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(
            pos.bishop_targets(60),
            Err(ChessError::WrongPieceType {
                expected_piece: Piece::Bishop,
                found_piece: Piece::Knight,
            })
        );
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position();

        assert_eq!(
            pos.bishop_targets(35),
            Err(ChessError::NoPieceOnSquare { square: 35 })
        )
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(
            pos.bishop_targets(65),
            Err(ChessError::NotASquareOnBoard { square: 65 })
        )
    }
}
