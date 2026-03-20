use std::usize;

use crate::{
	ColoredPiece, Piece, Side, Square,
	errors::ChessError,
	helper::{is_square_on_board, is_valid_promomotion_piece},
	position::{Position, Undo}, zobrist::{ZobristTable, piece_index, zobrist},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Move {
	pub from_square: Square,
	pub to_square: Square,
	pub move_kind: MoveKind,
	pub colored_piece: ColoredPiece,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MoveKind {
	Quiet,
	Capture,
	DoublePawnPush {
		passed_square: Square,
	},
	EnPassant {
		capture_square: Square,
	},
	Promotion {
		promotion_piece: Piece,
	},
	Castling {
		rook_from: Square,
		rook_to: Square,
	},
}

impl Position {
	pub fn make_move(&mut self, mv: &Move) -> Result<Undo, ChessError> {
		let legal_moves = self.get_legal_moves(mv.from_square, self.side_to_move)?;

		is_square_on_board(mv.to_square)?;

		if !legal_moves.contains(mv) {
			return Err(ChessError::NotAValidMove);
		}

		self.make_move_unvalidated(*mv)
	}

	pub fn make_move_unvalidated(&mut self, mv: Move) -> Result<Undo, ChessError> {
		let piece = self.get_piece_from_square(mv.from_square)?;
		let mut undo = self.build_undo();

		let zobrist = zobrist();

		for i in 0..4 {
			if self.castle[i] {
				self.zobrist_hash ^= zobrist.castling[i];
			}
		}
		if let Some(ep) = self.en_passant {
			self.zobrist_hash ^= zobrist.enpassant[(ep % 8) as usize];
		}

		self.apply_move_to_board(mv, piece, &mut undo, zobrist)?;
		self.update_en_passant(mv);
		self.update_clocks_and_side(mv, zobrist);
		self.update_king_positions(mv);
		self.set_castle_rights(mv);

		for i in 0..4 {
			if self.castle[i] {
				self.zobrist_hash ^= zobrist.castling[i];
			}
		}
		if let Some(ep) = self.en_passant {
			self.zobrist_hash ^= zobrist.enpassant[(ep % 8) as usize];
		}

		Ok(undo)
	}

	pub fn undo_move(&mut self, undo: Undo, mv: Move) -> Result<(), ChessError> {
		let zobrist = zobrist();

		for i in 0..4 {
			if self.castle[i] {
				self.zobrist_hash ^= zobrist.castling[i];
			}
		}

		if let Some(ep) = self.en_passant {
			self.zobrist_hash ^= zobrist.enpassant[(ep % 8) as usize];
		}

		self.undo_move_on_board(mv, undo, zobrist);
		self.apply_undo(undo); 

		for i in 0..4 {
			if self.castle[i] {
				self.zobrist_hash ^= zobrist.castling[i];
			}
		}


		if let Some(ep) = self.en_passant {
			self.zobrist_hash ^= zobrist.enpassant[(ep % 8) as usize];
		}

		self.zobrist_hash ^= zobrist.side;

		Ok(())
	}

	pub fn build_undo(&self) -> Undo {
		Undo {
			captured_piece: None,
			previous_en_passant: self.en_passant,
			previous_king_squares: self.king_squares,
			previous_halfway_clock: self.halfmove_clock,
			previous_castling_rights: self.castle,
			fullmove_number: self.fullmove_counter,
		}
	}

	pub fn apply_undo(&mut self, undo: Undo) {
		self.halfmove_clock = undo.previous_halfway_clock;
		self.fullmove_counter = undo.fullmove_number;
		self.castle = undo.previous_castling_rights;
		self.en_passant = undo.previous_en_passant;
		self.king_squares = undo.previous_king_squares;
		self.side_to_move = self.side_to_move.opponent();
	}

	pub fn undo_move_on_board(&mut self, mv: Move, undo: Undo, zobrist: &ZobristTable) {
		let piece = self.board[mv.to_square as usize].unwrap();

		self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.to_square as usize];
		self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.from_square as usize];

		if let Some(captured) = undo.captured_piece {
			if !matches!(mv.move_kind, MoveKind::EnPassant { .. }) {
				self.zobrist_hash ^= zobrist.pieces[piece_index(captured)][mv.to_square as usize];
			}
		}

		self.board[mv.from_square as usize] = Some(mv.colored_piece);
		self.board[mv.to_square as usize] = undo.captured_piece;

		match mv.move_kind {
			MoveKind::EnPassant { capture_square } => {
				self.board[capture_square as usize] = undo.captured_piece;
				self.board[mv.to_square as usize] = None;

				self.zobrist_hash ^= zobrist.pieces[piece_index(undo.captured_piece.unwrap())][capture_square as usize];
			}
			MoveKind::Promotion { promotion_piece } => {
				self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.from_square as usize];       
				self.zobrist_hash ^= zobrist.pieces[piece_index(mv.colored_piece)][mv.from_square as usize]; 
			}
			MoveKind::Castling { rook_from, rook_to } => {
				let rook = self.board[rook_to as usize].unwrap(); // rook is currently at rook_to
																  //
				self.zobrist_hash ^= zobrist.pieces[piece_index(rook)][rook_to as usize];
				self.zobrist_hash ^= zobrist.pieces[piece_index(rook)][rook_from as usize];

				self.board[rook_from as usize] = Some(rook);
				self.board[rook_to as usize] = None;
			}
			_ => {}
		}
	}

	pub fn apply_move_to_board(&mut self, mv: Move, piece: ColoredPiece, undo: &mut Undo, zobrist: &ZobristTable) -> Result<(), ChessError> {
		self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.from_square as usize];
		self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.to_square as usize];

		// Remove piece from hash for capture and promotion capture
		if self.board[mv.to_square as usize].is_some() {
			self.zobrist_hash ^= zobrist.pieces[piece_index(self.board[mv.to_square as usize].unwrap())][mv.to_square as usize]
		}

		self.board[mv.from_square as usize] = None;
		undo.captured_piece = self.board[mv.to_square as usize];
		self.board[mv.to_square as usize] = Some(piece);

		match mv.move_kind {
			MoveKind::EnPassant { capture_square } => {
				undo.captured_piece = self.board[capture_square as usize];
				self.board[capture_square as usize] = None;

				self.zobrist_hash ^= zobrist.pieces[piece_index(undo.captured_piece.unwrap())][capture_square as usize];
			}

			MoveKind::Promotion { promotion_piece } => {
				is_valid_promomotion_piece(promotion_piece)?;
				let promotion_colored_piece = ColoredPiece { piece: promotion_piece, side: piece.side };

				self.zobrist_hash ^= zobrist.pieces[piece_index(piece)][mv.to_square as usize];
				self.zobrist_hash ^= zobrist.pieces[piece_index(promotion_colored_piece)][mv.to_square as usize];

				self.board[mv.to_square as usize] = Some(ColoredPiece { piece: promotion_piece, side: piece.side });
			}

			MoveKind::Castling { rook_from, rook_to } => {
				let rook = self.board[rook_from as usize].unwrap();

				self.zobrist_hash ^= zobrist.pieces[piece_index(rook)][rook_from as usize];
				self.zobrist_hash ^= zobrist.pieces[piece_index(rook)][rook_to as usize];

				self.board[rook_from as usize] = None;
				self.board[rook_to as usize] = Some(ColoredPiece { piece: Piece::Rook, side: piece.side });
			}

			_ => {}
		}

		Ok(())
	}

	pub fn update_en_passant(&mut self, mv: Move) {
		if let MoveKind::DoublePawnPush { passed_square } = mv.move_kind {
			self.en_passant = Some(passed_square);
		} else {
			self.en_passant = None;
		}
	}

	pub fn update_clocks_and_side(&mut self, mv: Move, zobrist: &ZobristTable) {
		if mv.colored_piece.piece == Piece::Pawn || matches!(mv.move_kind, MoveKind::Capture | MoveKind::EnPassant { .. }) {
			self.halfmove_clock = 0;
		} else {
			self.halfmove_clock += 1;
		}

		self.zobrist_hash ^= zobrist.side;

		match self.side_to_move {
			Side::White => self.side_to_move = Side::Black,
			Side::Black => {
				self.side_to_move = Side::White;
				self.fullmove_counter += 1;
			}
		}
	}

	pub fn update_king_positions(&mut self, mv: Move) {
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
	}

	pub fn set_castle_rights(&mut self, mv: Move) {
		match mv.from_square {
			4 => {
				self.castle[0] = false;
				self.castle[1] = false
			}
			60 => {
				self.castle[2] = false;
				self.castle[3] = false
			}
			0 => self.castle[1] = false,
			7 => self.castle[0] = false,
			56 => self.castle[3] = false,
			63 => self.castle[2] = false,
			_ => {}
		}

		match mv.to_square {
			0 => self.castle[1] = false,
			7 => self.castle[0] = false,
			56 => self.castle[3] = false,
			63 => self.castle[2] = false,
			_ => {}
		}
	}
}
