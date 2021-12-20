// Removed all the code from lib.rs. Instead, it now imports
// all the necessary modules, and runs the game. Each module
// has a corresponding <module_name>.rs file in the same directory
// as this lib.rs file.
mod board;
mod common;
mod game;
mod player;

use std::{fmt, io};

use common::{CellCoord, Marker};
use game::{Game, GameState, TicTacToe, Winner};
use player::*;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct MenuList<T> {
    state: ListState,
    items: Vec<T>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MainMenuEntry {
    Play,
    Exit,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PlayerTypeEntry {
    Human,
    RandomComp,
    BasicComp,
    OptimalComp,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum EndMenuEntry {
    PlayAgain,
    Exit,
}

// We need to implement display here so we can convert the enum
// into a string.
impl fmt::Display for MainMenuEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PlayerTypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerTypeEntry::Human => write!(f, "Human"),
            PlayerTypeEntry::RandomComp => write!(f, "Random Computer"),
            PlayerTypeEntry::BasicComp => write!(f, "Basic Computer"),
            PlayerTypeEntry::OptimalComp => write!(f, "Optimal Computer"),
        }
    }
}

impl fmt::Display for EndMenuEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EndMenuEntry::PlayAgain => write!(f, "Play Again"),
            EndMenuEntry::Exit => write!(f, "Exit"),
        }
    }
}

impl<T> MenuList<T> {
    fn with_items(items: Vec<T>) -> MenuList<T> {
        let mut list = MenuList {
            state: ListState::default(),
            items,
        };
        list.state.select(Some(0));
        list
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

struct App {
    main_menu: MenuList<MainMenuEntry>,
    player_select_menu: MenuList<PlayerTypeEntry>,
    end_menu: MenuList<EndMenuEntry>,
    selected_cell: u8,
    game: Box<dyn Game>,
}

impl App {
    fn new() -> App {
        App {
            main_menu: MenuList::with_items(vec![MainMenuEntry::Play, MainMenuEntry::Exit]),
            player_select_menu: MenuList::with_items(vec![
                PlayerTypeEntry::Human,
                PlayerTypeEntry::RandomComp,
                PlayerTypeEntry::BasicComp,
                PlayerTypeEntry::OptimalComp,
            ]),
            end_menu: MenuList::with_items(vec![EndMenuEntry::PlayAgain, EndMenuEntry::Exit]),
            selected_cell: 0,
            // We don't want to create th Game object when we start the App, because the user
            // hasn't selected their opponent yet. We can't have an uninitialized Box, so
            // we should use Option here.
            // If we do that though, we'd have to place `expect()` calls on ever invocation
            // of a function through the `game` member variable. And also `as_ref()`. This
            // muddies the code quite a bit.
            game: Box::new(TicTacToe::new(
                human::Human::new("Human", Marker::X),
                ai_optimal::OptimalAI::new("Optimal", Marker::O),
            )),
        }
    }

    fn handle_main_menu_enter(&self) -> MainMenuEntry {
        match self.main_menu.state.selected() {
            Some(i) => self.main_menu.items[i],
            None => MainMenuEntry::Exit,
        }
    }

    fn handle_player_select_menu_enter(&mut self) {
        match self.player_select_menu.state.selected() {
            Some(i) => match self.player_select_menu.items[i] {
                PlayerTypeEntry::Human => {
                    todo!("Human vs Human is not yet supported");
                }
                PlayerTypeEntry::RandomComp => {
                    self.game = Box::new(TicTacToe::new(
                        human::Human::new("Human", Marker::X),
                        ai_random::RandomAI::new("Random", Marker::O),
                    ));
                }
                PlayerTypeEntry::BasicComp => {
                    self.game = Box::new(TicTacToe::new(
                        human::Human::new("Human", Marker::X),
                        ai_basic::BasicAI::new("Basic", Marker::O),
                    ));
                }
                PlayerTypeEntry::OptimalComp => {
                    self.game = Box::new(TicTacToe::new(
                        human::Human::new("Human", Marker::X),
                        ai_optimal::OptimalAI::new("Optimal", Marker::O),
                    ));
                }
            },
            None => {}
        };
    }

    fn handle_end_menu_enter(&self) -> EndMenuEntry {
        match self.end_menu.state.selected() {
            Some(i) => self.end_menu.items[i],
            None => EndMenuEntry::Exit,
        }
    }

    fn update_selected_cell(&mut self, key: &KeyCode) {
        match key {
            KeyCode::Right => {
                if self.selected_cell != 2 && self.selected_cell != 5 && self.selected_cell != 8 {
                    self.selected_cell += 1;
                }
            }
            KeyCode::Left => {
                if self.selected_cell != 0 && self.selected_cell != 3 && self.selected_cell != 6 {
                    self.selected_cell -= 1;
                }
            }
            KeyCode::Down if self.selected_cell < 6 => self.selected_cell += 3,
            KeyCode::Up if self.selected_cell > 2 => self.selected_cell -= 3,
            _ => {}
        }
    }

    fn restart_game(&mut self) {
        self.game.reset();
    }
}

pub fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let choice = loop {
        terminal.draw(|f| menu_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => app.main_menu.next(),
                KeyCode::Up => app.main_menu.previous(),
                KeyCode::Enter => break app.handle_main_menu_enter(),
                _ => {}
            };
        }
    };

