use chrust_core_simple::{Square, move_gen::knight_targets};

fn main() {
   let test_square: Square = 36;
    
   let knight_squares = knight_targets(test_square);

   for knight_square in knight_squares {
        println!("Square: {knight_square}")
   }
    
}
