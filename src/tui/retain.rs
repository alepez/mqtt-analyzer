use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, Widget};

use crate::format::MessageFormat;
use crate::tui::App;
use crate::tui::notification_list::{Notification, NotificationsList};

pub fn draw_retain_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, format: MessageFormat)
    where
        B: Backend,
{
    let retained_messages = app
        .retained_messages
        .iter()
        .map(|(_, notification)| Notification::new(notification));

    NotificationsList::new(retained_messages)
        .format(format.payload_format)
        .block(Block::default().borders(Borders::ALL))
        .start_corner(Corner::TopLeft)
        .render(f, area);
}
