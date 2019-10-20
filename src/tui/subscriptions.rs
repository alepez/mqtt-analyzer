use std::sync::mpsc::Sender;

use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Frame;

use crate::engine::Event;
use crate::tui::style::get_color;
use crate::tui::{App, BlockId, Route};

fn draw_subscribe_text_input<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let highlight_state = (
        app.navigation.active_block == BlockId::SubscribeInput,
        app.navigation.hovered_block == BlockId::SubscribeInput,
    );

    Paragraph::new([Text::raw(&app.subscribe_input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscribe")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .render(f, area);
}

pub fn draw_subscriptions_tab<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    draw_subscribe_text_input(f, chunks[0], app);

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
    let writing_subscription = app.navigation.active_block == BlockId::SubscribeInput;

    match input {
        Key::Up => {
            app.navigation = Route {
                id: BlockId::Tabs,
                active_block: BlockId::Tabs,
                hovered_block: BlockId::Tabs,
            }
        }
        Key::Char('\n') => {
            if writing_subscription {
                let sub: String = app.subscribe_input.drain(..).collect();
                engine_tx
                    .send(crate::engine::Event::Subscribe(sub.clone()))
                    .unwrap();
                app.subscriptions.push(sub);
            } else {
                app.navigation.active_block = BlockId::SubscribeInput;
            }
        }
        Key::Char('/') => {
            if writing_subscription {
                app.subscribe_input.push('/');
            }
        }
        Key::Esc => {
            if writing_subscription {
                app.navigation.active_block = BlockId::None;
            }
        }
        Key::Char(c) => {
            if writing_subscription {
                app.subscribe_input.push(c);
            }
        }
        Key::Backspace => {
            if writing_subscription {
                app.subscribe_input.pop();
            }
        }
        _ => {}
    }
}
