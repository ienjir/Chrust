use chrust_core_simple::position::{Position, load_position_from_fen};
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
        next_frame().await
    }
}

const GRID_SIZE: i32 = 8;
const CELL_SIZE: f32 = 80.0;
const GRID_ORIGIN_X: f32 = 100.0;
const GRID_ORIGIN_Y: f32 = 100.0;

async fn draw_chessboard() {
    for rank in 0..GRID_SIZE {
        for file in 0..GRID_SIZE {
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
