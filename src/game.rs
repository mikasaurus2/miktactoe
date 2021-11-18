//mod board;
//use common;
//mod player;
//
use crate::board::{Board, BoardState};
use crate::common::Marker;
use crate::player::{
    ai_basic::BasicAI, ai_forking::ForkingAI, ai_optimal::OptimalAI, ai_random::RandomAI,
    human::Human,
};

pub struct Game<'a> {
    player1: OptimalAI<'a>,
    player2: OptimalAI<'a>,
    board: Board,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            player1: OptimalAI::new("Optimal1", Marker::X),
            player2: OptimalAI::new("Optimal2", Marker::O),
            board: Board::new(),
        }
    }

    pub fn run(&mut self) {
        self.board.display();
        loop {
            let mut player_move = self.player1.get_valid_move(&self.board);
            self.board.place_marker(player_move, self.player1.marker);
            //self.board.print_info();
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
            //self.board.print_info();
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
