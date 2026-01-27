use crate::{Piece, Square, errors::MoveGenError, position::Position};

pub fn get_possible_moves(position: &Position, from_square: Square) -> Result<Vec<Square>, MoveGenError>{
    let piece = position.board[from_square as usize].ok_or(MoveGenError::NoPieceOnSquare { square: from_square })?;

    match piece.piece {
        Piece::Rook => position.rook_targets(from_square),
        Piece::Bishop => position.bishop_targets(from_square),
        Piece::King => position.king_targets(from_square),
        Piece::Pawn => {
            position.pawn_targets(from_square);
            let test: Vec<Square> = Vec::new(); 
            Ok(test)
        },
        Piece::Knight => position.knight_targets(from_square),
        Piece::Queen => Err(MoveGenError::NotImplemented),
    }
}
