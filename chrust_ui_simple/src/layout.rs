use chrust_core_simple::Piece;
use macroquad::math::Rect;

// pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_FEN_STRING: &str = "8/8/8/8/1p6/8/P7/8 w - d3 0 1";

pub const CELL_SIZE: f32 = 80.0;
pub const GRID_ORIGIN_X: f32 = 100.0;
pub const GRID_ORIGIN_Y: f32 = 100.0;

pub const PROMOTION_LEFT_CELLS: f32 = 2.0;
pub const PROMOTION_TOP_CELLS: f32 = 3.5;
pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

pub const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);
