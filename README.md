# Chess
It's a chess REPL

## Commands

#### `cargo run v3`
```
peek <position>               Show the piece at the given position>

                                > peek e2
                                w_P


board                         Renders the chess board.

                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              8| b_R | b_N | b_B | b_Q | b_K | b_B | b_N | b_R |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              7| b_P | b_P | b_P | b_P | b_P | b_P | b_P | b_P |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              6|     |     |     |     |     |     |     |     |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              5|     |     |     |     |     |     |     |     |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              4|     |     |     |     |     |     |     |     |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              3|     |     |     |     |     |     |     |     |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              2| w_P | w_P | w_P | w_P | w_P | w_P | w_P | w_P |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                              1| w_R | w_N | w_B | w_Q | w_K | w_B | w_N | w_R |
                               |-----|-----|-----|-----|-----|-----|-----|-----|
                                  a     b     c     d     e     f     g     h    


move <from> <to>              Move a piece on the board.

                                > move e2 e4
                          

undo                          Undo the last move.


moves <from>                  PARTIALLY IMPLEMENTED (only implemented 
                              for pawns). Get list of tiles the 
                              piece at the given position can move to.

                                > moves e2


history                       Get a list of all moves made so far.


vector <from> ...direction    Get a list of tiles a piece would land on by 
                              following the provided directions. This was 
                              mostly implemented as a way to test internal 
                              path finding functions...

                              Direction argments can be:
                              'up' or 'w'
                              'right' or 'd'
                              'down' or 's'
                              'right' or 'a'
                              
                                > vector e2 up left
                                d3, c4, b5, a6,

                                > vector e2 up up left
                                d4, c6, b8,

                                > vector e2 up 
                                e3, e4, e5, e6, e7, e8
```

#### `cargo run v4`
```
board                         Display the current state of the game board.

move <from> <to> [opts]       Move a piece on the board.
                                > move e2 e4
                              [opts]:
                                -f --force      Move the piece <from> tile x <to> to tile b 
                                                regardless of whether or not the move is legal

moves <from>                  Display available moves for the piece at a given position <from>
                                > moves e2

undo                          Undo the last move. Reverts the game board to its state prior to 
                              the last executed 'move' command.

history                       Display the move history.

help                          Display this list of commands.
```