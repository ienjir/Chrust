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

