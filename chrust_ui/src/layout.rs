use chrust_core::Piece;
use egui::Color32;
use macroquad::math::Rect;

// pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_FEN_STRING: &str = "r3k2r/8/8/8/4p3/3P2p1/2P2P1P/R3K2R w KQ - 0 1";

pub const CELL_SIZE: f32 = 40.0;
pub const GRID_ORIGIN_X: f32 = 100.0;
pub const GRID_ORIGIN_Y: f32 = 100.0;

pub const PROMOTION_LEFT_CELLS: f32 = 2.0;
pub const PROMOTION_TOP_CELLS: f32 = 3.5;
pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

pub const RESET_BUTTON: Rect = Rect::new(100.0, 20.0, 140.0, 40.0);

pub(crate) const BOARD_BLACK_COLOR: Color32 = Color32::from_rgb(60, 60, 60);
pub(crate) const BOARD_WHITE_COLOR: Color32 = Color32::from_rgb(140, 90, 210);
pub(crate) const BOARD_HIGHLIGHTED_COLOR: Color32 = Color32::from_rgb(80, 210, 250);
