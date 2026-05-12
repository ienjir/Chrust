# Chrust
## Chrust is a simple 2 player chess game and later (hopefully) a chess engine

## Next steps
- Add dragging
- Display side to move
- Captured pieces
- Check indicator

## Information
### Assets
The chess piece assets were not created by me. They were created by Uray M. János and are published on <a href="https://greenchess.net/info.php?item=downloads">greenchess.net</a>. The author retains the copyright. The images are licensed under the Creative Commons Attribution-ShareAlike 3.0 (CC BY-SA 3.0) license.

### Chessboard with intager positions
<table style="border-collapse: collapse; text-align: center;">
<tr>
<th style="border:1px solid black;width:40px;height:40px;">Y/X</th>
<th style="border:1px solid black;width:40px;">a</th>
<th style="border:1px solid black;width:40px;">b</th>
<th style="border:1px solid black;width:40px;">c</th>
<th style="border:1px solid black;width:40px;">d</th>
<th style="border:1px solid black;width:40px;">e</th>
<th style="border:1px solid black;width:40px;">f</th>
<th style="border:1px solid black;width:40px;">g</th>
<th style="border:1px solid black;width:40px;">h</th>
</tr>

<tr>
<th style="border:1px solid black;height:40px;">8</th>
<td style="border:1px solid black;">56</td>
<td style="border:1px solid black;">57</td>
<td style="border:1px solid black;">58</td>
<td style="border:1px solid black;">59</td>
<td style="border:1px solid black;">60</td>
<td style="border:1px solid black;">61</td>
<td style="border:1px solid black;">62</td>
<td style="border:1px solid black;">63</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">7</th>
<td style="border:1px solid black;">48</td>
<td style="border:1px solid black;">49</td>
<td style="border:1px solid black;">50</td>
<td style="border:1px solid black;">51</td>
<td style="border:1px solid black;">52</td>
<td style="border:1px solid black;">53</td>
<td style="border:1px solid black;">54</td>
<td style="border:1px solid black;">55</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">6</th>
<td style="border:1px solid black;">40</td>
<td style="border:1px solid black;">41</td>
<td style="border:1px solid black;">42</td>
<td style="border:1px solid black;">43</td>
<td style="border:1px solid black;">44</td>
<td style="border:1px solid black;">45</td>
<td style="border:1px solid black;">46</td>
<td style="border:1px solid black;">47</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">5</th>
<td style="border:1px solid black;">32</td>
<td style="border:1px solid black;">33</td>
<td style="border:1px solid black;">34</td>
<td style="border:1px solid black;">35</td>
<td style="border:1px solid black;">36</td>
<td style="border:1px solid black;">37</td>
<td style="border:1px solid black;">38</td>
<td style="border:1px solid black;">39</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">4</th>
<td style="border:1px solid black;">24</td>
<td style="border:1px solid black;">25</td>
<td style="border:1px solid black;">26</td>
<td style="border:1px solid black;">27</td>
<td style="border:1px solid black;">28</td>
<td style="border:1px solid black;">29</td>
<td style="border:1px solid black;">30</td>
<td style="border:1px solid black;">31</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">3</th>
<td style="border:1px solid black;">16</td>
<td style="border:1px solid black;">17</td>
<td style="border:1px solid black;">18</td>
<td style="border:1px solid black;">19</td>
<td style="border:1px solid black;">20</td>
<td style="border:1px solid black;">21</td>
<td style="border:1px solid black;">22</td>
<td style="border:1px solid black;">23</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">2</th>
<td style="border:1px solid black;">8</td>
<td style="border:1px solid black;">9</td>
<td style="border:1px solid black;">10</td>
<td style="border:1px solid black;">11</td>
<td style="border:1px solid black;">12</td>
<td style="border:1px solid black;">13</td>
<td style="border:1px solid black;">14</td>
<td style="border:1px solid black;">15</td>
</tr>
<tr>
<th style="border:1px solid black;height:40px;">1</th>
<td style="border:1px solid black;">0</td>
<td style="border:1px solid black;">1</td>
<td style="border:1px solid black;">2</td>
<td style="border:1px solid black;">3</td>
<td style="border:1px solid black;">4</td>
<td style="border:1px solid black;">5</td>
<td style="border:1px solid black;">6</td>
<td style="border:1px solid black;">7</td>
</tr>
</table>

