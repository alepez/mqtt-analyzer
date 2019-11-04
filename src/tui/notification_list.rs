use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::{Corner, Rect};
use tui::style::Style;
use tui::widgets::{Block, Widget};

use crate::format::*;

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
    format: PayloadFormat,
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
            format: PayloadFormat::Hex,
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
            format: PayloadFormat::Hex,
        }
    }

    pub fn format(mut self, format: PayloadFormat) -> NotificationsList<'b, L> {
        self.format = format;
        self
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

fn draw_generic_notification(
    notification: &rumqtt::Notification,
    buf: &mut Buffer,
    x: u16,
    y: u16,
    width: usize,
) {
    buf.set_stringn(x, y, format!("{:?}", notification), width, Style::default());
}

fn draw_publish_notification(
    msg: &mqtt311::Publish,
    buf: &mut Buffer,
    x: u16,
    y: u16,
    width: usize,
    format: PayloadFormat,
) {
    let format_str = format.to_string();
    let topic = msg.topic_name.as_str();
    let payload = msg.payload.as_slice();
    let formatted_payload = format_payload(format, payload);

    let mut offset: u16 = 0;
    buf.set_stringn(x + offset, y, &format_str, width, FORMAT_STYLE.into());
    offset += (format_str.len() + 1) as u16;
    buf.set_stringn(x + offset, y, topic, width, TOPIC_STYLE.into());
    offset += (topic.len() + 1) as u16;
    buf.set_stringn(
        x + offset,
        y,
        formatted_payload,
        width,
        PAYLOAD_STYLE.into(),
    );
}

fn draw_notification(
    notification: &Notification,
    buf: &mut Buffer,
    x: u16,
    y: u16,
    width: usize,
    format: PayloadFormat,
) {
    match notification.content {
        rumqtt::Notification::Publish(a) => draw_publish_notification(a, buf, x, y, width, format),
        a => draw_generic_notification(a, buf, x, y, width),
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
            draw_notification(&item, buf, x, y, list_area.width as usize, self.format);
        }
    }
}
