mod assets;
mod controller;
mod input;
mod layout;
mod renderer;
mod state;
use std::usize;

use crate::state::{GameState, InputState};
use crate::{assets::load_assets, controller::apply_ui_event, input::route_click, layout::TEST_FEN_STRING};
use chrust_core::helper::square;
use chrust_core::position::Game;
use egui::{Color32, Pos2, Rect};
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

		egui_macroquad::ui(|egui_ctx| {
			egui::SidePanel::right("sidebar").show(egui_ctx, |ui| {
				ui.label("Overview!");
			});

			egui::CentralPanel::default().show(egui_ctx, |ui| {
				let rect = ui.available_rect_before_wrap();
				let board_size = rect.width().min(rect.height());
				let square_side = board_size / 8f32;

				let response = ui.allocate_rect(rect, egui::Sense::click());

				if response.clicked() {
					let mouse_position = response.interact_pointer_pos().expect("Hello");
					let y = mouse_position.y - rect.min.y;
					let x = mouse_position.x - rect.min.x;

					let rank = (y / square_side).floor();
					let file = (x / square_side).floor();

					println!("Clicked Square: {}", square(file as u8, rank as u8));
					game_state.selected = Some(square(file as u8, rank as u8));
				}

				for rank in (0..8).rev() {
					for file in 0..8 {
						let square_idk = square(file, rank);
						let x = file as f32 * square_side + rect.min.y;
						let y = rank as f32 * square_side + rect.min.x;

						let mut color = Color32::from_rgb(250, 150, 250);
						if (rank + file) % 2 == 1 {
							color = Color32::from_rgb(77, 77, 70);
						};

						if Some(square_idk) == game_state.selected {
							color = Color32::from_rgb(140, 90, 210)
						}

						let square_rect = Rect {
							min: Pos2 { x, y },
							max: Pos2 {
								x: (x + square_side as f32),
								y: (y + square_side as f32),
							},
						};

						let _idk = ui.painter().rect_filled(square_rect, 0, color);

						let Some(piece) = game_state.game.position.board[square_idk as usize] else {
							continue;
						};
					}
				}
			});
		});

		egui_macroquad::draw();

		next_frame().await;
	}
}
