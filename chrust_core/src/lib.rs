pub mod moves;
pub mod position;
pub mod errors;
pub mod helper;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Side { White, Black }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColoredPiece {
    pub piece: Piece,
    pub side: Side,
}

pub type Square = u8;

pub enum CastleRigth { WK, WQ, BK, BQ }


pub fn file(square: Square) -> u8 {
    square % 8
}

pub fn rank(square: Square) -> u8 {
    square / 8
}

pub fn square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

impl ColoredPiece {
    pub fn to_char(&self) -> char {
        let mut piece_char = match self.piece {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };

        if self.side == Side::White {
            piece_char = piece_char.to_ascii_uppercase();
        }

        piece_char
    }
}

