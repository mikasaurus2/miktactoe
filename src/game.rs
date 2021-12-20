use crate::board::{Board, BoardState};
use crate::common::{CellCoord, Marker, Move};
use crate::player::Player;
use crate::player::{
    ai_basic::BasicAI, ai_forking::ForkingAI, ai_optimal::OptimalAI, ai_random::RandomAI,
    human::Human,
};

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Done,
}

pub trait Game {
    fn run(&mut self) -> GameState;
    fn make_human_move(&mut self, player_move: CellCoord) -> GameState;
    fn get_game_state(&self) -> GameState;
    fn get_winner(&self) -> Winner;
    fn get_cellstate_char(&self, cell_index: usize) -> char;
    fn reset(&mut self);
}

pub struct TicTacToe<'a, P1, P2> {
    player1: P1,
    player2: P2,
    board: Board,
    record: Record<'a>,
    state: GameState,
}

#[derive(Copy, Clone)]
pub enum Winner {
    Player1,
    Player2,
    None,
}

struct Record<'a> {
    player1: &'a str,
    player2: &'a str,
    pub winner: Winner,
    move_history: Vec<CellCoord>,
}

impl<'a, P1: Player<'a>, P2: Player<'a>> TicTacToe<'a, P1, P2> {
    pub fn new(player1: P1, player2: P2) -> TicTacToe<'a, P1, P2> {
        TicTacToe {
            player1,
            player2,
            board: Board::new(),
            // TODO: fix the names on the record. These are just hardcoded.
            record: Record::new("steph", "mike"),
            state: GameState::Player1Turn,
        }
    }
}

impl<'a, P1: Player<'a>, P2: Player<'a>> Game for TicTacToe<'a, P1, P2> {

    fn run(&mut self) -> GameState {
        match self.state {
            GameState::Player1Turn => GameState::Player1Turn,
            GameState::Player2Turn => {
                let comp_move = self.player2.get_valid_move(&self.board);
                self.board
                    .place_marker(comp_move, self.player2.get_marker());
                self.record.record_move(comp_move);
                match self
                    .board
                    .check_board_state(comp_move, self.player2.get_marker())
                {
                    BoardState::Win => {
                        self.state = GameState::Done;
                        self.record.record_outcome(Winner::Player2);
                    }
                    BoardState::Tie => {
                        self.state = GameState::Done;
                        self.record.record_outcome(Winner::None);
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

    fn make_human_move(&mut self, player_move: CellCoord) -> GameState {
        if self.state == GameState::Player1Turn {
            if self.board.validate_move(player_move) == Move::Valid {
                self.board
                    .place_marker(player_move, self.player1.get_marker());
                self.record.record_move(player_move);
                match self
                    .board
                    .check_board_state(player_move, self.player1.get_marker())
                {
                    BoardState::Win => {
                        self.state = GameState::Done;
                        self.record.record_outcome(Winner::Player1);
                    }
                    BoardState::Tie => {
                        self.state = GameState::Done;
                        self.record.record_outcome(Winner::None);
                    }
                    BoardState::Playing => {
                        self.state = GameState::Player2Turn;
                    }
                }
            }
        }
        self.state
    }

    fn get_cellstate_char(&self, cell_index: usize) -> char {
        self.board.get_cellstate_char(cell_index)
    }

    fn get_game_state(&self) -> GameState {
        self.state
    }

    fn get_winner(&self) -> Winner {
        self.record.winner
    }

    fn reset(&mut self) {
        self.player1 = P1::new("Human", Marker::X);
        self.player2 = P2::new("Computer", Marker::O);
        self.board = Board::new();
        // TODO: fix the names on the record. These are just hardcoded.
        self.record = Record::new("steph", "mike");
        self.state = GameState::Player1Turn;
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
