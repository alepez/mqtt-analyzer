use std::io::{self};
use std::thread;

use circular_queue::CircularQueue;
use rumqtt::{Notification, Receiver};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

use utils::{Event, Events};

use crate::engine::Engine;
use crate::format::MessageFormat;
use crate::tui::stream::draw_stream_tab;
use crate::tui::subscriptions::draw_subscriptions_tab;
use crate::tui::tabs::TabsState;

mod stream;
mod style;
mod subscriptions;
mod tabs;
mod utils;

pub struct App {
    tabs: TabsState,
    subscriptions: Vec<String>,
    subscribe_input: String,
    notifications: CircularQueue<Notification>,
    writing_subscription: bool,
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
            notifications: CircularQueue::with_capacity(100),
            writing_subscription: false,
        }
    }
}

pub fn draw_empty_tab<B>(
    f: &mut tui::Frame<B>,
    area: tui::layout::Rect,
    _app: &mut App,
    _format: MessageFormat,
) where
    B: tui::backend::Backend,
{
    Block::default().borders(Borders::ALL).render(f, area)
}

pub fn start_tui(engine: Engine, format_options: MessageFormat) -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let engine_tx = engine.tx();
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut app = App::default();

    let tx = events.tx();
    thread::spawn(move || {
        for notification in engine.notifications {
            tx.send(Event::MqttNotification(notification)).unwrap();
        }
    });

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
                0 => draw_subscriptions_tab(&mut f, chunks[1], &mut app),
                1 => draw_stream_tab(&mut f, chunks[1], &mut app, format_options),
                2 => draw_empty_tab(&mut f, chunks[1], &mut app, format_options),
                3 => draw_empty_tab(&mut f, chunks[1], &mut app, format_options),
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
                    if app.writing_subscription {
                        let sub: String = app.subscribe_input.drain(..).collect();
                        engine_tx
                            .send(crate::engine::Event::Subscribe(sub.clone()))
                            .unwrap();
                        app.subscriptions.push(sub);
                        app.writing_subscription = false;
                    }
                }
                Key::Char('/') => {
                    if !app.writing_subscription {
                        app.writing_subscription = true;
                    } else {
                        app.subscribe_input.push('/');
                    }
                }
                Key::Esc => {
                    if app.writing_subscription {
                        app.writing_subscription = false;
                    }
                }
                Key::Char(c) => {
                    if app.writing_subscription {
                        app.subscribe_input.push(c);
                    }
                }
                Key::Backspace => {
                    if app.writing_subscription {
                        app.subscribe_input.pop();
                    }
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
