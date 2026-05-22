use chrust_core::{Piece, Side};
use egui::{
	ColorImage,
	ahash::{HashMap, HashMapExt},
};

pub struct Assets {
	pub pieces: HashMap<(Side, Piece), egui::TextureHandle>,
}

pub fn load_assets(ctx: &egui::Context) -> Assets {
	let mut pieces = HashMap::new();
	let defs: &[(Side, Piece, &[u8])] = &[
		(Side::White, Piece::King, include_bytes!("../assets/w_king.png")),
		(Side::White, Piece::Queen, include_bytes!("../assets/w_queen.png")),
		(Side::White, Piece::Rook, include_bytes!("../assets/w_rook.png")),
		(Side::White, Piece::Bishop, include_bytes!("../assets/w_bishop.png")),
		(Side::White, Piece::Knight, include_bytes!("../assets/w_knight.png")),
		(Side::White, Piece::Pawn, include_bytes!("../assets/w_pawn.png")),
		(Side::Black, Piece::King, include_bytes!("../assets/b_king.png")),
		(Side::Black, Piece::Queen, include_bytes!("../assets/b_queen.png")),
		(Side::Black, Piece::Rook, include_bytes!("../assets/b_rook.png")),
		(Side::Black, Piece::Bishop, include_bytes!("../assets/b_bishop.png")),
		(Side::Black, Piece::Knight, include_bytes!("../assets/b_knight.png")),
		(Side::Black, Piece::Pawn, include_bytes!("../assets/b_pawn.png")),
	];
	for &(side, kind, bytes) in defs {
		let image = image::load_from_memory(bytes).unwrap().to_rgba8();
		let size = [image.width() as usize, image.height() as usize];
		let pixels = image.as_flat_samples();
		let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
		let handle = ctx.load_texture(format!("{:?}{:?}", side, kind), color_image, egui::TextureOptions::LINEAR);
		pieces.insert((side, kind), handle);
	}
	Assets { pieces }
}
