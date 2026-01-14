use chrust_core_simple::file;
use chrust_core_simple::Square;
use chrust_core_simple::rank;
use chrust_core_simple::square;

fn main() {
   let test_square: Square = 63;
   let file = file(test_square);
   let rank = rank(test_square);
   println!("File: {file}, Rank: {rank}");

   let square = square(7, 7);
   println!("Square: {square}")
}
