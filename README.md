# Let's Learn Rust With Tic-Tac-Toe


## Plan of action

### ✓Get a basic board printed.

I set up some basic structures needed for the game. This includes structs like
`Marker`, `Player`, and `Board`. The `Game` struct manages ownership and executes
the game driving logic.

### ✓Run a basic game with 2 players.

I implemented the game logic and win checking functions. Win checking is done
based on the coordinates of the last move. It checks the corresponding row, column,
and diagonals.

Rust iterators and closures make for some pretty elegant and concise win checking
code.

### ✓commit point

I committed the basic working game and tagged it with `v1`. My intention here
is that those who are interested can checkout specific tags to see the progress
described here.

tag: [`v1`](https://github.com/mikasaurus2/miktactoe/tree/v1)

### ✓split to main.rs and lib.rs and other modules

I split the implementation code into separate modules. The basic structures
and their implementation are now in individual files. Each module is then
imported in `lib.rs` with the `mod` keyword. Modules that rely on code in
other modules then use `use` to bring the required functionality into scope.

For example, `player` module requires structures found in the `common` module.
It brings those structures into scope with `use create::common::*`.

### add some basic unit tests
### make random computer player
### make optimal computer player
### add text user interface representation
### allow choosing human or computer players
### make web service to serve games to clients
### run web service on cloud
