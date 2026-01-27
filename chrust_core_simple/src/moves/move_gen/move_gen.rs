use crate::{Piece, Square, errors::MoveGenError, position::Position};

pub fn get_possible_moves(position: &Position, from_square: Square) -> Result<Vec<Square>, MoveGenError>{
    let piece = position.board[from_square as usize].ok_or(MoveGenError::NoPieceOnSquare { square: from_square })?;

    match piece.piece {
        Piece::Rook => Err(MoveGenError::NotImplemented),
        Piece::Bishop => Err(MoveGenError::NotImplemented),
        Piece::King => Err(MoveGenError::NotImplemented),
        Piece::Pawn => Err(MoveGenError::NotImplemented),
        Piece::Knight => Err(MoveGenError::NotImplemented),
        Piece::Queen => Err(MoveGenError::NotImplemented),
    }
}
