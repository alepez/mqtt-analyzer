use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};
use tui::Frame;

use crate::engine;
use crate::tui::navigation::BlockId;
use crate::tui::style::get_color;
use crate::tui::App;

fn draw_subscribe_input<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let highlight_state = (
        app.navigation.peek() == BlockId::SubscribeInput,
        app.navigation.peek() == BlockId::SubscribeInput,
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

fn draw_subscriptions_list<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let subscriptions: Vec<_> = app
        .engine
        .subscriptions
        .read()
        .map(|subscriptions| subscriptions.clone().into_iter().collect())
        .unwrap();

    let highlight_state = (
        app.navigation.parent() == BlockId::SubscriptionsList,
        app.navigation.peek() == BlockId::SubscriptionsList,
    );

    let selected_subscription_index =
        if let BlockId::SubscriptionsListItem(i) = app.navigation.peek() {
            i
        } else {
            0
        };

    SelectableList::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscriptions")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .items(subscriptions.as_slice())
        .select(Some(selected_subscription_index))
        .highlight_style(Style::default().fg(Color::LightCyan))
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

    draw_subscribe_input(f, chunks[0], app);
    draw_subscriptions_list(f, chunks[1], app);
}

pub fn handle_input_on_subscriptions_list(c: Key, app: &mut App) {
    use Key::*;

    match c {
        Up => {
            app.navigation.modify_top(BlockId::SubscribeInput);
        }
        Char('\n') => {
            if app.engine.subscriptions.read().unwrap().len() > 0 {
                app.navigation.push(BlockId::SubscriptionsListItem(0));
            }
        }
        _ => {}
    }
}

pub fn handle_input_on_subscriptions_list_item(c: Key, app: &mut App, index: usize) {
    use Key::*;

    let subscriptions_len = app.engine.subscriptions.read().unwrap().len();

    if subscriptions_len == 0 {
        app.navigation.pop();
        return;
    }

    let max = subscriptions_len - 1;
    let prev_index = index - (if index > 0 { 1 } else { 0 });
    let next_index = index + (if index < max { 1 } else { 0 });

    match c {
        Up => app
            .navigation
            .modify_top(BlockId::SubscriptionsListItem(prev_index)),
        Down => app
            .navigation
            .modify_top(BlockId::SubscriptionsListItem(next_index)),
        Char('d') | Char('x') | Backspace | Delete => {
            let sub = app
                .engine
                .subscriptions
                .read()
                .map(|x| x.iter().nth(index).cloned());

            if let Ok(Some(sub)) = sub {
                app.engine
                    .tx()
                    .send(engine::Event::Unsubscribe(sub))
                    .unwrap();
            }
        }
        _ => (),
    }
}

pub fn handle_input_on_subscribe_input(c: Key, app: &mut App) {
    use Key::*;

    match c {
        Up => {
            app.subscribe_input.clear();
            app.navigation.modify_top(BlockId::TabNav);
        }
        Down => {
            app.subscribe_input.clear();
            app.navigation.modify_top(BlockId::SubscriptionsList);
        }
        Char('\n') => {
            let sub: String = app.subscribe_input.drain(..).collect();
            if !sub.is_empty() {
                app.engine
                    .tx()
                    .send(crate::engine::Event::Subscribe(sub.clone()))
                    .unwrap();
            }
        }
        Backspace => {
            app.subscribe_input.pop();
        }
        Char(c) => {
            app.subscribe_input.push(c);
        }
        _ => {}
    }
}
