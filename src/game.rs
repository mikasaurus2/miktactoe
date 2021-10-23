//mod board;
//use common;
//mod player;
//
use crate::board::Board;
use crate::common::{Marker, Move};
use crate::player::Player;

pub struct Game {
    player1: Player,
    player2: Player,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Player {
                name: String::from("Mike"),
                marker: Marker::X,
            },
            player2: Player {
                name: String::from("Steph"),
                marker: Marker::O,
            },
            board: Board::new(),
        }
    }

    pub fn run(&mut self) {
        let mut marker_count = 0;
        self.board.display();
        loop {
            let mut player_move = {
                let mut player_move = self.player1.get_move();
                let mut move_validation = self.board.validate_move(&player_move);
                while let Move::Invalid = move_validation {
                    player_move = self.player1.get_move();
                    move_validation = self.board.validate_move(&player_move);
                }
                player_move
            };
            self.board.place_marker(&player_move, &self.player1.marker);
            marker_count += 1;
            self.board.display();
            if self.board.check_win(&player_move, &self.player1.marker) {
                println!("{} won!", self.player1.name);
                break;
            }

            if marker_count == 9 {
                println!("The game was a tie!");
                return;
            }

            player_move = {
                let mut player_move = self.player2.get_move();
                let mut move_validation = self.board.validate_move(&player_move);
                while let Move::Invalid = move_validation {
                    player_move = self.player2.get_move();
                    move_validation = self.board.validate_move(&player_move);
                }
                player_move
            };
            self.board.place_marker(&player_move, &self.player2.marker);
            marker_count += 1;
            self.board.display();
            if self.board.check_win(&player_move, &self.player2.marker) {
                println!("{} won!", self.player2.name);
                break;
            }

            if marker_count == 9 {
                println!("The game was a tie!");
                return;
            }
        }
    }
}
