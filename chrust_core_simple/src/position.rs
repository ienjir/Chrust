use crate::{ColoredPiece, Piece, Side, Square, errors::FenError, square};

#[derive(Clone, Debug)]
pub struct Position {
    pub board: [Option<ColoredPiece>; 64],
    pub side_to_move: Side,
    pub castle: [bool; 4],
    pub en_passant: Option<Square>,
}


pub fn convert_square_string_to_square(square_string: &str) -> Result<u8, FenError> {
    if square_string.len() != 2 {
        return Err(FenError::SquareLenghtIsnt2Wide(square_string.len()));
    }

    let chars: Vec<char> = square_string.to_lowercase().chars().collect();

    let file = (chars[0] as u8).wrapping_sub(b'a');
    if file > 7 {
        return Err(FenError::InvalidFile(chars[0]));
    }

    let rank = chars[1].to_digit(10)
        .map(|d| d as u8)
        .and_then(|d| d.checked_sub(1)) 
        .filter(|&d| d < 8)            
        .ok_or(FenError::InvalidRank(chars[1]))?;

    let square_index = rank * 8 + file;

    if square_index > 63 {
        return Err(FenError::OutOfBounds(square_index));
    }

    Ok(square_index)
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


pub fn load_position_from_fen(fen: &str) -> Result<Position, FenError> {
    let mut position = Position {
        board: [None; 64],
        castle: [false; 4],
        en_passant: None,
        side_to_move: Side::White,
    };

    let fen_parts: Vec<&str> = fen.split_whitespace().collect();

    if fen_parts.len() != 6 {
       return Err(FenError::MissingFenParts); 
    }

    let en_passant = fen_parts[3];
    if en_passant != "-" {
        let square = convert_square_string_to_square(en_passant);
        match square {
            Ok(x) => {position.en_passant = Some(x)},
            Err(x) => return Err(x),
        }
    }

    let test = fen_parts[1];
    match test {
        "b" => {position.side_to_move = Side::Black},
        "w" => {position.side_to_move = Side::White},
        _ => return Err(FenError::NotAValideSide) 
    };

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
