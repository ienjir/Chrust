use chrust_core_simple::{Piece, Side};
use macroquad::prelude::*;

use crate::state::UiError;

pub struct PieceTextures {
    tex: [[Texture2D; 6]; 2],
}

pub async fn load_chess_piece(color: &str, piece: &str) -> Result<Texture2D, UiError> {
    let path = format!("{}/{}", color, piece);

    load_texture(&path)
        .await
        .map_err(|e| UiError::CouldNotLoadTexture { path, source: e })
}

impl PieceTextures {
    pub async fn load() -> Result<Self, UiError> {
        async fn lt(path: String) -> Result<Texture2D, UiError> {
            load_texture(&path)
                .await
                .map_err(|e| UiError::CouldNotLoadTexture { path, source: e })
        }

        let white = [
            lt("assets/white/king.png".to_string()).await?,
            lt("assets/white/queen.png".to_string()).await?,
            lt("assets/white/rook.png".to_string()).await?,
            lt("assets/white/bishop.png".to_string()).await?,
            lt("assets/white/knight.png".to_string()).await?,
            lt("assets/white/pawn.png".to_string()).await?,
        ];

        let black = [
            lt("assets/black/king.png".to_string()).await?,
            lt("assets/black/queen.png".to_string()).await?,
            lt("assets/black/rook.png".to_string()).await?,
            lt("assets/black/bishop.png".to_string()).await?,
            lt("assets/black/knight.png".to_string()).await?,
            lt("assets/black/pawn.png".to_string()).await?,
        ];

        Ok(Self { tex: [white, black] })
    }

    #[inline]
    pub async fn get(&self, side: Side, kind: Piece) -> Texture2D {
        let s = match side { Side::White => 0, Side::Black => 1 };
        let k = match kind {
            Piece::King => 0,
            Piece::Queen => 1,
            Piece::Rook => 2,
            Piece::Bishop => 3,
            Piece::Knight => 4,
            Piece::Pawn => 5,
        };
        self.tex[s][k].clone()
    }
}
