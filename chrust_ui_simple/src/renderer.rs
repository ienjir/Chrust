use core::f32;
use chrust_core_simple::{Piece, Side, Square, file, rank};
use macroquad::{color::WHITE, math::{Rect, Vec2}, shapes::draw_rectangle, texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture}};
use crate::{assets::load_chess_piece, layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y}, state::{GameState, UiError}};

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

        if let Some(highlighted) = game_state.highlighted {
            if highlighted == square {
                println!("Square: {square}");
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

        let piece_str = match piece.piece {
            Piece::King => "king.png",
            Piece::Rook => "rook.png",
            Piece::Pawn => "pawn.png",
            Piece::Queen => "queen.png",
            Piece::Bishop => "bishop.png",
            Piece::Knight => "knight.png",
        };

        let color_str = match piece.side {
            Side::White => "white_pieces",
            Side::Black => "black_pieces",
        };

        match load_chess_piece(color_str, piece_str).await {
            Ok(texture) => {
                draw_texture_ex(&texture, rect.x, rect.y, WHITE, DrawTextureParams {
                    dest_size: Some(Vec2::new(rect.w, rect.h)),
                    ..Default::default()
                });
            }
            Err(e) => {
                eprintln!("texture missing for {}/{}: {:?}", color_str, piece_str, e);
            }
        }
    };
}
