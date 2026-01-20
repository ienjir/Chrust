use chrust_core_simple::{Square, position::{Position, load_position_from_fen}, square};
use macroquad::prelude::*;

const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const CELL_SIZE: f32 = 80.0;
const GRID_ORIGIN_X: f32 = 100.0;
const GRID_ORIGIN_Y: f32 = 100.0;
const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);

struct GameState {
    position: Position,
    highlighted: Option<Square>,
}

struct InputState {
    mouse_x: f32,
    mouse_y: f32,
    left_mouse_clicked: bool,
}

struct UiState {
    // Currently not relevant 
}

enum UiEvent {
    ClickSquare(Square),
    ClickResetButton, 
}


#[macroquad::main("Chrust")]
async fn main() {
    let default_position = match load_position_from_fen(DEFAULT_FEN_STRING) {
        Ok(x) => x,
        Err(_x) => panic!("Paniced while loading default position"),
    };

    let mut game_state = GameState {
        position: default_position,
        highlighted: None,
    };

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let input_state = InputState {
            mouse_x: mouse_x,
            mouse_y: mouse_y,
            left_mouse_clicked: is_mouse_button_pressed(MouseButton::Left),
        };

        if let Some(ui_event) = route_click(&input_state) {
            apply_ui_event(&mut game_state, ui_event);
        };

        render_chessboard_without_pieces(&game_state);

        next_frame().await
    }
}

fn apply_ui_event(game_state: &mut GameState, ui_event: UiEvent) {
    match ui_event {
        UiEvent::ClickSquare(square) => {
            game_state.highlighted = Some(square);
        }
        UiEvent::ClickResetButton => {
            println!("Reset board (currently not implemented)");
        }
    }
}

fn route_click(input: &InputState) -> Option<UiEvent>{
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

fn get_chessboard_square(input: &InputState) -> Option<Square> {
    let relative_x = input.mouse_x - GRID_ORIGIN_X;
    let relative_y = input.mouse_y - GRID_ORIGIN_Y;

    let rank = (8.0 - relative_y / CELL_SIZE).floor();
    let file = (relative_x / CELL_SIZE).floor();
    let square = square(file as u8, rank as u8);

    Some(square)
}

fn mouse_over_rect(input: &InputState, rect: Rect) -> bool {
    input.mouse_x >= rect.x
        && input.mouse_x <= rect.x + rect.w
        && input.mouse_y >= rect.y
        && input.mouse_y <= rect.y + rect.h
}

fn render_chessboard_without_pieces(game_state: &GameState) {
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
