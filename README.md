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

### ✓add some basic unit tests

Rust has a neat unit and integration test capability. The unit tests are written
in the same file as the implementation code, under a nested module called `test`.
Integration tests are placed in a top level `tests` directory.

I added unit tests to the `Board` implementation to ensure it places markers,
validates moves, and correctly asserts wins and ties. Writing these tests exposed
a bounds checking bug in my move validation code! (Go unit tests!)

Writing the test for detecting a tie game led to refactoring some code. The marker
count used to determine the tie was used by the game loop. Since the game loop
requires user input, it was difficult to test the tie check. Instead, I moved
the marker count into the `Board` implementation and added a new `enum BoardState`
that indicates whether somebody won, the game is a tie, or is currently being played.
I was then able to add the unit test easily.

However, at this stage, the game requires user input from the player to run.
This makes it difficult to write tests for that component. I'll have to refactor
this later. One possibility is to "hide" the user input code behind a trait, and
use different implementation for testing vs playing. I can then inject the proper
implementation into the `Game` struct for it to use.

### make random computer player
### make optimal computer player
### add text user interface representation
### allow choosing human or computer players
### make web service to serve games to clients
### run web service on cloud
