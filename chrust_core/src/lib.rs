pub fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!()
    }
}

pub mod nr_of {
    pub const SQUARES: usize = 64;
    pub const SIDES: usize = 2;
    pub const PIECE_TYPES: usize = 7;
}

pub type Squares = usize; 

pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

pub enum Side {
    White = 0,
    Black = 1,
}

