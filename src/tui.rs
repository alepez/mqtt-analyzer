use std::io::{self};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rumqtt::{Notification, Receiver};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    match evt {
                        Ok(key) => {
                            if let Err(_) = tx.send(Event::Input(key)) {
                                return;
                            }
                            if key == config.exit_key {
                                return;
                            }
                        }
                        Err(_) => {}
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

struct App<'a> {
    tabs: TabsState<'a>,
}

pub fn start_tui(notifications: Receiver<Notification>) -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    // App
    let mut app = App {
        tabs: TabsState::new(vec!["Topics", "Stream", "Retain", "Statistics"]),
    };

    // Main loop
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            Block::default()
                .style(Style::default().bg(Color::White))
                .render(&mut f, size);
            Tabs::default()
                .block(Block::default().borders(Borders::ALL))
                .titles(&app.tabs.titles)
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);
            match app.tabs.index {
                0 => Block::default()
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                1 => Block::default()
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                2 => Block::default()
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                3 => Block::default()
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                _ => {}
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
