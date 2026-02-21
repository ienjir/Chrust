use chrust_core::{Square, moves::make_move::Move, position::Position};
use crate::assets::Assets;

pub struct GameState {
    pub position: Position,
    pub assets: Assets, 
    pub selected: Option<Square>,
    pub possible_moves: Vec<Move>,
    pub ui_state: Option<UiState>,
}

pub struct InputState {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub left_mouse_clicked: bool,
}

pub enum UiState {
    PROMOTION { pending_move: Move }
}
