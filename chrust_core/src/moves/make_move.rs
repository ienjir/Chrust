use std::usize;

use crate::{
    errors::ChessError,
    helper::is_square_on_board,
    position::{Position, Undo},
    ColoredPiece, Piece, Side, Square,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
    pub move_kind: MoveKind,
    pub colored_piece: ColoredPiece,
}

#[derive(PartialEq, Debug, Clone)]
pub enum MoveKind {
    Quiet,
    Capture,
    DoublePawnPush { passed_square: Square },
    EnPassant { capture_square: Square },
    Promotion { promotion_piece: Option<Piece> },
    Castling { rook_from: Square, rook_to: Square },
}

impl Position {
    pub fn make_move(&mut self, mv: &Move) -> Result<Undo, ChessError> {
	let legal_moves = match self.get_legal_moves(mv.from_square) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	if let Err(x) = is_square_on_board(mv.to_square) {
	    return Err(x);
	}

	if !legal_moves.contains(mv) {
	    return Err(ChessError::NotAValidMove); 
	}

	let mut piece = match self.get_unvalidated_colored_piece_from_square(mv.from_square) {
	    Ok(x) => x,
	    Err(x) => return Err(x),
	};

	let mut undo = Undo {
	    captured_piece: None,
	    previous_en_passant: self.en_passant,
	    previous_king_squares: self.king_squares,
	    previous_halfway_clock: self.halfmove_clock,
	    previous_castling_rights: self.castle,
	    fullmove_number: self.fullmove_number,
	};

	match mv.move_kind {
	    MoveKind::Quiet => {
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
	    }
	    MoveKind::Capture => {
		undo.captured_piece = self.board[mv.to_square as usize];
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
	    }
	    MoveKind::EnPassant { capture_square } => {
		undo.captured_piece = self.board[capture_square as usize];
		self.board[capture_square as usize] = None;
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
	    }
	    MoveKind::DoublePawnPush { passed_square: _ } => {
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
	    }
	    MoveKind::Promotion { promotion_piece } => {
		if promotion_piece.is_none() {
		    return Err(ChessError::PromotionPieceCantBeEmpty);
		}

		piece.piece =
		    promotion_piece.expect("make_move_unvalidated(): Promotion piece is none");
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
	    }
	    MoveKind::Castling { rook_from, rook_to } => {
		self.board[mv.from_square as usize] = None;
		self.board[mv.to_square as usize] = Some(piece);
		self.board[rook_from as usize] = None;
		self.board[rook_to as usize] = Some(crate::ColoredPiece {
		    piece: Piece::Rook,
		    side: piece.side,
		})
	    }
	};

	if let MoveKind::DoublePawnPush { passed_square } = mv.move_kind {
	    self.en_passant = Some(passed_square);
	} else {
	    self.en_passant = None;
	}

	match self.side_to_move {
	    Side::White => { self.side_to_move = Side::Black },
	    Side::Black => { 
		self.side_to_move = Side::White;
		self.fullmove_number += 1;
	    },
	}

	if mv.colored_piece.piece == Piece::Pawn || matches!(mv.move_kind, MoveKind::Capture | MoveKind::EnPassant { .. }) {
	    self.halfmove_clock = 0;
	} else {
	    self.halfmove_clock += 1;
	}

	match mv.from_square {
	    4  => { self.castle[0] = false; self.castle[1] = false; } 
	    60 => { self.castle[2] = false; self.castle[3] = false; } 
	    0  => { self.castle[1] = false; } 
	    7  => { self.castle[0] = false; }
	    56 => { self.castle[3] = false; } 
	    63 => { self.castle[2] = false; } 
	    _ => {}
	}

	match mv.to_square {
	    0  => { self.castle[1] = false; } 
	    7  => { self.castle[0] = false; } 
	    56 => { self.castle[3] = false; } 
	    63 => { self.castle[2] = false; } 
	    _ => {}
	}

	if mv.colored_piece.piece == Piece::King {
	    match mv.colored_piece.side {
		Side::White => {
		    self.king_squares[0] = mv.to_square;
		}
		Side::Black => {
		    self.king_squares[1] = mv.to_square;
		}
	    }
	}

	Ok(undo)
    }

    pub fn undo_move(&mut self, undo: Undo, mv: Move) -> Result<(), ChessError> {
	match mv.move_kind {
	    MoveKind::Quiet => {
		self.board[mv.from_square as usize] = Some(mv.colored_piece);
		self.board[mv.to_square as usize] = None;
	    }
	    MoveKind::Capture => {
		self.board[mv.from_square as usize] = Some(mv.colored_piece);
		self.board[mv.to_square as usize] = undo.captured_piece;
	    }
	    MoveKind::EnPassant { capture_square } => {

	    }
	    MoveKind::Promotion { promotion_piece } => {}
	    MoveKind::DoublePawnPush { passed_square } => {}
	    MoveKind::Castling { rook_from, rook_to } => {}
	}

	self.fullmove_number = undo.fullmove_number;
	self.castle = undo.previous_castling_rights;
	self.en_passant = undo.previous_en_passant;
	self.king_squares = undo.previous_king_squares;
	self.side_to_move = match self.side_to_move {
	    Side::White => Side::Black,
	    Side::Black => Side::White,
	};

	Ok(())
    }
}