### Addition/Substraction to get to a specific position
<table style="border-collapse: collapse; table-layout: fixed; text-align:center;">
<tr>
<td style="border:1px solid black;width:40px;height:40px;">+21</td>
<td style="border:1px solid black;width:40px;height:40px;">+22</td>
<td style="border:1px solid black;width:40px;height:40px;">+23</td>
<td style="border:1px solid black;width:40px;height:40px;">+24</td>
<td style="border:1px solid black;width:40px;height:40px;">+25</td>
<td style="border:1px solid black;width:40px;height:40px;">+26</td>
<td style="border:1px solid black;width:40px;height:40px;">+27</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">+13</td>
<td style="border:1px solid black;width:40px;height:40px;">+14</td>
<td style="border:1px solid black;width:40px;height:40px;">+15</td>
<td style="border:1px solid black;width:40px;height:40px;">+16</td>
<td style="border:1px solid black;width:40px;height:40px;">+17</td>
<td style="border:1px solid black;width:40px;height:40px;">+18</td>
<td style="border:1px solid black;width:40px;height:40px;">+19</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">+5</td>
<td style="border:1px solid black;width:40px;height:40px;">+6</td>
<td style="border:1px solid black;width:40px;height:40px;">+7</td>
<td style="border:1px solid black;width:40px;height:40px;">+8</td>
<td style="border:1px solid black;width:40px;height:40px;">+9</td>
<td style="border:1px solid black;width:40px;height:40px;">+10</td>
<td style="border:1px solid black;width:40px;height:40px;">+11</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">-3</td>
<td style="border:1px solid black;width:40px;height:40px;">-2</td>
<td style="border:1px solid black;width:40px;height:40px;">-1</td>
<td style="border:1px solid black;width:40px;height:40px;">X</td>
<td style="border:1px solid black;width:40px;height:40px;">+1</td>
<td style="border:1px solid black;width:40px;height:40px;">+2</td>
<td style="border:1px solid black;width:40px;height:40px;">+3</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">-11</td>
<td style="border:1px solid black;width:40px;height:40px;">-10</td>
<td style="border:1px solid black;width:40px;height:40px;">-9</td>
<td style="border:1px solid black;width:40px;height:40px;">-8</td>
<td style="border:1px solid black;width:40px;height:40px;">-7</td>
<td style="border:1px solid black;width:40px;height:40px;">-6</td>
<td style="border:1px solid black;width:40px;height:40px;">-5</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">-19</td>
<td style="border:1px solid black;width:40px;height:40px;">-18</td>
<td style="border:1px solid black;width:40px;height:40px;">-17</td>
<td style="border:1px solid black;width:40px;height:40px;">-16</td>
<td style="border:1px solid black;width:40px;height:40px;">-15</td>
<td style="border:1px solid black;width:40px;height:40px;">-14</td>
<td style="border:1px solid black;width:40px;height:40px;">-13</td>
</tr>

<tr>
<td style="border:1px solid black;width:40px;height:40px;">-27</td>
<td style="border:1px solid black;width:40px;height:40px;">-26</td>
<td style="border:1px solid black;width:40px;height:40px;">-25</td>
<td style="border:1px solid black;width:40px;height:40px;">-24</td>
<td style="border:1px solid black;width:40px;height:40px;">-23</td>
<td style="border:1px solid black;width:40px;height:40px;">-22</td>
<td style="border:1px solid black;width:40px;height:40px;">-21</td>
</tr>
</table>

## Small errors 
- Slider.rs: Find a way to not return a queen when a faulty piece is provided

## Important info
- king_squares[0] is white and [1] is black
- castling: 
- [0] => White kingside
- [1] => White queenside 
- [2] => Black kingside
- [3] => Black queenside

1. Create  chrust_core/src/attack_tables.rs
New file. Build two  [[u8; 8]; 64]  arrays (targets) and two  [u8; 64]  arrays (counts) — one pair for knights, one for
kings. Iterate all 64 squares, apply the 8 offsets for each piece type, filter out wrap-arounds and out-of-bounds using
the same file/rank diff logic that currently lives in  knight_targets  and  king_targets . Store the result in a  struct
AttackTables  with a  fn build() -> Self . Expose a  fn tables() -> &'static AttackTables  using  std::sync::OnceLock
or a  static  initialized at startup.

