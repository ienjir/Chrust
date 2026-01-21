use std::collections::HashMap;
use chrust_core_simple::{Piece, Side};
use macroquad::texture::{FilterMode, Texture2D, load_texture};

pub struct Assets {
    pub pieces: HashMap<(Side, Piece), Texture2D>,
}

pub async fn load_assets() -> Assets {
    let mut pieces = HashMap::new();

    let defs = [
        (Side::White, Piece::King,   "w_king.png"),
        (Side::White, Piece::Queen,  "w_queen.png"),
        (Side::White, Piece::Rook,   "w_rook.png"),
        (Side::White, Piece::Bishop, "w_bishop.png"),
        (Side::White, Piece::Knight, "w_knight.png"),
        (Side::White, Piece::Pawn,   "w_pawn.png"),
        (Side::Black, Piece::King,   "b_king.png"),
        (Side::Black, Piece::Queen,  "b_queen.png"),
        (Side::Black, Piece::Rook,   "b_rook.png"),
        (Side::Black, Piece::Bishop, "b_bishop.png"),
        (Side::Black, Piece::Knight, "b_knight.png"),
        (Side::Black, Piece::Pawn,   "b_pawn.png"),
    ];

    for (side, kind, path) in defs {
        let tex = load_texture(path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        pieces.insert((side, kind), tex);
    }

    Assets { pieces }
}
