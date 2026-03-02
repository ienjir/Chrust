use crate::{ColoredPiece, Piece, Square, errors::ChessError, moves::make_move::{Move, MoveKind}, position::Position};

impl Position {
    /// Does not validate yet
    pub fn get_legal_moves(&self, from_square: Square) -> Result<Vec<Move>, ChessError> {
	let colored_piece = match self.get_unvalidated_colored_piece_from_square(from_square) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	let pseudo_moves = match self.get_pseduo_legal_moves(from_square, colored_piece) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	for pseudo_move in pseudo_moves {
	    
	}

	let test: Vec<Move> = Vec::new();
	Ok(test)
    }

    pub fn get_pseduo_legal_moves(&self, from_square: Square, colored_piece: ColoredPiece) -> Result<Vec<Move>, ChessError> {
	match colored_piece.piece {
	    Piece::Rook => self.rook_targets(from_square),
	    Piece::Bishop => self.bishop_targets(from_square),
	    Piece::King => self.king_targets(from_square),
	    Piece::Pawn => self.pawn_targets(from_square),
	    Piece::Knight => self.knight_targets(from_square),
	    Piece::Queen => self.queen_targets(from_square),
	}
    }
}
