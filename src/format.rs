use rumqtt::Notification;

#[derive(Copy, Clone)]
pub enum PayloadFormat {
    Text,
    Hex,
    Base64,
    Escape,
}

#[derive(Copy, Clone)]
pub struct MessageFormat {
    pub payload_format: PayloadFormat,
}

impl MessageFormat {
    pub fn default() -> Self {
        MessageFormat {
            payload_format: PayloadFormat::Text,
        }
    }
}

#[derive(Copy, Clone)]
enum Color {
    Background,
    OnBackground,
    Primary,
    OnPrimary,
    Secondary,
    OnSecondary,
    Error,
    OnError,
}

impl Into<colored::Color> for Color {
    fn into(self) -> colored::Color {
        match self {
            Color::Background => colored::Color::Black,
            Color::OnBackground => colored::Color::White,
            Color::Primary => colored::Color::Blue,
            Color::OnPrimary => colored::Color::Black,
            Color::Secondary => colored::Color::BrightGreen,
            Color::OnSecondary => colored::Color::Black,
            Color::Error => colored::Color::Red,
            Color::OnError => colored::Color::White,
        }
    }
}

impl Into<tui::style::Color> for Color {
    fn into(self) -> tui::style::Color {
        match self {
            Color::Background => tui::style::Color::Black,
            Color::OnBackground => tui::style::Color::White,
            Color::Primary => tui::style::Color::Blue,
            Color::OnPrimary => tui::style::Color::Black,
            Color::Secondary => tui::style::Color::LightGreen,
            Color::OnSecondary => tui::style::Color::Black,
            Color::Error => tui::style::Color::Red,
            Color::OnError => tui::style::Color::White,
        }
    }
}

struct TokenStyle {
    color: Color,
    background: Color,
}

pub struct FormattedToken {
    style: TokenStyle,
    content: String,
}

impl FormattedToken {
    fn new(style: TokenStyle, content: String) -> FormattedToken {
        FormattedToken { style, content }
    }
}

pub struct FormattedString(Vec<FormattedToken>);

impl FormattedString {
    pub fn to_color_string(&self) -> String {
        self.0
            .iter()
            .map(|tok| {
                use colored::{Color, Colorize};
                let fg: colored::Color = tok.style.color.into();
                let bg: colored::Color = tok.style.background.into();
                format!("{}", tok.content.color(fg).on_color(bg))
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn to_tui_color_string(&self) -> Vec<tui::widgets::Text> {
        self.0
            .iter()
            .map(|tok| {
                use tui::style::{Color, Modifier, Style};
                use tui::widgets::Text;
                let fg: Color = tok.style.color.into();
                let bg: Color = tok.style.background.into();
                let style = Style {
                    fg,
                    bg,
                    modifier: Modifier::empty(),
                };
                Text::styled(tok.content.clone(), style)
            })
            .collect()
    }
}

impl ToString for FormattedString {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|tok| tok.content.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub fn format_payload_hex(payload: &[u8]) -> String {
    hex::encode(payload)
}

fn escape_only_non_printable(text: String) -> String {
    text.chars()
        .map(|c| match c {
            '!'..='~' => c.to_string(),
            _ => c.escape_debug().to_string(),
        })
        .collect()
}

pub fn format_payload_text(payload: &[u8]) -> String {
    match String::from_utf8(payload.to_vec()) {
        Result::Ok(text) => escape_only_non_printable(text),
        _ => format_payload_hex(payload),
    }
}

pub fn format_payload_base64(payload: &[u8]) -> String {
    base64::encode(payload)
}

pub fn format_payload_ascii(payload: &[u8]) -> String {
    match String::from_utf8(payload.to_vec()) {
        Result::Ok(text) => text.escape_default().to_string(),
        _ => format_payload_hex(payload),
    }
}

pub fn format_payload(format: PayloadFormat, payload: &[u8]) -> String {
    match format {
        PayloadFormat::Hex => format_payload_hex(payload),
        PayloadFormat::Text => format_payload_text(payload),
        PayloadFormat::Base64 => format_payload_base64(payload),
        PayloadFormat::Escape => format_payload_ascii(payload),
    }
}

pub fn format_message(format: MessageFormat, msg: &rumqtt::Publish) -> FormattedString {
    let payload = format_payload(format.payload_format, msg.payload.as_ref());

    FormattedString(vec![
        FormattedToken::new(
            TokenStyle {
                color: Color::OnPrimary,
                background: Color::Primary,
            },
            msg.topic_name.clone(),
        ),
        FormattedToken::new(
            TokenStyle {
                color: Color::OnBackground,
                background: Color::Background,
            },
            payload,
        ),
    ])
}

pub fn format_notification(
    format: MessageFormat,
    notification: &rumqtt::Notification,
) -> FormattedString {
    match notification {
        Notification::Publish(msg) => format_message(format, msg),
        n => FormattedString(vec![FormattedToken::new(
            TokenStyle {
                color: Color::OnError,
                background: Color::Error,
            },
            format!("{:?}", n),
        )]),
    }
}
