use chrust_core_simple::{Square, position::Position};

pub struct GameState {
    pub position: Position,
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

