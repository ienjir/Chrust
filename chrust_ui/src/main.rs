mod assets;
mod controller;
mod helper;
mod input;
mod layout;
mod renderer;
mod state;

use crate::assets::load_assets;
use crate::controller::{apply_ui_event, UiEvent};
use crate::renderer::render_board;
use crate::state::GameState;
use chrust_core::game_status::GameStatus;
use chrust_core::position::Game;
use macroquad::prelude::*;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[macroquad::main("Chrust")]
async fn main() {
	let mut game_state = GameState {
		game: Game::try_from_fen(DEFAULT_FEN).expect("default FEN must be valid"),
		assets: None,
		selected: None,
		legal_moves: Vec::new(),
		ui_state: None,
	};

	loop {
		clear_background(DARKGRAY);

		egui_macroquad::ui(|egui_ctx| {
			if game_state.assets.is_none() {
				game_state.assets = Some(load_assets(egui_ctx));
			}

			egui::SidePanel::right("sidebar").min_width(140.0).show(egui_ctx, |ui| {
				ui.heading("Chrust");
				ui.separator();

				let side_label = format!("{} to move", game_state.game.position.side_to_move);
				ui.label(side_label);

				let status_label = match &game_state.game.game_status {
					GameStatus::Playing => "Playing".to_string(),
					GameStatus::InCheck => "In check!".to_string(),
					GameStatus::CheckmateForSide(s) => format!("{} wins!", s),
					GameStatus::Stalemate => "Stalemate".to_string(),
					GameStatus::DrawByAgreement => "Draw".to_string(),
					GameStatus::DrawByFiftyMoves => "Draw (50-move rule)".to_string(),
					GameStatus::DrawByRepetition => "Draw (repetition)".to_string(),
					GameStatus::DrawByInsufficientMaterial => "Draw (material)".to_string(),
				};
				ui.label(status_label);

				ui.separator();
				if ui.button("Reset").clicked() {
					apply_ui_event(&mut game_state, UiEvent::ClickResetButton);
				}
			});

			render_board(egui_ctx, &mut game_state);
		});

		egui_macroquad::draw();
		next_frame().await;
	}
}
