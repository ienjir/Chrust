use crate::{Piece, Square, position::Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveGenError {
    NotASquareOnBoard {square: Square}, 
    WrongPieceTypeOnSquare {expected_piece: Piece, found_piece: Piece, square: Square},
    NoPieceOnSquare {square: Square},
}

pub fn get_possible_moves(position: &Position, from_square: Square) -> Result<Vec<Square>, MoveGenError>{
    let piece = position.board[from_square as usize].ok_or(MoveGenError::NoPieceOnSquare { square: from_square })?;

    match piece.piece {
        Piece::Rook => position.rook_targets(from_square),
        Piece::Bishop => position.bishop_targets(from_square),
        Piece::King => position.king_targets(from_square),
        Piece::Pawn => position.pawn_targets(from_square),
        Piece::Knight => position.knight_targets(from_square),
        Piece::Queen => {
            // Queen not implemented yet 
            Err(MoveGenError::WrongPieceTypeOnSquare { expected_piece: Piece::Queen, found_piece: Piece::Queen, square: from_square })
        }
    }
}
