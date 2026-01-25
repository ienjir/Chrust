use macroquad::math::Rect;

// pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_FEN_STRING: &str = "8/p7/1P6/p7/p7/1p6/P7/8 w KQkq - 0 1";
pub const CELL_SIZE: f32 = 80.0;
pub const GRID_ORIGIN_X: f32 = 100.0;
pub const GRID_ORIGIN_Y: f32 = 100.0;
pub const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);
