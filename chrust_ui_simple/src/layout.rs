use chrust_core_simple::Piece;
use macroquad::math::Rect;

// pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_FEN_STRING: &str = "1q3P1P/6k1/5P1P/4b3/8/p1p5/1K6/2p5 w - - 0 1";

pub const CELL_SIZE: f32 = 40.0;
pub const GRID_ORIGIN_X: f32 = 100.0;
pub const GRID_ORIGIN_Y: f32 = 100.0;

pub const PROMOTION_LEFT_CELLS: f32 = 2.0;
pub const PROMOTION_TOP_CELLS: f32 = 3.5;
pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

pub const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);
