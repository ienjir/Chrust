use core::f32;
use chrust_core_simple::{Square, file, rank};
use macroquad::{color::{GREEN, WHITE}, math::{Rect, Vec2}, shapes::{draw_circle, draw_rectangle}, texture::{DrawTextureParams, draw_texture_ex}};
use crate::{layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y}, state::{GameState}};

pub fn get_square_coordinates(square: Square) -> (f32, f32) {
    let file = file(square);
    let rank = rank(square);

    let x: f32 = GRID_ORIGIN_X + file as f32 * CELL_SIZE;
    let y: f32 = (GRID_ORIGIN_Y + (7.0 * CELL_SIZE)) - rank as f32 * CELL_SIZE;

    (x,y)
}

pub fn get_square_rectangle(square: Square) -> Rect {
    let file = file(square);
    let rank = rank(square);

    let x: f32 = GRID_ORIGIN_X + file as f32 * CELL_SIZE;
    let y: f32 = (GRID_ORIGIN_Y + (7.0 * CELL_SIZE)) - rank as f32 * CELL_SIZE;

    let rect = Rect {
        x: x,
        y: y,
        h: CELL_SIZE,
        w: CELL_SIZE,
    };

    rect
}

pub fn render_chessboard_without_pieces(game_state: &GameState) {
    for square in 0..64 {
        let (x, y) = get_square_coordinates(square as u8);

        let file = file(square);
        let rank = rank(square);

        let mut color = macroquad::color::colors::WHITE; 
        if (rank + file) % 2 == 0 {
            color = macroquad::color::colors::DARKGRAY;
        };

        if let Some(highlighted) = game_state.selected {
            if highlighted == square {
                color = macroquad::color::colors::RED;
            }
        };

        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
    }
}

pub async fn render_chess_pieces(game_state: &GameState) {
    for square in 0..64 {
        let rect = get_square_rectangle(square);

        let Some(piece) = game_state.position.board[square as usize] else { continue };
        let texture = game_state.assets.pieces.get(&(piece.side, piece.piece)).expect("missing texture");

        draw_texture_ex(&texture, rect.x, rect.y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(rect.w, rect.h)),
            ..Default::default()
        });
    }
}

pub fn render_possible_moves(game_state: &GameState) {
    for mv in game_state.possible_moves.iter() {
        let rect = get_square_rectangle(mv.to_square);
        let rect_center = rect.center();

        draw_circle(rect_center.x, rect_center.y, 20.0, GREEN);
    }
}
