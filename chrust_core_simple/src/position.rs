use crate::{ColoredPiece, Piece, square, Side, Square};

#[derive(Clone, Debug)]
pub struct Position {
    pub board: [Option<ColoredPiece>; 64],
    pub side_to_move: Side,
    pub castle: [bool; 4],
    pub en_passent: Option<Square>,
}

impl Position {
    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = square(file, rank);
                let test = self.board[square as usize];

                match test {
                    Some(piece) => {
                        print!(" {}", piece.to_char());
                    }
                    None => {
                        print!("  ")
                    }
                }
            }
            print!("\n");
        } 
    }
}

#[derive(Debug)]
pub enum FenError {
    InvalidPieceChar(char),
}

pub fn load_position_from_fen(fen: &str) -> Result<Position, FenError> {
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
