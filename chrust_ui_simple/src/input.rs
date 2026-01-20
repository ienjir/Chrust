use chrust_core_simple::{Square, square};
use macroquad::prelude::Rect;
use crate::{layout::{CELL_SIZE, GRID_ORIGIN_X, GRID_ORIGIN_Y, RESET_BUTTON}, state::InputState, controller::{UiEvent}};

pub fn get_chessboard_square(input: &InputState) -> Option<Square> {
    let relative_x = input.mouse_x - GRID_ORIGIN_X;
    let relative_y = input.mouse_y - GRID_ORIGIN_Y;

    let rank = (8.0 - relative_y / CELL_SIZE).floor();
    let file = (relative_x / CELL_SIZE).floor();
    let square = square(file as u8, rank as u8);

    Some(square)
}

pub fn mouse_over_rect(input: &InputState, rect: Rect) -> bool {
    input.mouse_x >= rect.x
        && input.mouse_x <= rect.x + rect.w
        && input.mouse_y >= rect.y
        && input.mouse_y <= rect.y + rect.h
}

pub fn route_click(input: &InputState) -> Option<UiEvent>{
    if !input.left_mouse_clicked {
        return None;
    }

    if mouse_over_rect(input, RESET_BUTTON) {
        return Some(UiEvent::ClickResetButton);
    }

    let chess_grid = Rect::new(GRID_ORIGIN_X, GRID_ORIGIN_Y, 8.0 * CELL_SIZE, 8.0 * CELL_SIZE);
    if mouse_over_rect(input, chess_grid) {
        if let Some(square) = get_chessboard_square(input) {
            return Some(UiEvent::ClickSquare(square));
        }
    }

    None
}
