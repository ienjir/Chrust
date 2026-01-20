use chrust_core_simple::square;
use macroquad::shapes::draw_rectangle;
use crate::{layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y}, state::GameState};

pub fn render_chessboard_without_pieces(game_state: &GameState) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let x = GRID_ORIGIN_X + file as f32 * CELL_SIZE;
            let y = GRID_ORIGIN_Y + (7 - rank) as f32 * CELL_SIZE;

            let mut color = macroquad::color::colors::BLACK; 
            if (rank + file) % 2 == 0 {
                color = macroquad::color::colors::WHITE;
            };

            if let Some(highlighted) = game_state.highlighted {
                let square_num = square(file, rank);
                if highlighted == square_num {
                    color = macroquad::color::colors::RED;
                }
            };

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
        }
    }
}
