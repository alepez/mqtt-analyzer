use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Frame;

use crate::tui::style::get_color;
use crate::tui::{App, BlockId};

fn draw_subscribe_text_input<B>(f: &mut Frame<B>, area: Rect, app: &App)
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
        app.navigation.parent() == BlockId::SubscriptionsList,
        app.navigation.peek() == BlockId::SubscriptionsList,
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
