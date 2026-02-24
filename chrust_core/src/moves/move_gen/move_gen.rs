use crate::{errors::ChessError, moves::make_move::Move, position::Position, Piece, Square};

pub fn get_possible_moves(position: &Position, from_square: Square) -> Result<Vec<Move>, ChessError> {
    if from_square > 63 {
        return Err(ChessError::NotASquareOnBoard {
            square: from_square,
        });
    }

    let piece = position.board[from_square as usize].ok_or(ChessError::NoPieceOnSquare {
        square: from_square,
    })?;

    match piece.piece {
        Piece::Rook => position.rook_targets(from_square),
        Piece::Bishop => position.bishop_targets(from_square),
        Piece::King => position.king_targets(from_square),
        Piece::Pawn => position.pawn_targets(from_square),
        Piece::Knight => position.knight_targets(from_square),
        Piece::Queen => position.queen_targets(from_square),
    }
}
