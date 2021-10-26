//mod board;
//use common;
//mod player;
//
use crate::board::{Board, BoardState};
use crate::common::Marker;
use crate::player::{human::Human, ai_random::RandomAI, ai_basic::BasicAI};

pub struct Game {
    // Human players
    player1: Human,
    player2: Human,
    // Computer players
    //player1: RandomAI,
    //player2: BasicAI,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            // Human players
            //
            player1: Human {
                name: String::from("Mike"),
                marker: Marker::X,
            },
            player2: Human {
                name: String::from("Steph"),
                marker: Marker::O,
            },

            // Computer players
            //player1: RandomAI::new(String::from("Computron"), Marker::X),
            //player2: BasicAI::new(String::from("Hal9000"), Marker::O),
            board: Board::new(),
        }
    }

    pub fn run(&mut self) {
        self.board.display();
        loop {
            let mut player_move = self.player1.get_valid_move(&self.board);
            self.board.place_marker(player_move, self.player1.marker);
            self.board.display();
            match self
                .board
                .check_board_state(player_move, self.player1.marker)
            {
                BoardState::Win => {
                    println!("{} won!", self.player1.name);
                    break;
                }
                BoardState::Tie => {
                    println!("The game was a tie!");
                    return;
                }
                BoardState::Playing => (),
            }

            player_move = self.player2.get_valid_move(&self.board);
            self.board.place_marker(player_move, self.player2.marker);
            self.board.display();
            match self
                .board
                .check_board_state(player_move, self.player2.marker)
            {
                BoardState::Win => {
                    println!("{} won!", self.player2.name);
                    break;
                }
                BoardState::Tie => {
                    println!("The game was a tie!");
                    return;
                }
                BoardState::Playing => (),
            }
        }
    }
}
