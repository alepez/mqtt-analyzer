use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::{Corner, Rect};
use tui::style::Style;
use tui::widgets::{Block, Widget};

pub struct Notification<'b> {
    content: &'b rumqtt::client::Notification,
}

impl Notification<'_> {
    pub fn new(content: &rumqtt::client::Notification) -> Notification {
        Notification { content }
    }
}

pub struct NotificationsList<'b, L>
where
    L: Iterator<Item = Notification<'b>>,
{
    block: Option<Block<'b>>,
    items: L,
    start_corner: Corner,
}

impl<'b, L> Default for NotificationsList<'b, L>
where
    L: Iterator<Item = Notification<'b>> + Default,
{
    fn default() -> NotificationsList<'b, L> {
        NotificationsList {
            block: None,
            items: L::default(),
            start_corner: Corner::TopLeft,
        }
    }
}

impl<'b, L> NotificationsList<'b, L>
where
    L: Iterator<Item = Notification<'b>>,
{
    pub fn new(items: L) -> NotificationsList<'b, L> {
        NotificationsList {
            block: None,
            items,
            start_corner: Corner::TopLeft,
        }
    }

    pub fn block(mut self, block: Block<'b>) -> NotificationsList<'b, L> {
        self.block = Some(block);
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> NotificationsList<'b, L> {
        self.start_corner = corner;
        self
    }
}

impl<'b, L> Widget for NotificationsList<'b, L>
where
    L: Iterator<Item = Notification<'b>>,
{
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        for (i, item) in self
            .items
            .by_ref()
            .enumerate()
            .take(list_area.height as usize)
        {
            let (x, y) = match self.start_corner {
                Corner::TopLeft => (list_area.left(), list_area.top() + i as u16),
                Corner::BottomLeft => (list_area.left(), list_area.bottom() - (i + 1) as u16),
                // Not supported
                _ => (list_area.left(), list_area.top() + i as u16),
            };
            buf.set_stringn(
                x,
                y,
                format!("{:?}", item.content),
                list_area.width as usize,
                Style::default(),
            );
        }
    }
}
