use colored::*;

pub enum PayloadFormat {
    Text,
    Hex,
    Base64,
}

pub struct MessageFormat {
    payload_format: PayloadFormat,
}

impl MessageFormat {
    pub fn default() -> Self {
        MessageFormat { payload_format: PayloadFormat::Text }
    }
}

pub fn format_payload_hex(payload: &[u8]) -> String {
    hex::encode(payload)
}

pub fn format_payload_text(payload: &[u8]) -> String {
    match String::from_utf8(payload.to_vec()) {
        Result::Ok(text) => text,
        _ => format_payload_hex(payload),
    }
}

pub fn format_payload_base64(payload: &[u8]) -> String {
    base64::encode(payload)
}

pub fn format_payload(format: PayloadFormat, payload: &[u8]) -> String {
    match format {
        PayloadFormat::Hex => format_payload_hex(payload),
        PayloadFormat::Text => format_payload_text(payload),
        PayloadFormat::Base64 => format_payload_base64(payload),
    }
}

pub fn format_message(format: MessageFormat, msg: &rumqtt::Publish) -> String {
    let payload = format_payload(format.payload_format, msg.payload.as_ref());
    format!("{} {}\n", msg.topic_name.blue(), payload)
}