2. Register the module in  lib.rs
Add  pub mod attack_tables;  to  chrust_core/src/lib.rs .

3. Rewrite  knight_targets  in  knight.rs
Replace the offset loop,  in_bounds , and  file_diff / rank_diff  calls with a lookup into  tables().knight[from_square
as usize] . Iterate  0..tables().knight_count[from_square as usize] , read the target square, then just do the board
occupancy check ( None  → quiet,  Some  same side → skip,  Some  other side → capture). Delete all the arithmetic and
validation that was doing this at runtime.

4. Rewrite king normal moves in  king.rs
Same as above but for  king_targets . Replace the  directions  loop and  get_validated_candidate_square  +
get_file_and_rank_difference  calls with a lookup into  tables().king[from_square as usize] . The castling logic (
check_castling ) is separate and stays untouched.

5. Rewrite knight and king checks in  check.rs  ( is_square_attacked )
 is_square_attacked  currently has its own inline offset loops with the same bounds/wrap math duplicated for both
knights and kings. Replace both with table lookups — iterate the table entries, check if  board[sq]  holds the attacking
piece type and side. This is probably the biggest win since  is_square_attacked  is called on every single pseudo-move
to verify legality.

6. Run perft and confirm node counts are unchanged
Run  perft  at depths 1–5 from the starting position and a few tricky FENs (positions with castling, en passant,
promotions). Counts must be identical to before. This is your correctness gate — if any number differs, a table entry is
wrong.

### Part 2 — Piece List

7. Design the piece list data structure
Add two fields to  Position  in  position.rs :

•  piece_squares: [[u8; 16]; 2]  — indexed by  side as usize  (0=White, 1=Black), each slot holds a square
•  piece_counts: [u8; 2]  — how many pieces each side currently has

Max 16 pieces per side (standard chess), so fixed arrays, no heap allocation. You could alternatively store
(ColoredPiece, Square)  pairs if you want type info in the list, but since you already have the mailbox  board  to look
up piece type, storing just the square is enough and simpler.

8. Populate piece list in  load_position_from_fen
After  load_piece_placement  fills  board , scan the board once to populate  piece_squares  and  piece_counts  for both
sides. This replaces nothing — it's additive init code. While you're here, remove the second pass that finds king
squares (it can be merged into the same loop).

9. Update  make_move_unvalidated  in  make_move.rs
After each board mutation, mirror it in the piece lists:
• Any move: find  from_square  in the moving side's list, replace it with  to_square
• Capture ( MoveKind::Capture ): find  to_square  (before the move) in the opponent's list, swap-remove it (swap with
last element, decrement count — O(1))
• En passant ( MoveKind::EnPassant { capture_square } ): swap-remove  capture_square  from opponent's list instead of
to_square
• Promotion: the pawn's square entry is already updated by the "any move" step above; the piece type change doesn't
affect the list since the list only stores squares
• Castling: update the rook's entry in the moving side's list (find  rook_from , replace with  rook_to )

10. Update  undo_move_on_board  in  make_move.rs
Mirror the inverse of each operation in step 9:
• Any move: find  to_square  in the moving side's list, replace with  from_square
• Capture: push  to_square  back onto the opponent's list, increment count
• En passant: push  capture_square  back onto the opponent's list
• Castling: find  rook_to  in the moving side's list, replace with  rook_from

11. Rewrite  get_all_legal_moves_for_side  in  game_status.rs
Replace the  self.board.iter().enumerate().filter_map(...)  scan (currently line 85) with iteration over
self.piece_squares[side as usize][0..self.piece_counts[side as usize]] . Same logic after that — call  get_legal_moves
per square. Also remove the  Vec<Square>  intermediate allocation; iterate the slice directly.

12. Run perft again, confirm counts still match
Same check as step 6. If counts differ, the piece list is getting corrupted — add a debug assertion that cross-checks
piece_squares  against the mailbox  board  at the start of each  perft  call to find which move type is going wrong.
