// Removed all the code from lib.rs. Instead, it now imports
// all the necessary modules, and runs the game. Each module
// has a corresponding <module_name>.rs file in the same directory
// as this lib.rs file.
mod board;
mod common;
mod game;
mod player;

pub fn run_game() {
    let mut game = game::Game::new();
    game.run();
}
