use crate::board::{Board, BoardState};
use crate::common::{CellCoord, Marker, Move};
use crate::player::{
    ai_basic::BasicAI, ai_forking::ForkingAI, ai_optimal::OptimalAI, ai_random::RandomAI,
    human::Human,
};

pub struct Game<'a> {
    player1: Human<'a>,
    player2: OptimalAI<'a>,
    board: Board,
    record: Record<'a>,
    state: GameState,
}

enum Winner {
    Player1,
    Player2,
    None,
}

struct Record<'a> {
    player1: &'a str,
    player2: &'a str,
    winner: Winner,
    move_history: Vec<CellCoord>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Done,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            player1: Human::new("Mike", Marker::X),
            player2: OptimalAI::new("Optimal2", Marker::O),
            //player1: Human::new("steph", Marker::X),
            //player2: Human::new("mike", Marker::O),
            board: Board::new(),
            record: Record::new("steph", "mike"),
            state: GameState::Player1Turn,
        }
    }

    pub fn run(&mut self) -> GameState {
        match self.state {
            GameState::Player1Turn => GameState::Player1Turn,
            GameState::Player2Turn => {
                let comp_move = self.player2.get_valid_move(&self.board);
                self.board.place_marker(comp_move, self.player2.marker);
                match self.board.check_board_state(comp_move, self.player2.marker) {
                    BoardState::Win | BoardState::Tie => {
                        self.state = GameState::Done;
                    }
                    BoardState::Playing => {
                        self.state = GameState::Player1Turn;
                    }
                }
                self.state
            }
            GameState::Done => GameState::Done,
        }
    }

    pub fn make_human_move(&mut self, player_move: CellCoord) -> GameState {
        if self.state == GameState::Player1Turn {
            if self.board.validate_move(player_move) == Move::Valid {
                self.board.place_marker(player_move, self.player1.marker);
                match self
                    .board
                    .check_board_state(player_move, self.player1.marker)
                {
                    BoardState::Win | BoardState::Tie => {
                        self.state = GameState::Done;
                    }
                    BoardState::Playing => {
                        self.state = GameState::Player2Turn;
                    }
                }
            }
        }
        self.state
    }

    pub fn get_cellstate_char(&self, cell_index: usize) -> char {
        self.board.get_cellstate_char(cell_index)
    }

    pub fn ui_run(&mut self) {
        self.board.display();
        loop {
            let mut player_move = self.player1.get_valid_move(&self.board);
            self.board.place_marker(player_move, self.player1.marker);
            self.record.record_move(player_move);
            //self.board.print_info();
            self.board.display();
            match self
                .board
                .check_board_state(player_move, self.player1.marker)
            {
                BoardState::Win => {
                    println!("{} won!", self.player1.name);
                    self.record.record_outcome(Winner::Player1);
                    break;
                }
                BoardState::Tie => {
                    println!("The game was a tie!");
                    self.record.record_outcome(Winner::None);
                    break;
                }
                BoardState::Playing => (),
            }

            player_move = self.player2.get_valid_move(&self.board);
            self.board.place_marker(player_move, self.player2.marker);
            self.record.record_move(player_move);
            //self.board.print_info();
            self.board.display();
            match self
                .board
                .check_board_state(player_move, self.player2.marker)
            {
                BoardState::Win => {
                    println!("{} won!", self.player2.name);
                    self.record.record_outcome(Winner::Player2);
                    break;
                }
                BoardState::Tie => {
                    println!("The game was a tie!");
                    self.record.record_outcome(Winner::None);
                    break;
                }
                BoardState::Playing => (),
            }
        }
        self.record.print_game_history();
    }
}

impl<'a> Record<'a> {
    fn new(player1: &'a str, player2: &'a str) -> Record<'a> {
        Record {
            player1,
            player2,
            winner: Winner::None,
            move_history: Vec::new(),
        }
    }

    fn record_move(&mut self, player_move: CellCoord) {
        self.move_history.push(player_move);
    }

    fn record_outcome(&mut self, winner: Winner) {
        self.winner = winner;
    }

    fn print_game_history(&self) {
        println!("Player1 ({})", self.player1);
        println!("Player2 ({})", self.player2);
        self.move_history
            .iter()
            .enumerate()
            .map(|(index, coord)| match (index + 1) % 2 {
                1 => (index + 1, Marker::X, coord),
                0 => (index + 1, Marker::O, coord),
                _ => (0, Marker::X, coord),
            })
            .for_each(|(index, marker, coord)| {
                println!("{}: {:?} {:?}", index, marker, coord);
            });
        match self.winner {
            Winner::Player1 => println!("Player1 ({}) won", self.player1),
            Winner::Player2 => println!("Player2 ({}) won", self.player2),
            Winner::None => println!("Game tied"),
        }
    }
}
