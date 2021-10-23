# Let's Learn Rust With Tic-Tac-Toe


## Plan of action

### 1. ✓Get a basic board printed.

I set up some basic structures needed for the game. This includes structs like
`Marker`, `Player`, and `Board`. The `Game` struct manages ownership and executes
the game driving logic.

### 2. ✓Run a basic game with 2 players.

I implemented the game logic and win checking functions. Win checking is done
based on the coordinates of the last move. It checks the corresponding row, column,
and diagonals.

Rust iterators and closures make for some pretty elegant and concise win checking
code.

### 3. commit point

I committed the basic working game and tagged it with `v1`. My intention here
is that those who are interested can checkout specific tags to see the progress
described here.

tag: `v1`

### 4. split to main.rs and lib.rs and other modules
### 5. make random computer player
### 6. make optimal computer player
### 7. add text user interface representation
### 8. allow choosing human or computer players
### 9. make web service to serve games to clients
### 10. run web service on cloud
