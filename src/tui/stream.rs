use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, Widget};
use tui::Frame;

use crate::format::MessageFormat;
use crate::tui::notification_list::{Notification, NotificationsList};
use crate::tui::App;

pub fn draw_stream_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, format: MessageFormat)
where
    B: Backend,
{
    // TODO Use `format` argument
    let notifications = app
        .notifications
        .iter()
        .map(|notification| Notification::new(notification));

    NotificationsList::new(notifications)
        .format(format.payload_format)
        .block(Block::default().borders(Borders::ALL))
        .start_corner(Corner::BottomLeft)
        .render(f, area);
}
