//mod board;
//use common;
//mod player;
//
use crate::board::{Board, BoardState};
use crate::common::Marker;
use crate::player::RandomComputer;

pub struct Game {
    // Human players
    //player1: Player,
    //player2: Player,
    // Computer players
    player1: RandomComputer,
    player2: RandomComputer,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            // Human players
            //
            //player1: Player {
            //    name: String::from("Mike"),
            //    marker: Marker::X,
            //},
            //player2: Player {
            //    name: String::from("Steph"),
            //    marker: Marker::X,
            //},

            // Computer players
            player1: RandomComputer::new(String::from("Computron"), Marker::X),
            player2: RandomComputer::new(String::from("Hal9000"), Marker::O),
            board: Board::new(),
        }
    }

    pub fn run(&mut self) {
        self.board.display();
        loop {
            let mut player_move = self.player1.get_valid_move(&self.board);
            self.board.place_marker(&player_move, &self.player1.marker);
            self.board.display();
            match self
                .board
                .check_board_state(&player_move, &self.player1.marker)
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
            self.board.place_marker(&player_move, &self.player2.marker);
            self.board.display();
            match self
                .board
                .check_board_state(&player_move, &self.player2.marker)
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
