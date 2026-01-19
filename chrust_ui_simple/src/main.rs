use chrust_core_simple::{Square, position::{Position, load_position_from_fen}, square};
use macroquad::prelude::*;

#[macroquad::main("Chrust")]
async fn main() {
    let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let position = load_position_from_fen(default_fen.to_string()); 
    let pos: Position = match position {
        Ok(x) => x,
        Err(_x) => return println!("Error"),
    };

    loop {
        clear_background(DARKGRAY);
        draw_chessboard().await; 
        process_click();
        next_frame().await
    }
}

const CELL_SIZE: f32 = 80.0;
const GRID_ORIGIN_X: f32 = 100.0;
const GRID_ORIGIN_Y: f32 = 100.0;

async fn draw_chessboard() {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let x = GRID_ORIGIN_X + file as f32 * CELL_SIZE;
            let y = GRID_ORIGIN_Y + rank as f32 * CELL_SIZE;

            let mut color = macroquad::color::colors::BLACK; 
            if (rank + file) % 2 == 0 {
                color = macroquad::color::colors::WHITE;
            };

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
        }
    }
}

fn process_click() {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_position = Rect {
            x: mouse_x,
            y: mouse_y,
            h: 1.0,
            w: 1.0,
        };
         
        let chess_board = Rect{
            x: GRID_ORIGIN_X,
            y: GRID_ORIGIN_Y,
            w: (8.0 * CELL_SIZE),
            h: (8.0 * CELL_SIZE),
        };

        if chess_board.overlaps(&mouse_position) {
            get_chessboard_square(&mouse_position);
        }
    }
}

fn get_chessboard_square(mouse_position: &Rect) -> (Square, Rect) {
    let relative_x = mouse_position.x - GRID_ORIGIN_X;
    let relative_y = mouse_position.y - GRID_ORIGIN_Y;

    let rank = (8.0 - relative_y / CELL_SIZE).floor();
    let rank_cooridnate = GRID_ORIGIN_Y + (relative_y / CELL_SIZE).floor() * CELL_SIZE;

    let file = (relative_x / CELL_SIZE).floor();
    let file_cooridnate = GRID_ORIGIN_X + file * CELL_SIZE;

    let square = square(file as u8, rank as u8);
    
    let board_square = Rect {
        x: rank_cooridnate,
        y: file_cooridnate,
        w: CELL_SIZE,
        h: CELL_SIZE,
    };

    (square, board_square)
}
