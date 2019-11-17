use std::str::FromStr;
use std::string::ToString;

use rumqtt::Notification;

#[derive(Copy, Clone)]
pub enum PayloadFormat {
    Auto,
    Text,
    Hex,
    Base64,
    Escape,
}

impl FromStr for PayloadFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hex" => Ok(PayloadFormat::Hex),
            "text" | "txt" => Ok(PayloadFormat::Text),
            "base64" | "b64" => Ok(PayloadFormat::Base64),
            "escape" | "esc" => Ok(PayloadFormat::Escape),
            "auto" => Ok(PayloadFormat::Auto),
            _ => Err(()),
        }
    }
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

impl std::fmt::Display for PayloadFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PayloadFormat::Hex => write!(f, "HEX"),
            PayloadFormat::Text => write!(f, "TXT"),
            PayloadFormat::Base64 => write!(f, "B64"),
            PayloadFormat::Escape => write!(f, "ESC"),
            PayloadFormat::Auto => panic!("Auto"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Color {
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
            Color::Secondary => colored::Color::Yellow,
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
            Color::Secondary => tui::style::Color::Yellow,
            Color::OnSecondary => tui::style::Color::Black,
            Color::Error => tui::style::Color::Red,
            Color::OnError => tui::style::Color::White,
        }
    }
}

#[derive(Clone)]
pub struct TokenStyle {
    color: Color,
    background: Color,
}

#[derive(Clone)]
pub struct FormattedToken {
    style: TokenStyle,
    content: String,
}

impl FormattedToken {
    fn new(style: TokenStyle, content: String) -> FormattedToken {
        FormattedToken { style, content }
    }
}

#[derive(Clone)]
pub struct FormattedString(Vec<FormattedToken>);

impl FormattedString {
    pub fn to_color_string(&self) -> String {
        self.0
            .iter()
            .map(|tok| {
                use colored::Colorize;
                let fg: colored::Color = tok.style.color.into();
                let bg: colored::Color = tok.style.background.into();
                format!("{}", tok.content.color(fg).on_color(bg))
            })
            .collect::<Vec<String>>()
            .join(" ")
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
            ' '..='~' => c.to_string(),
            _ => c.escape_debug().to_string(),
        })
        .collect()
}

pub fn format_payload_text(payload: &[u8]) -> (PayloadFormat, String) {
    match String::from_utf8(payload.to_vec()) {
        Result::Ok(text) => (PayloadFormat::Text, escape_only_non_printable(text)),
        _ => (PayloadFormat::Hex, format_payload_hex(payload)),
    }
}

pub fn format_payload_base64(payload: &[u8]) -> String {
    base64::encode(payload)
}

pub fn format_payload_ascii(payload: &[u8]) -> (PayloadFormat, String) {
    match String::from_utf8(payload.to_vec()) {
        Result::Ok(text) => (PayloadFormat::Escape, text.escape_default().to_string()),
        _ => (PayloadFormat::Hex, format_payload_hex(payload)),
    }
}

fn is_printable(payload: &[u8]) -> bool {
    payload.iter().all(|c| match c {
        b' '..=b'~' => true,
        _ => false,
    })
}

fn guess_format(payload: &[u8]) -> PayloadFormat {
    if is_printable(payload) {
        return PayloadFormat::Text;
    }

    PayloadFormat::Escape
}

pub fn format_payload(format: PayloadFormat, payload: &[u8]) -> (PayloadFormat, String) {
    match format {
        PayloadFormat::Hex => (format, format_payload_hex(payload)),
        PayloadFormat::Text => format_payload_text(payload),
        PayloadFormat::Base64 => (format, format_payload_base64(payload)),
        PayloadFormat::Escape => format_payload_ascii(payload),
        PayloadFormat::Auto => {
            let actual_format = guess_format(payload);
            format_payload(actual_format, payload)
        }
    }
}

pub const TOPIC_STYLE: TokenStyle = TokenStyle {
    color: Color::OnPrimary,
    background: Color::Primary,
};

pub const PAYLOAD_STYLE: TokenStyle = TokenStyle {
    color: Color::OnBackground,
    background: Color::Background,
};

pub const FORMAT_STYLE: TokenStyle = TokenStyle {
    color: Color::OnSecondary,
    background: Color::Secondary,
};

pub const NOTIFICATION_STYLE: TokenStyle = TokenStyle {
    color: Color::OnError,
    background: Color::Error,
};

pub fn format_message(format: MessageFormat, msg: &rumqtt::Publish) -> FormattedString {
    let (format, payload) = format_payload(format.payload_format, msg.payload.as_ref());

    FormattedString(vec![
        FormattedToken::new(FORMAT_STYLE, format.to_string()),
        FormattedToken::new(TOPIC_STYLE, msg.topic_name.clone()),
        FormattedToken::new(PAYLOAD_STYLE, payload),
    ])
}

fn format_generic_notification(notification: &Notification) -> FormattedString {
    FormattedString(vec![FormattedToken::new(
        NOTIFICATION_STYLE,
        format!("{:?}", notification),
    )])
}

pub fn format_notification(
    format: MessageFormat,
    notification: &rumqtt::Notification,
) -> FormattedString {
    match notification {
        Notification::Publish(msg) => format_message(format, msg),
        notification => format_generic_notification(notification),
    }
}

impl Into<tui::style::Style> for TokenStyle {
    fn into(self) -> tui::style::Style {
        tui::style::Style::default()
            .fg(self.color.into())
            .bg(self.background.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_payload_hex_non_empty() {
        assert_eq!(format_payload(PayloadFormat::Hex, b"ciao").1, "6369616f");
    }

    #[test]
    fn format_payload_text_non_empty() {
        assert_eq!(format_payload(PayloadFormat::Text, b"ciao").1, "ciao");
    }

    #[test]
    fn format_payload_text_with_special_chars_non_empty() {
        assert_eq!(format_payload(PayloadFormat::Text, b"{ciao?").1, "{ciao?");
        println!(
            "{} == {}",
            format_payload(PayloadFormat::Text, b"{ciao?").1,
            "{ciao?"
        );
    }

    #[test]
    fn format_payload_text_with_unicode_non_empty() {
        assert_eq!(
            format_payload(PayloadFormat::Text, "ciao❤".as_bytes()).1,
            "ciao❤"
        );
    }

    #[test]
    fn format_payload_text_non_utf8() {
        assert_eq!(
            format_payload(PayloadFormat::Text, b"\xf1\xf2\xf4\xf7").1,
            "f1f2f4f7"
        );
    }

    #[test]
    fn format_payload_text_non_printable() {
        assert_eq!(
            format_payload(PayloadFormat::Text, b"\tc\ni\0a\ro").1,
            "\\tc\\ni\\u{0}a\\ro"
        );
    }

    #[test]
    fn format_payload_base64_non_empty() {
        assert_eq!(format_payload(PayloadFormat::Base64, b"ciao").1, "Y2lhbw==");
    }

    #[test]
    fn format_payload_ascii_non_empty() {
        assert_eq!(
            format_payload(PayloadFormat::Escape, "ciao❤".as_bytes()).1,
            "ciao\\u{2764}"
        );
    }
}
