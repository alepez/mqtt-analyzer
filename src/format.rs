use colored::*;

pub fn format_payload(payload: &[u8]) -> String {
    hex::encode(payload)
}

pub fn format_message(msg: &rumqtt::Publish) -> String {
    let payload = format_payload(msg.payload.as_ref());
    format!("{} {}\n", msg.topic_name.blue(), payload)
}