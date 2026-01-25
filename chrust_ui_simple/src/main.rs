mod assets;
mod renderer;
mod state;
mod layout;
mod input;
mod controller;
use chrust_core_simple::{position::load_position_from_fen};
use crate::{assets::load_assets, controller::apply_ui_event, input::route_click, layout::TEST_FEN_STRING, renderer::{render_chess_pieces, render_chessboard_without_pieces, render_possible_moves}};
use macroquad::prelude::*;
use macroquad::file::set_pc_assets_folder;
use crate::state::{GameState, InputState};

#[macroquad::main("Chrust")]
async fn main() {
    set_pc_assets_folder("chrust_ui_simple/assets");
    let assets = load_assets().await;

    let default_position = match load_position_from_fen(TEST_FEN_STRING) {
        Ok(x) => x,
        Err(_x) => panic!("Paniced while loading default position"),
    };

    let test = default_position.pawn_targets(8 as u8);

    let test2: Vec<u8> = match test {
        Ok(x) => x,
        Err(_x) => {
            println!("Error");
            return;
        } 
    };
    for test in test2 {
        println!("Squre: {test}");
    };

    let mut game_state = GameState {
        position: default_position,
        assets: assets,
        selected: None,
        possible_moves: Vec::new(),
    };

    loop {
        clear_background(LIGHTGRAY);

        // Debug
        if let Some(selected) = game_state.selected {
            println!("Selected square: {selected}");
        }
        let test = &game_state.possible_moves;
        for test1 in test {
            println!("Square: {test1}") 
        }



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
        render_chess_pieces(&game_state).await;
        render_possible_moves(&game_state);

        next_frame().await;
    }
}
