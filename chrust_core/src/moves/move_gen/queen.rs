use crate::{ Piece, Square, errors::ChessError, helper::{file, rank}, moves::make_move::{Move, MoveKind}, position::Position };

impl Position {
    pub fn queen_targets(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
        let mut target_moves: Vec<Move> = Vec::with_capacity(27);

	let queen = match self.get_validated_colored_piece(from_square, Piece::Queen) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

        let directions: [i16; 8] = [-8, 8, -1, 1, -7, 7, -9, 9];

        for direction in directions {
            let mut step_from_i: i16 = from_square as i16;
            loop {
                let step_to_i = step_from_i + direction;

                if !(0..=63).contains(&step_to_i) {
                    break;
                }

                let file_difference_i =
                    (file(step_to_i as u8) as i16 - file(step_from_i as u8) as i16).abs();
                let rank_difference_i =
                    (rank(step_to_i as u8) as i16 - rank(step_from_i as u8) as i16).abs();

                if direction.abs() == 8 {
                    if file_difference_i != 0 || rank_difference_i != 1 {
                        break;
                    }
                } else if direction.abs() == 1 {
                    if file_difference_i != 1 || rank_difference_i != 0 {
                        break;
                    }
                } else {
                    if rank_difference_i != 1 || file_difference_i != 1 {
                        break;
                    }
                }

                let candidate_occupant = self.board[step_to_i as usize];
                match candidate_occupant {
                    None => {
                        target_moves.push(Move {
                            colored_piece: queen,
                            from_square: from_square,
                            to_square: step_to_i as u8,
                            move_kind: MoveKind::Quiet,
                        });
                        step_from_i = step_to_i;
                    }
                    Some(colored_piece) => {
                        if colored_piece.side != queen.side {
                            target_moves.push(Move {
                                colored_piece: queen,
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
    fn queen_d4_empty_board() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Queen,
            side: crate::Side::White,
        });

        let moves = pos.queen_targets(27).expect("queen_targets returned Err");

        assert_eq!(moves.len(), 27);

        assert!(has_move(&moves, 27, 31, MoveKind::Quiet)); // h4
        assert!(has_move(&moves, 27, 3, MoveKind::Quiet)); // d1
        assert!(has_move(&moves, 27, 63, MoveKind::Quiet)); // h8
        assert!(has_move(&moves, 27, 18, MoveKind::Quiet)); // c3
        assert!(has_move(&moves, 27, 36, MoveKind::Quiet)); // e5
    }

    #[test]
    fn queen_d4_blocked_by_friendly_piece_f4() {
        let mut pos = empty_position();

        pos.board[27] = Some(ColoredPiece {
            piece: crate::Piece::Queen,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        let moves = pos.queen_targets(27).expect("queen_targets returned Err");

        assert!(has_move(&moves, 27, 28, MoveKind::Quiet));
        assert!(!has_to_square(&moves, 29));
        assert!(!has_to_square(&moves, 30));
    }

    #[test]
    fn queen_c7_enemy_f4() {
        let mut pos = empty_position();

        pos.board[50] = Some(ColoredPiece {
            piece: crate::Piece::Queen,
            side: crate::Side::White,
        });

        pos.board[29] = Some(ColoredPiece {
            piece: crate::Piece::Pawn,
            side: crate::Side::Black,
        });

        let moves = pos.queen_targets(50).expect("queen_targets returned Err");

        assert!(has_move(&moves, 50, 36, MoveKind::Quiet));
        assert!(has_move(&moves, 50, 29, MoveKind::Capture));
        assert!(!has_to_square(&moves, 22));
    }

    #[test]
    fn wrong_piece_e8() {
        let mut pos = empty_position();

        pos.board[60] = Some(ColoredPiece {
            piece: crate::Piece::Knight,
            side: crate::Side::White,
        });

        assert_eq!(
            pos.queen_targets(60),
            Err(ChessError::WrongPieceType {
                expected_piece: Piece::Queen,
                found_piece: Piece::Knight,
            })
        );
    }

    #[test]
    fn no_piece_d5() {
        let pos = empty_position();

        assert_eq!(
            pos.queen_targets(35),
            Err(ChessError::NoPieceOnSquare { square: 35 })
        )
    }

    #[test]
    fn try_move_on_non_existing_square() {
        let pos = empty_position();

        assert_eq!(
            pos.queen_targets(65),
            Err(ChessError::NotASquareOnBoard { square: 65 })
        )
    }
}