    if choice == MainMenuEntry::Play {
        loop {
            terminal.draw(|f| player_select_ui(f, &mut app))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => app.player_select_menu.next(),
                    KeyCode::Up => app.player_select_menu.previous(),
                    KeyCode::Enter => break app.handle_player_select_menu_enter(),
                    _ => {}
                }
            }
        }

        loop {
            let mut game_state = app.game.run();
            let choice = loop {
                terminal.draw(|f| board_ui(f, &mut app))?;

                match game_state {
                    GameState::Player1Turn => {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Char('q') => break EndMenuEntry::Exit,
                                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                                    app.update_selected_cell(&key.code);
                                }
                                KeyCode::Enter => {
                                    let player_move = CellCoord::new(
                                        usize::from(app.selected_cell / 3),
                                        usize::from(app.selected_cell % 3),
                                    );
                                    game_state = app.game.make_human_move(player_move);
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }
                    GameState::Player2Turn => {
                        game_state = app.game.run();
                    }
                    GameState::Done => {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Down => app.end_menu.next(),
                                KeyCode::Up => app.end_menu.previous(),
                                KeyCode::Enter => break app.handle_end_menu_enter(),
                                _ => {}
                            }
                        }
                    }
                }
            };

            match choice {
                EndMenuEntry::PlayAgain => app.restart_game(),
                EndMenuEntry::Exit => break,
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn menu_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let items: Vec<ListItem> = app
        .main_menu
        .items
        .iter()
        .map(|&i| ListItem::new(Span::raw(i.to_string())).style(Style::default().fg(Color::White)))
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, center_chunks[1], &mut app.main_menu.state)
}

fn player_select_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let items: Vec<ListItem> = app
        .player_select_menu
        .items
        .iter()
        .map(|&i| ListItem::new(Span::raw(i.to_string())).style(Style::default().fg(Color::White)))
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Opponent"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, center_chunks[1], &mut app.player_select_menu.state)
}

fn board_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // If the Game is done, show the result, and allow user
    // to select whether to play again.
    if app.game.get_game_state() == GameState::Done {
        let left_box = center_chunks[0];
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(left_box);

        let text = match app.game.get_winner() {
            // TODO: Once we figure out how to use names, we should indicate who won
            // by name. This is correct, but pretty generic.
            Winner::Player1 => "Player 1 Won!",
            Winner::Player2 => "Player 2 Won!",
            Winner::None => "The game was a tie!",
        };
        let end_prompt = List::new([ListItem::new(Span::raw(text))]).block(
            Block::default()
                .borders(Borders::ALL),
        );
        f.render_widget(end_prompt, left_chunks[0]);

        let items: Vec<ListItem> = app
            .end_menu
            .items
            .iter()
            .map(|&i| {
                ListItem::new(Span::raw(i.to_string())).style(Style::default().fg(Color::White))
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, left_chunks[1], &mut app.end_menu.state)
    }

    // The block layout with TUI is a bit weird. The last block
    // will try to fill the remaining space in the parent block.
    // As a result, I added a fourth block for which I don't draw
    // a border. That way, only the 3x3 board is displayed.
    let center_box = center_chunks[1];
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(center_box);

    let mut cell_index = 0;
    for row in &rows[0..3] {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ]
                .as_ref(),
            )
            .split(*row);

        for (index, _) in columns[0..3].iter().enumerate() {
            let border_style = if cell_index == app.selected_cell {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::Red)
            };

            let marker = List::new([ListItem::new(Span::raw(format!(
                "{:^3}",
                app.game.get_cellstate_char(usize::from(cell_index))
            )))])
            .block(
                Block::default()
                    .border_style(border_style)
                    .borders(Borders::ALL),
            );

            f.render_widget(marker, columns[index]);
            cell_index += 1;
        }
    }
}
