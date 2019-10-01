use std::io::{self};
use std::sync::mpsc;
use std::thread;

use rumqtt::{Notification, Receiver};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Tabs, Text, Widget};
use tui::{Frame, Terminal};

use utils::{Event, Events};

mod utils;

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
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

struct App {
    tabs: TabsState,
    subscriptions: Vec<String>,
    subscribe_input: String,
    notifications: Vec<Notification>, // FIXME grow forever
}

impl Default for App {
    fn default() -> App {
        let tabs = vec!["Subscriptions", "Stream", "Retain", "Statistics"]
            .iter()
            .map(|&s| String::from(s))
            .collect();

        App {
            tabs: TabsState::new(tabs),
            subscriptions: Vec::new(),
            subscribe_input: String::new(),
            notifications: Vec::new(),
        }
    }
}

fn draw_topics_tab<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    Paragraph::new([Text::raw(&app.subscribe_input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Subscribe"))
        .render(f, chunks[0]);

    let messages = app
        .subscriptions
        .iter()
        .enumerate()
        .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));

    List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscriptions"),
        )
        .render(f, chunks[1]);
}

fn draw_stream_tab<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let formatted_notifications = app
        .notifications
        .iter()
        .map(|n| Text::raw(format!("{:?}", n)));

    List::new(formatted_notifications)
        .block(Block::default().borders(Borders::ALL))
        .render(f, area);
}

pub fn start_tui(notifications: Receiver<Notification>) -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut app = App::default();

    {
        let tx = events.tx();
        thread::spawn(move || {
            for notification in notifications {
                tx.send(Event::MqttNotification(notification)).unwrap();
            }
        });
    }

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
                0 => draw_topics_tab(&mut f, chunks[1], &mut app),
                1 => draw_stream_tab(&mut f, chunks[1], &mut app),
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
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Char('\n') => {
                    app.subscriptions
                        .push(app.subscribe_input.drain(..).collect());
                }
                Key::Char(c) => {
                    app.subscribe_input.push(c);
                }
                Key::Backspace => {
                    app.subscribe_input.pop();
                }
                _ => {}
            },
            Event::MqttNotification(notification) => {
                app.notifications.push(notification);
            }
            _ => {}
        }
    }
    Ok(())
}
