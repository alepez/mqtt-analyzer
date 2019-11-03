use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, Widget};
use tui::Frame;

use crate::format::MessageFormat;
use crate::tui::notification_list::{Notification, NotificationsList};
use crate::tui::App;

pub fn draw_retain_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, _format: MessageFormat)
where
    B: Backend,
{
    // TODO Use `format` argument
    let retained_messages = app
        .retained_messages
        .iter()
        .map(|(_, notification)| Notification::new(notification));

    NotificationsList::new(retained_messages)
        .block(Block::default().borders(Borders::ALL))
        .start_corner(Corner::BottomLeft)
        .render(f, area);
}
