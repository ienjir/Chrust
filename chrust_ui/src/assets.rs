use chrust_core::{ColoredPiece, Piece, Side};
use egui::{ColorImage, TextureHandle, ahash::{HashMap, HashMapExt}};

pub struct Assets {
    pub pieces: HashMap<(Side, Piece), egui::TextureHandle>,
}

pub fn load_assets(ctx: &egui::Context) -> Assets {
    let mut pieces = HashMap::new();
    let defs = [
		(Side::White, Piece::King, include_bytes!("../assets/w_king.png") as &[u8]),
		(Side::White, Piece::Queen, include_bytes!("../assets/w_queen.png") as &[u8]),
		(Side::White, Piece::King, include_bytes!("../assets/w_king.png") as &[u8]),
		(Side::White, Piece::Queen, include_bytes!("../assets/w_queen.png") as &[u8]),
		(Side::White, Piece::Rook, include_bytes!("../assets/w_rook.png") as &[u8]),
		(Side::White, Piece::Bishop, include_bytes!("../assets/w_bishop.png") as &[u8]),
		(Side::White, Piece::Knight, include_bytes!("../assets/w_knight.png") as &[u8]),
		(Side::White, Piece::Pawn, include_bytes!("../assets/w_pawn.png") as &[u8]),
		(Side::Black, Piece::King, include_bytes!("../assets/b_king.png") as &[u8]),
		(Side::Black, Piece::Queen, include_bytes!("../assets/b_queen.png") as &[u8]),
		(Side::Black, Piece::Rook, include_bytes!("../assets/b_rook.png") as &[u8]),
		(Side::Black, Piece::Bishop, include_bytes!("../assets/b_bishop.png") as &[u8]),
		(Side::Black, Piece::Knight, include_bytes!("../assets/b_knight.png") as &[u8]),
		(Side::Black, Piece::Pawn, include_bytes!("../assets/b_pawn.png") as &[u8]),
	];
	for (side, kind, bytes) in defs {
		let image = image::load_from_memory(bytes).unwrap().to_rgba8();
		let size = [image.width() as usize, image.height() as usize];
		let pixels = image.as_flat_samples();
		let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
		let handle = ctx.load_texture(
			format!("{:?}{:?}", side, kind),
			color_image,
			egui::TextureOptions::NEAREST, 
		);
		pieces.insert((side, kind), handle);
	}
	Assets { pieces }
}

pub(crate) fn get_texture(colored_piece: ColoredPiece) -> TextureHandle {

}
