use egui::{Pos2, Rect};

/// Returns (file, rank)
pub(crate) fn position_to_square(position: Pos2, board_rect: Rect, square_side: f32) -> (f32, f32) {
	let y = position.y - board_rect.min.y;
	let x = position.x - board_rect.min.x;

	let rank = (8.0 - y / square_side).floor();
	let file = (x / square_side).floor();

	return (file, rank);
}
