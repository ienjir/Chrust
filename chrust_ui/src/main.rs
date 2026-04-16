mod assets;
mod controller;
mod input;
mod layout;
mod renderer;
mod helper;
mod state;

use crate::assets::Assets;
use crate::renderer::render_board;
use crate::state::{GameState, InputState};
use crate::{assets::load_assets, controller::apply_ui_event, input::route_click, layout::TEST_FEN_STRING};
use chrust_core::position::Game;
use macroquad::file::set_pc_assets_folder;
use macroquad::prelude::coroutines::wait_seconds;
use macroquad::prelude::*;

#[macroquad::main("Chrust")]
async fn main() {
	set_pc_assets_folder("chrust_ui/assets");

	let mut game_state = GameState {
		game: match Game::try_from_fen(TEST_FEN_STRING) {
			Ok(x) => x,
			Err(_x) => {
				println!("Error while loading position");
				return;
			}
		},
		assets: None,
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

		egui_macroquad::ui(|egui_ctx| {
			if game_state.assets.is_none() {
				game_state.assets = Some(load_assets(egui_ctx))
			}

			egui::SidePanel::right("sidebar").show(egui_ctx, |ui| {
				ui.label("Overview!");
			});

			render_board(egui_ctx, &mut game_state);

		});

		egui_macroquad::draw();

		wait_seconds(5f32);

		next_frame().await;
	}
}
