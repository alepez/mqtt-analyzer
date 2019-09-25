use colored::*;

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

pub fn format_payload_hex(payload: &[u8]) -> String {
    hex::encode(payload)
}

fn escape_only_non_printable(text: String) -> String {
    text.chars().map(|c| match c {
        '!'..='~' => c.to_string(),
        _ => c.escape_debug().to_string(),
    }).collect()
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

pub fn format_message(format: MessageFormat, msg: &rumqtt::Publish) -> String {
    let payload = format_payload(format.payload_format, msg.payload.as_ref());
    let colored_topic = msg.topic_name.blue();
    colored_topic.to_string() + " " + payload.as_str()
}
