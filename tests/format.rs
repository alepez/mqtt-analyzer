use mqtt_analyzer::format::*;

#[test]
fn format_payload_hex_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Hex, b"ciao"), "6369616f");
}

#[test]
fn format_payload_text_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Text, b"ciao"), "ciao");
}

#[test]
fn format_payload_text_non_utf8() {
    assert_eq!(format_payload(PayloadFormat::Text, b"\xf1\xf2\xf4\xf7"), "f1f2f4f7");
}

#[test]
fn format_payload_base64_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Base64, b"ciao"), "Y2lhbw==");
}