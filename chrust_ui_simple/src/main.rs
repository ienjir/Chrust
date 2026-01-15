use chrust_core_simple::{Square, move_gen::{knight_targets, rook_targets}};

fn main() {
   let test_square: Square = 63;
    
   let rook_squares = rook_targets(test_square);

   for rook_square in rook_squares {
        println!("Square: {rook_square}")
   }
    
}
