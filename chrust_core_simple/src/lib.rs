pub enum Side { White, Black }

pub enum Piece {
    King,
    Queen,
    Rook,
    Bisoph,
    Knight,
    Pawn,
}

pub struct ColoredPiece {
    pub piece: Piece,
    pub side: Side,
}

pub type Square = u8;

pub enum CastleRigth { WK, WQ, BK, BQ }

pub struct Position {
    pub board: [Option<ColoredPiece>; 64],
    pub side_to_move: Side,
    pub castle: [bool; 4],
    pub en_passent: Option<Square>,
}

pub fn file(square: Square) -> u8 {
    square % 8
}

pub fn rank(square: Square) -> u8 {
    square / 8
}

pub fn square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

