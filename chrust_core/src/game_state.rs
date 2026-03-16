// get_all_legal_moves()
// 		Loop throug all 64 squares. If piece = side to move -> get legal moves -> add to vector
// is_checkmate()
// 		King needs to be in check, no legal moves exist
// is_stalemate()
// 		King cant be in check, no legal moves exist
// is_draw_by_fifty_moves()
// 		if halway clock is over 100 (50 full moves)
// is_insufficient_material()
// 		if only kings, knight/bishop + 2 kings, 2 bishops on same square color + 2 kings. Bishop color: (file + rank) % 2
// is_draw_by_repetition()
// 		after each move make hash of position -> hashmap key = position hash, value = how many position were the same i		
// get_game_status() -> GameStatus

use crate::{Side, position::Position};

pub enum GameStatus {
	Playing,
	InCheck,
	Checkmate(Side),
	Stalemate,
	DrawByFiftyMoves,
	DrawByRepetition,
	DrawByInsufficientMaterial,
}

impl Position {
	pub fn is_draw_by_fifty_moves(&self) -> bool {
		if self.halfmove_clock <= 100 {
			return true;
		}

		false
	}
}
