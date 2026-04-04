mod assets;
mod controller;
mod input;
mod layout;
mod renderer;
mod state;
use crate::state::{GameState, InputState};
use crate::{
	assets::load_assets,
	controller::apply_ui_event,
	input::route_click,
	layout::TEST_FEN_STRING,
	renderer::{handle_ui_state, render_chess_pieces, render_chessboard_without_pieces, render_possible_moves},
};
use chrust_core::position::Game;
use macroquad::file::set_pc_assets_folder;
use macroquad::prelude::*;

#[macroquad::main("Chrust")]
async fn main() {
	set_pc_assets_folder("chrust_ui/assets");
	let assets = load_assets().await;

	let mut game_state = GameState {
		game: match Game::try_from_fen(TEST_FEN_STRING) {
			Ok(x) => x,
			Err(_x) => {
				println!("Error while loading position");
				return;
			}
		},
		assets: assets,
		selected: None,
		legal_moves: Vec::new(),
		ui_state: None,
	};

	loop {
		clear_background(LIGHTGRAY);

		let (mouse_x, mouse_y) = mouse_position();
		let input_state = InputState {
			mouse_x: mouse_x,
			mouse_y: mouse_y,
			left_mouse_clicked: is_mouse_button_pressed(MouseButton::Left),
		};

		if let Some(ui_event) = route_click(&input_state, &game_state) {
			apply_ui_event(&mut game_state, ui_event);
		};

		render_chessboard_without_pieces(&game_state);
		render_chess_pieces(&game_state).await;
		render_possible_moves(&game_state);
		handle_ui_state(&game_state);

		next_frame().await;
	}
}
