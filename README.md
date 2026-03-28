# Chrust
## Chrust is a simple 2 player chess game and later (hopefully) a chess engine

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

## Next steps
- Add UCI move support
- Make some helper functions pub
- Fix promotion piece can be king
- Add offer and accept draw
- Add game history converter
- Add perft

## Small errors 
    - Slider.rs: Find a way to not return a queen when a faulty piece is provided

## Important info
    - king_squares[0] is white and [1] is black
    - castling: 
        - [0] => White kingside
        - [1] => White queenside 
        - [2] => Black kingside
        - [3] => Black queenside
