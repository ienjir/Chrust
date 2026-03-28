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
use chrust_core::{ColoredPiece, Piece, Side};
use chrust_core::game_status::GameStatus;
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
		possible_moves: Vec::new(),
		ui_state: None,
	};

	let sdjlkf = ColoredPiece {
		piece: Piece::Knight,
		side: Side::Black,
	};

	let test = game_state.game.update_game_status();

	if game_state.game.game_status == GameStatus::CheckmateForSide(Side::Black) {
		println!("Checkmate");
	}

	let test = match game_state.game.position.export_position_to_fen() {
		Ok(x) => x,
		Err(_x) => {
			println!("Error exportign");
			return;
		} 
	};

	println!("Export: {test}");

	game_state.game.position.print_board();

	if 1 == 1 {
		return;
	}

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
