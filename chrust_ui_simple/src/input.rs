use std::usize;

use chrust_core_simple::{Piece, Square, square};
use macroquad::{math::i32, prelude::Rect};
use crate::{controller::UiEvent, layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y, PROMOTION_LEFT_CELLS, PROMOTION_PIECES, PROMOTION_TOP_CELLS, RESET_BUTTON}, state::{GameState, InputState, UiState}};

pub fn mouse_over_rect(input: &InputState, rect: Rect) -> bool {
    input.mouse_x >= rect.x
        && input.mouse_x <= rect.x + rect.w
        && input.mouse_y >= rect.y
        && input.mouse_y <= rect.y + rect.h
}

pub fn route_click(input: &InputState, game_state: &GameState) -> Option<UiEvent>{
    if !input.left_mouse_clicked {
        return None;
    }

    if mouse_over_rect(input, RESET_BUTTON) {
        return Some(UiEvent::ClickResetButton);
    }

    if matches!(game_state.ui_state, Some(UiState::PROMOTION { .. })) {
        let promotion_ui = Rect::new(GRID_ORIGIN_X + (CELL_SIZE * PROMOTION_LEFT_CELLS), GRID_ORIGIN_Y + (CELL_SIZE * PROMOTION_TOP_CELLS), CELL_SIZE * 4.0, CELL_SIZE);
        if mouse_over_rect(input, promotion_ui) {
            let piece = get_promotion_square(input);
            return Some(UiEvent::ClickPromotionSquare(piece));
        }
    }

    let chess_grid = Rect::new(GRID_ORIGIN_X, GRID_ORIGIN_Y, 8.0 * CELL_SIZE, 8.0 * CELL_SIZE);
    if mouse_over_rect(input, chess_grid) {
        if let Some(square) = get_chessboard_square(input) {
            return Some(UiEvent::ClickSquare(square));
        }
    }

    None
}

pub fn get_chessboard_square(input: &InputState) -> Option<Square> {
    let relative_x = input.mouse_x - GRID_ORIGIN_X;
    let relative_y = input.mouse_y - GRID_ORIGIN_Y;

    let rank = (8.0 - relative_y / CELL_SIZE).floor();
    let file = (relative_x / CELL_SIZE).floor();
    let square = square(file as u8, rank as u8);

    Some(square)
}

pub fn get_promotion_square(input: &InputState) -> Piece {
    let relative_x = input.mouse_x - (GRID_ORIGIN_X + (CELL_SIZE * PROMOTION_LEFT_CELLS));

    let rank = (relative_x / CELL_SIZE).floor() as i32;

    if !(0..=3).contains(&rank) {
        return Piece::Pawn;
    }

    let piece = PROMOTION_PIECES[rank as usize];
    piece
}
