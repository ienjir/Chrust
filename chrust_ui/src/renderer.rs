use crate::{
	controller::{apply_ui_event, UiEvent},
	helper::position_to_square,
	layout::{BOARD_BLACK_COLOR, BOARD_HIGHLIGHTED_COLOR, BOARD_WHITE_COLOR, PROMOTION_PIECES},
	state::{GameState, Overlay},
};
use chrust_core::helper::square;
use egui::{Color32, Context, Pos2, Rect, Vec2};

const LEGAL_MOVE_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 77, 33, 160);

pub(crate) fn render_board(egui_ctx: &Context, game_state: &mut GameState) {
	egui::CentralPanel::default().show(egui_ctx, |ui| {
		let board_rect = ui.available_rect_before_wrap();
		let board_size = board_rect.width().min(board_rect.height());
		let sq = board_size / 8.0;

		let response = ui.allocate_rect(
			Rect::from_min_size(board_rect.min, Vec2::splat(board_size)),
			egui::Sense::click(),
		);

		let painter = ui.painter();

		for rank in 0..8u8 {
			for file in 0..8u8 {
				let sq_idx = square(file, rank);
				let x = board_rect.min.x + file as f32 * sq;
				let y = board_rect.min.y + (7 - rank) as f32 * sq;
				let sq_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(sq));

				let is_selected = Some(sq_idx) == game_state.selected;
				let is_legal = game_state.legal_moves.iter().any(|m| m.to_square == sq_idx);

				let base = if (rank + file) % 2 == 1 { BOARD_WHITE_COLOR } else { BOARD_BLACK_COLOR };
				let bg = if is_selected { BOARD_HIGHLIGHTED_COLOR } else { base };

				painter.rect_filled(sq_rect, 0.0, bg);

				if let Some(assets) = &game_state.assets {
					if let Some(piece) = game_state.game.position.board[sq_idx as usize] {
						if let Some(texture) = assets.pieces.get(&(piece.side, piece.piece)) {
							painter.image(
								texture.id(),
								sq_rect,
								Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
								Color32::WHITE,
							);
						}
					}
				}

				if is_legal {
					painter.circle_filled(sq_rect.center(), sq * 0.18, LEGAL_MOVE_COLOR);
				}
			}
		}

		let promotion_rects: Option<[Rect; 4]> = if let Some(Overlay::Promotion { .. }) = &game_state.ui_state {
			let overlay_w = sq * 4.0;
			let overlay_x = board_rect.min.x + (board_size - overlay_w) / 2.0;
			let overlay_y = board_rect.min.y + (board_size - sq) / 2.0;

			let bg = Rect::from_min_size(Pos2::new(overlay_x, overlay_y), Vec2::new(overlay_w, sq));
			painter.rect_filled(bg, 4.0, Color32::from_rgb(40, 40, 40));

			let rects: [Rect; 4] = std::array::from_fn(|i| {
				Rect::from_min_size(
					Pos2::new(overlay_x + i as f32 * sq, overlay_y),
					Vec2::splat(sq),
				)
			});

			let promoting_side = game_state.game.position.side_to_move;
			if let Some(assets) = &game_state.assets {
				for (i, &piece) in PROMOTION_PIECES.iter().enumerate() {
					if let Some(texture) = assets.pieces.get(&(promoting_side, piece)) {
						painter.image(
							texture.id(),
							rects[i],
							Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
							Color32::WHITE,
						);
					}
				}
			}

			Some(rects)
		} else {
			None
		};

		if response.clicked() {
			if let Some(pos) = response.interact_pointer_pos() {
				let event = if let Some(rects) = &promotion_rects {
					rects.iter().enumerate().find_map(|(i, r)| {
						r.contains(pos).then_some(UiEvent::ClickPromotionSquare(PROMOTION_PIECES[i]))
					})
				} else {
					let (file, rank) = position_to_square(pos, board_rect, sq);
					if (0.0..8.0).contains(&file) && (0.0..8.0).contains(&rank) {
						Some(UiEvent::ClickSquare(square(file as u8, rank as u8)))
					} else {
						None
					}
				};

				if let Some(ev) = event {
					apply_ui_event(game_state, ev);
				}
			}
		}
	});
}
