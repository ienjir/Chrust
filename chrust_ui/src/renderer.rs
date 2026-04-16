use crate::{helper::position_to_square, layout::{BOARD_BLACK_COLOR, BOARD_HIGHLIGHTED_COLOR, BOARD_WHITE_COLOR}, state::GameState};
use chrust_core::{helper::{square}};
use core::f32;
use std::usize;
use egui::{Context, Pos2, Rect};

pub(crate) fn render_board(egui_ctx: &Context, game_state: &mut GameState) {
	egui::CentralPanel::default().show(egui_ctx, |ui| {
		let board = ui.available_rect_before_wrap();
		let board_size = board.width().min(board.height());
		let square_side = board_size / 8f32;

		let response = ui.allocate_rect(board, egui::Sense::click());

		if response.clicked() {
			let mouse_position = response.interact_pointer_pos().expect("Hello");
			
			let (file, rank) = position_to_square(mouse_position, board, square_side);

			game_state.selected = Some(square(file as u8, rank as u8));
		}

		for rank in (0..8).rev() {
			for file in 0..8 {
				let square = square(file, rank);

				let absolute_x = file as f32 * square_side + board.min.x; 
				let absolute_y = (7 - rank) as f32 * square_side + board.min.y; 

				let mut color = BOARD_BLACK_COLOR;
				if (rank + file) % 2 == 1 {
					color = BOARD_WHITE_COLOR;
				};

				if Some(square) == game_state.selected {
					color = BOARD_HIGHLIGHTED_COLOR;
				}

				let square_rect = Rect {
					min: Pos2 { x: absolute_x, y: absolute_y },
					max: Pos2 {
						x: (absolute_x + square_side as f32),
						y: (absolute_y + square_side as f32),
					},
				};

				let _ = ui.painter().rect_filled(square_rect, 0, color);

				let piece = match game_state.game.position.board[square as usize] {
					Some(x) => x,
					None => continue,
				};


			}
		}
	});
}
