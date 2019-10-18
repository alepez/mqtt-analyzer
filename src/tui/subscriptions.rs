use std::sync::mpsc::Sender;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Frame;

use crate::engine::Event;
use crate::tui::style::style;
use crate::tui::App;

pub fn draw_subscriptions_tab<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    Paragraph::new([Text::raw(&app.subscribe_input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscribe")
                .border_style(style(app.writing_subscription)),
        )
        .render(f, chunks[0]);

    let subscriptions = app.subscriptions.iter().map(Text::raw);

    List::new(subscriptions)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscriptions"),
        )
        .render(f, chunks[1]);
}

pub fn handle_subscriptions_input(input: Key, app: &mut App, engine_tx: &Sender<Event>) {
    match input {
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
    }
}
