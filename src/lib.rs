// Removed all the code from lib.rs. Instead, it now imports
// all the necessary modules, and runs the game. Each module
// has a corresponding <module_name>.rs file in the same directory
// as this lib.rs file.
mod board;
mod common;
mod game;
mod player;

use std::{fmt, io};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
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

// We need to implement display here so we can convert the enum
// into a string.
impl fmt::Display for MainMenuEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
    selected_cell: u8,
}

impl App {
    fn new() -> App {
        App {
            main_menu: MenuList::with_items(vec![MainMenuEntry::Play, MainMenuEntry::Exit]),
            selected_cell: 0,
        }
    }

    fn handle_main_menu_enter(&self) -> MainMenuEntry {
        match self.main_menu.state.selected() {
            Some(i) => self.main_menu.items[i],
            None => MainMenuEntry::Exit,
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
            }
        }
    };

    if choice == MainMenuEntry::Play {
        loop {
            terminal.draw(|f| board_ui(f, &mut app))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        app.update_selected_cell(&key.code);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    //let mut game = game::Game::new();
    //game.run();
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

fn board_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

    let center_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1])[1];

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
        .split(center_chunk);

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

            let marker = List::new([ListItem::new(Span::raw(format!("{:^3}", "X")))]).block(
                Block::default()
                    .border_style(border_style)
                    .borders(Borders::ALL),
            );

            f.render_widget(marker, columns[index]);
            cell_index += 1;
        }
    }
}
