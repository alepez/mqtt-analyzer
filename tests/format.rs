use mqtt_analyzer::format::{format_payload_hex};

#[test]
fn format_payload_hex_non_empty() {
    assert_eq!(format_payload_hex(b"ciao"), "6369616f");
}