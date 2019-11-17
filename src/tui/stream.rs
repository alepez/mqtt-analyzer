use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, Widget};

use crate::format::MessageFormat;
use crate::tui::App;
use crate::tui::notification_list::{Notification, NotificationsList};

pub fn draw_stream_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, format: MessageFormat)
    where
        B: Backend,
{
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
