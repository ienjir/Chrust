use chrust_core_simple::{Square, position::Position};
use crate::assets::Assets;

pub struct GameState {
    pub position: Position,
    pub assets: Assets, 
    pub highlighted: Option<Square>,
}

pub struct InputState {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub left_mouse_clicked: bool,
}

pub struct UiState {
    // Currently not relevant 
}

#[derive(Debug)]
pub enum UiError {
    CouldNotLoadTexture {
        path: String,
        source: macroquad::Error,
    },
}
