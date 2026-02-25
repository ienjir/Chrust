# Chrust
## Chrust is a simple 2 player chess game and later (hopefully) a chess engine

## Information
### Assets
The chess piece assets were not created by me. They were created by Uray M. János and are published on <a href="https://greenchess.net/info.php?item=downloads">greenchess.net</a>. The author retains the copyright. The images are licensed under the Creative Commons Attribution-ShareAlike 3.0 (CC BY-SA 3.0) license.

### Chessboard with intager positions
Files / X                          
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |  
|       | a     | b     | c     | d     | e     | f     | g     | h     |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **8** | 56    | 57    | 58    | 59    | 60    | 61    | 62    | 63    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **7** | 48    | 49    | 50    | 51    | 52    | 53    | 54    | 55    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **6** | 40    | 41    | 42    | 43    | 44    | 45    | 46    | 47    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **5** | 32    | 33    | 34    | 35    | 36    | 37    | 38    | 39    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **4** | 24    | 25    | 26    | 27    | 28    | 29    | 30    | 31    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **3** | 16    | 17    | 18    | 19    | 20    | 21    | 22    | 23    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **2** | 8     | 9     | 10    | 11    | 12    | 13    | 14    | 15    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **1** | 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
                           
                                        Ranks / Y


### Addition/Substraction to get to a specific position
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | 
| +21   | +22   | +23   | +24   | +25   | +26   | +27   |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| +13   | +14   | +15   | +16   | +17   | +18   | +19   |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| +5    | +6    | +7    | +8    | +9    | +10   | +11   |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| -3    | -2    | -1    |  X    | +1    | +2    | +3    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| -11   | -10   | -9    | -8    | -7    | -6    | -5    |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| -19   | -18   | -17   | -16   | -15   | -14   | -13   |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| -27   | -26   | -25   | -24   | -23   | -22   | -21   |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |


### Tests
#### For every piece
    - No piece on square
    - Wrong piece on square 
    - Empty board -> Map exact squares not just includes
    - Friendly and enemy pieces in way (for both pieces)

### King
    - Check if squares are really empty
    - Check if rooks (same side) are in the right places (for both sides)


## Real Chess Rule Checks (Cheat-Proof Play)

When accepting a move from UI/player input, do not trust raw click/input data. Only accept a move if it exists in the engine-generated legal move list for the current position.

### Move Accept Pipeline
1. Read intent only (`from`, `to`, optional promotion piece).
2. Verify `from` has a piece of `side_to_move`.
3. Generate legal moves for that side.
4. Find exact matching move (`from`, `to`, and promotion piece when needed).
5. Apply move and update all position state fields.
6. Reject everything else.

### Rules To Enforce Before Move Is Legal
- Piece belongs to side to move.
- Piece movement shape is valid (with blockers for sliders).
- Cannot capture own piece.
- King cannot stay in or move into check.
- If in check: only legal responses (king move, capture checker, block checker).
- If in double check: only king move.
- Pinned pieces cannot move to expose own king.
- Castling requires rights, rook present, clear path, and king not in/through/into check.
- En passant only on the correct immediate turn and only if it does not expose own king.
- Promotion only on last rank and only to `Q/R/B/N`.

### Rules To Enforce While Applying Move
- Move piece source -> destination.
- Remove captured piece (including en passant capture square).
- Toggle `side_to_move`.
- Update king square when king moves.
- Update castling rights on king/rook move and rook capture on home square.
- Set en passant target only after double pawn push; clear otherwise.
- Apply promotion piece replacement correctly.

## Current Coverage in This Repo

