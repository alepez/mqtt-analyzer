use rumqtt::Receiver;
use tui::backend::Backend;
use tui::layout::{Corner, Layout, Rect};
use tui::widgets::{Block, Borders, List, Text, Widget};
use tui::Frame;

use crate::format::format_notification;
use crate::format::FormattedString;
use crate::format::MessageFormat;
use crate::tui::App;

pub fn draw_stream_tab<B>(f: &mut Frame<B>, area: Rect, app: &mut App, format: MessageFormat)
where
    B: Backend,
{
    let formatted: Vec<FormattedString> = app
        .notifications
        .iter()
        .map(|notification| format_notification(format, notification))
        .collect();

    let formatted = formatted.iter().flat_map(|n| n.to_tui_color_string());

    List::new(formatted)
        .block(Block::default().borders(Borders::ALL))
        .start_corner(Corner::BottomLeft)
        .render(f, area);
}
