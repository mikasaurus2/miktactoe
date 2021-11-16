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

tag: [`v2`](https://github.com/mikasaurus2/miktactoe/tree/v2)

### ✓make random computer player

I created a random computer player with pregenerated move coordinates. I created
a cartesian product of the two axis indexes and randomized the ordering to simulate
a random choice computer player.

tag: [`v3`](https://github.com/mikasaurus2/miktactoe/tree/v3)


### ✓make basic computer player

I broke out the player implementations into separate submodules and implemented a basic
AI player.

This basic AI will place a winning move it is available on the board. If not, it will
block any opponent winning move. If neither are present, it will move randomly.

I implemented a BoardMetadata struct to keep track of the winning coords for each player.
Each time a player places a marker, the board metadata is updated. This includes adding
and removing winning moves.

Removing winning moves from the metadata was interesting. I fought with the rust compiler
for a bit to get this to work. Initially, I wanted to assign a vector of callback functions
to each cell. These would serve as event handlers in the event that a cell had a marker placed
into it. Rust made this difficult because the callbacks would have mutable references to the board,
and Rust doesn't like multiple mutable references at the same time.

To overcome this, I ended up using an enum CellFlags, and assigning the flags to a cell.
Then, when we update the metadata, we iterate over the cell flags and handle them accordingly
based on the player's move. The logic for handling cell flags lives in the board, and so
there are not multiple mutable references anymore.

This makes the AI more fun already. :] You have to create a fork to win.

### ✓make computer that creates forks

A fork creates two winning spaces. Here's how we can determine which moves
would create forks for the computer.

When a marker is first place on any row or column, the row, column, and
diagonal are considered fork candidates. Anytime an opponent also occupies
a row, column, or diagonal, that row, column or diagonal is no longer a fork
candidate.

If two fork candidates overlap on any cell, their intersection is a forking
move.
    
For example, if X moves to col 0 row 1, the following diagram indicates
the fork candidates.
```
c _ _
X r r
c _ _
```

O places a marker at col1 row0.
```
c O _
X r r
c _ _
```

X places at col2 row2. (f indicates forking move for X)
```
f O x
X f f
f x X
```

```
cd  O   c
X   rd  cr
cr  r   X
```

O places at col2 row1.
```
cd  O  _
X   d  O
cr  r  X
```

X now has a number of forking move

I modified the board metadata to calculate these intersections to determine
the forking move. This update happens after every player move.


 

### make optimal computer player
### add text user interface representation
### allow choosing human or computer players
### make web service to serve games to clients
### run web service on cloud
