use crate::{Side, Square, position::Position};


#[derive(Debug)]
pub enum MoveError {
    NoPieceOnInitalSquare(Square),
    OutOfBounds
}

impl Position {
    // No validation
    pub fn make_move(&self, initial_square: Square, target_square: Square) -> Result<Position, MoveError> {
        if initial_square > 63 || target_square > 63 {
            return Err(MoveError::OutOfBounds);
        }

        let potential_piece = self.board[initial_square as usize];
        let piece = match potential_piece {
           Some(x) => x,
           None => return Err(MoveError::NoPieceOnInitalSquare(initial_square))
        };

        let mut next_position = self.clone();

        next_position.board[initial_square as usize] = None;
        next_position.board[target_square as usize] = Some(piece);

        next_position.side_to_move = match next_position.side_to_move {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        Ok(next_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColoredPiece, Piece};

    use super::*;

    fn empty_position() -> Position {
        Position {
            board: [None; 64],
            side_to_move: Side::White,
            castle: [false; 4],
            en_passent: None,
        }
    }

    #[test]
    fn make_move_errors_if_initial_square_empty() {
        let pos = empty_position();

        let err = pos.make_move(0, 1).unwrap_err();
        match err {
            MoveError::NoPieceOnInitalSquare(sq) => assert_eq!(sq, 0),
            _ => panic!("expected NoPieceOnInitalSquare"),
        }
    }

    #[test]
    fn make_move_errors_if_out_of_bounds() {
        let pos = empty_position();

        assert!(matches!(pos.make_move(64, 0), Err(MoveError::OutOfBounds)));
        assert!(matches!(pos.make_move(0, 64), Err(MoveError::OutOfBounds)));
        assert!(matches!(pos.make_move(200, 201), Err(MoveError::OutOfBounds)));
    }

    #[test]
    fn make_move_moves_piece_clears_source_and_sets_target() {
        let mut pos = empty_position();

        let rook = ColoredPiece {
            piece: Piece::Rook,
            side: Side::White,
        };

        pos.board[0] = Some(rook); // a1

        let next = pos.make_move(0, 7).unwrap(); // a1 -> h1

        assert_eq!(next.board[0], None);
        assert_eq!(next.board[7], Some(rook));

        assert_eq!(pos.board[0], Some(rook));
        assert_eq!(pos.board[7], None);
    }

    #[test]
    fn make_move_overwrites_target_piece_capture_like() {
        let mut pos = empty_position();

        let white_rook = ColoredPiece {
            piece: Piece::Rook,
            side: Side::White,
        };
        let black_knight = ColoredPiece {
            piece: Piece::Knight,
            side: Side::Black,
        };

        pos.board[0] = Some(white_rook);
        pos.board[7] = Some(black_knight);

        let next = pos.make_move(0, 7).unwrap();

        assert_eq!(next.board[0], None);
        assert_eq!(next.board[7], Some(white_rook));
    }

    #[test]
    fn make_move_toggles_side_to_move() {
        let mut pos = empty_position();

        let pawn = ColoredPiece {
            piece: Piece::Pawn,
            side: Side::White,
        };
        pos.board[0] = Some(pawn);

        pos.side_to_move = Side::White;
        let next = pos.make_move(0, 1).unwrap();
        assert_eq!(next.side_to_move, Side::Black);

        pos.side_to_move = Side::Black;
        let next2 = pos.make_move(0, 1).unwrap();
        assert_eq!(next2.side_to_move, Side::White);
    }
}
