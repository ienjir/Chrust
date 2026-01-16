use std::{usize};

pub mod move_gen;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side { White, Black }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum FenError {
    InvalidPieceChar(char),
}

pub fn load_position_from_fen(fen: String) -> Result<Position, FenError> {
    let mut position = Position {
        board: [None; 64],
        castle: [false; 4],
        en_passent: None,
        side_to_move: Side::White,
    };

    let fen_parts: Vec<&str> = fen.split_whitespace().collect();

    let fen_board_normal = fen_parts[0];
    let fen_ranks = fen_board_normal.split("/");

    let mut current_rank = 7;
    for rank_str in fen_ranks {
        let mut file = 0;

        for c in rank_str.chars() {
            if let Some(digit) = c.to_digit(10) {
                file += digit as u8;
            } else {
                let piece_side = if c.is_uppercase() { Side::White } else { Side::Black };
                let piece_type = match c.to_ascii_lowercase() {
                    'k' => Piece::King,
                    'p' => Piece::Pawn,
                    'n' => Piece::Knight,
                    'b' => Piece::Bishop,
                    'r' => Piece::Rook,
                    'q' => Piece::Queen,
                    _   => return Err(FenError::InvalidPieceChar(c)),
                };

                let piece = ColoredPiece {
                    piece: piece_type,
                    side: piece_side,
                };

                let target_square = square(file, current_rank);
                position.board[target_square as usize] = Some(piece);

                file += 1; 
            }
        }

        if current_rank > 0 {
            current_rank -= 1;
        }
    }

    Ok(position)
}
