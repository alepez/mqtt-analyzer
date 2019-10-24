use std::sync::mpsc::Sender;

use colored::Color::Black;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Frame;

use crate::engine::Event;
use crate::engine::Event::Unsubscribe;
use crate::tui::style::get_color;
use crate::tui::{App, BlockId, Breadcrumbs, Route};

fn draw_subscribe_text_input<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let highlight_state = (
        app.navigation.peek().hovered_block == BlockId::SubscribeInput,
        app.navigation.peek().hovered_block == BlockId::SubscribeInput,
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

    let subscriptions = app
        .engine
        .subscriptions
        .read()
        .map(|subscriptions| subscriptions.clone().into_iter().map(Text::raw))
        .unwrap();

    let highlight_state = (
        app.navigation.peek().id == BlockId::Subscriptions,
        app.navigation.peek().hovered_block == BlockId::Subscriptions,
    );

    List::new(subscriptions)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscriptions")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .render(f, chunks[1]);
}

fn handle_subscriptions_input_on_subscribe(input: Key, app: &mut App, engine_tx: &Sender<Event>) {
    match input {
        Key::Up => {
            app.navigation.peek_mut().hovered_block = BlockId::Tabs;
            app.subscribe_input.clear();
        }
        Key::Down => {
            app.navigation.peek_mut().hovered_block = BlockId::Subscriptions;
            app.subscribe_input.clear();
        }
        Key::Char('\n') => {
            let sub: String = app.subscribe_input.drain(..).collect();
            if !sub.is_empty() {
                engine_tx
                    .send(crate::engine::Event::Subscribe(sub.clone()))
                    .unwrap();
            }
        }
        Key::Backspace => {
            app.subscribe_input.pop();
        }
        Key::Char(c) => {
            app.subscribe_input.push(c);
        }
        _ => {}
    }
}

fn handle_subscriptions_input_on_list(
    input: Key,
    app: &mut App,
    engine_tx: &Sender<Event>,
    breadcrumbs: Breadcrumbs,
) {
    match breadcrumbs.front().map(|x| x.id) {
        None => {
            match input {
                Key::Up => {
                    app.navigation.peek_mut().hovered_block = BlockId::SubscribeInput;
                    app.subscribe_input.clear();
                }
                Key::Char('d') | Key::Backspace | Key::Delete => {
                    // TODO Delete subscription
                }
                Key::Char('\n') => app.navigation.push(Route {
                    id: BlockId::Subscriptions,
                    hovered_block: BlockId::None,
                }),
                _ => {}
            }
        }
        Some(BlockId::Subscriptions) => {
            handle_subscriptions_input_on_active_list(input, app, engine_tx)
        }
        x => panic!("{:?}", x),
    }
}

fn handle_subscriptions_input_on_active_list(input: Key, app: &mut App, engine_tx: &Sender<Event>) {
    match input {
        Key::Esc | Key::Char('q') => app.navigation.pop(),
        Key::Up => {
            // TODO To upper element
        }
        Key::Down => {
            // TODO To lower element
        }
        Key::Char('d') | Key::Backspace | Key::Delete => {
            // TODO Delete select element (now delete the first)
            let sub = app
                .engine
                .subscriptions
                .read()
                .map(|x| x.iter().next().cloned());
            if let Ok(Some(sub)) = sub {
                // TODO MqttClient does not yet implement unsubscribe
                engine_tx.send(Unsubscribe(sub)).unwrap();
            }
        }
        _ => {}
    }
}

pub fn handle_subscriptions_input(
    input: Key,
    app: &mut App,
    engine_tx: &Sender<Event>,
    mut breadcrumbs: Breadcrumbs,
) {
    match breadcrumbs.front().map(|x| x.id) {
        Some(BlockId::Root) => match breadcrumbs.front().map(|x| x.hovered_block) {
            Some(BlockId::SubscribeInput) => {
                handle_subscriptions_input_on_subscribe(input, app, engine_tx)
            }
            Some(BlockId::Subscriptions) => {
                handle_subscriptions_input_on_list(input, app, engine_tx, breadcrumbs.split_off(1))
            }
            x => panic!("{:?}", x),
        },
        x => panic!("{:?}", x),
    }
}
