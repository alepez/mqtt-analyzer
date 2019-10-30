use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, List, Widget};
use tui::Frame;

use crate::format::format_notification;
use crate::format::FormattedString;
use crate::format::MessageFormat;
use crate::tui::App;

pub fn draw_stream_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, format: MessageFormat)
where
    B: Backend,
{
    let formatted: Vec<FormattedString> = app
        .notifications
        .iter()
        .map(|notification| format_notification(format, notification))
        .collect();

    /* Reverse it because list is rendered bottom to top */
    let formatted = formatted.iter().map(|n| n.into());

    List::new(formatted)
        .block(Block::default().borders(Borders::ALL))
        .start_corner(Corner::BottomLeft)
        .render(f, area);
}
