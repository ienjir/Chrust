use chrust_core_simple::{ColoredPiece, position::load_position_from_fen, };

fn main() {
   let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

   let position = load_position_from_fen(default_fen.to_string()); 
   match position {
       Ok(pos) => {
            pos.print_board();
       }
       Err(_x) => println!("Erorr")
   }

   let test = ColoredPiece {
       piece: chrust_core_simple::Piece::Queen,
       side: chrust_core_simple::Side::Black,
   };
    
   let test2 = test.to_char();
   

    println!("Test: {test2}")
}

