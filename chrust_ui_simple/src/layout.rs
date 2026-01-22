use macroquad::math::Rect;

pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const EMPTY_FEN_STRING: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
pub const TEST_FEN_STRING: &str = "6PK/8/8/4b3/8/8/8/8 w KQkq - 0 1";
pub const CELL_SIZE: f32 = 80.0;
pub const GRID_ORIGIN_X: f32 = 100.0;
pub const GRID_ORIGIN_Y: f32 = 100.0;
pub const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);
