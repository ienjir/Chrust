use core::f32;
use chrust_core_simple::{Square, file, rank};
use macroquad::{color::{Color, GREEN, WHITE}, math::{Rect, Vec2}, shapes::{draw_circle, draw_rectangle}, texture::{DrawTextureParams, draw_texture_ex}, window::{screen_height, screen_width}};
use crate::{layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y, PROMOTION_LEFT_CELLS, PROMOTION_PIECES, PROMOTION_TOP_CELLS}, state::GameState};

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

        draw_circle(rect_center.x, rect_center.y, CELL_SIZE / 4.0, GREEN);
    }
}

pub fn render_dark_background() {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.51, 0.51, 0.51, 0.9));
}

pub fn render_promotion_modal(game_state: &GameState) {
    render_dark_background();

    let relative_from_x = GRID_ORIGIN_X + (CELL_SIZE * PROMOTION_LEFT_CELLS);
    let relative_from_y = GRID_ORIGIN_Y + (CELL_SIZE * PROMOTION_TOP_CELLS);

    for piece_index in 0..=3 {
        let mut color = macroquad::color::colors::DARKGRAY;
        if piece_index % 2 == 0 {
            color = macroquad::color::colors::WHITE;
        }

        let rect = Rect {
            x: relative_from_x + (piece_index as f32 * CELL_SIZE),
            y: relative_from_y,
            w: CELL_SIZE,
            h: CELL_SIZE,
        };

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

        let piece = PROMOTION_PIECES[piece_index];
        let texture = game_state.assets.pieces.get(&(game_state.position.side_to_move, piece)).expect("missing texture");

        draw_texture_ex(&texture, rect.x, rect.y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(rect.w, rect.h)),
            ..Default::default()
        });
    }
}
