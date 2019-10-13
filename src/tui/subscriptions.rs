use std::io::{self};
use std::thread;

use circular_queue::CircularQueue;
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

use crate::format::format_notification;
use crate::format::FormattedString;
use crate::format::MessageFormat;
use crate::tui::tabs::TabsState;
use crate::tui::App;

pub fn draw_topics_tab<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
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

    let subscriptions = app.subscriptions.iter().map(Text::raw);

    List::new(subscriptions)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Subscriptions"),
        )
        .render(f, chunks[1]);
}
