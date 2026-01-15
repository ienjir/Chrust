use chrust_core_simple::{ColoredPiece, Piece, Position, Side, Square};

fn main() {
   let test_square: Square = 63;
   let mut test_board: [Option<ColoredPiece>; 64] = [None; 64];

   test_board[63] = Some(ColoredPiece {
       piece: Piece::Rook,
       side: Side::White,
   });


   let test_position = Position {
       board: test_board,
       castle: [false, false, false, false],
       side_to_move: Side::White,
       en_passent: None,
   };

   let rook_squares = test_position.rook_targets(test_square);

   for rook_square in rook_squares {
       println!("Square: {rook_square}")
   }
}
