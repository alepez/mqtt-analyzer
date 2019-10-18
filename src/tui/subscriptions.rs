use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Frame;

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