### Already Covered
- Piece-pattern pseudo-legal move generation for all piece types in `chrust_core/src/moves/move_gen/` (`pawn.rs`, `knight.rs`, `bishop.rs`, `rook.rs`, `queen.rs`, `king.rs`).
- Board-edge wrap prevention for king/knight/sliders in move generation.
- Check/attack detection helpers in `chrust_core/src/moves/move_gen/check.rs` (`is_square_attacked`, `is_king_in_check`).
- Castling generation checks for clear path, rook presence, and attacked king/path squares in `chrust_core/src/moves/move_gen/king.rs`.
- Core move application per move kind (quiet/capture/double push/en passant/promotion/castling) in `chrust_core/src/moves/make_move.rs`.
- UI currently limits selection to side-to-move pieces in `chrust_ui/src/controller.rs`.

### Partially Covered (Implemented In Some Places, Missing In Others)
- `King safety on move apply`: `make_move_validated` computes check after move, but does not reject self-check result yet (`chrust_core/src/moves/make_move.rs`).
- `Promotion flow`: UI asks for promotion choice, but pawn movegen currently emits promotion with `Some(Piece::Pawn)` and does not generate all legal promotion options/capture-promotions (`chrust_core/src/moves/move_gen/pawn.rs`, `chrust_ui/src/controller.rs`).
- `Side-to-move enforcement`: present in UI flow, but not guaranteed as a core API invariant in movegen/apply functions if called directly (`chrust_ui/src/controller.rs`, `chrust_core/src/moves/move_gen/move_gen.rs`).

### Not Yet Done (Must Be Added For Fully Real-Chess Enforcement)
- Global legal move generation for a side (not only per-square pseudo-legal generation): `generate_legal_moves(position)` style API.
- Full pin/check resolution filtering for non-king pieces at engine level (instead of relying on caller behavior).
- Castling rights updates during move application (king/rook move, rook capture on start square).
- Correct promotion legality set (`Q/R/B/N` only, including capture promotions).
- Full game-end/state rules (checkmate, stalemate, repetition, 50-move rule, insufficient material).


## Next Steps (Current Focus Only: Legal Move Generation)

### Phase A: Build `generate_legal_moves(position)`
- [ ] Keep piece-specific functions as pseudo-legal generators (`pawn_targets`, `knight_targets`, `bishop_targets`, `rook_targets`, `queen_targets`, `king_targets`)
- [ ] Add global move collection for `side_to_move` (iterate board and aggregate pseudo-legal moves)
- [ ] Add legality filtering by simulation: apply each pseudo-legal move on a temporary position and reject if own king is in check afterward
- [ ] Use this legal list as the source of truth for `get_possible_moves(position, from_square)` by filtering from-square moves from the global legal list

### Phase B: Check Handling Correctness (inside legal movegen)
- [ ] Ensure single-check responses include king moves, capture-checker moves, and block-check moves (for slider checks)
- [ ] Ensure double-check allows only king moves (naturally via legality filtering)
- [ ] Ensure pinned-piece moves that expose own king are rejected

### Phase C: Movegen Test Coverage
- [ ] Add targeted tests for single-check block/capture responses
- [ ] Add targeted tests for double-check king-only behavior
- [ ] Add targeted tests for pinned-piece restrictions
- [ ] Add perft tests (depth 1-5) to validate movegen correctness

### Deferred Until Movegen Is Stable
- [ ] Move application validation hardening (`make_move_validated`)
- [ ] Castling rights state updates during apply
- [ ] Promotion UX/application cleanup
- [ ] Game-end and draw-rule handling
- [ ] FEN parser hardening


## Product Roadmap (Now: Movegen-First)

### Phase 1: Legal Movegen Core (Current Priority)
- [ ] Finish board-wide legal move generation in `chrust_core`
- [ ] Enforce king-safety legality by post-move check filtering
- [ ] Cover checks, blocks, captures, pins, and king-only double-check responses
- [ ] Validate against perft depth 1-5 and targeted tactical test positions
- [ ] Ensure UI only presents and accepts engine-legal moves

### Later Phases (Intentionally Deferred)
- [ ] Move application/state updates and full rule enforcement
- [ ] Game-end and draw-rule detection
- [ ] Architecture decisions, product features, and engine/search track
- [ ] Release automation and distribution
