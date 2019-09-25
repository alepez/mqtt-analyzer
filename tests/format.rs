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
fn format_payload_text_with_special_chars_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Text, b"{ciao?"), "{ciao?");
}

#[test]
fn format_payload_text_with_unicode_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Text, "ciao❤".as_bytes()), "ciao❤");
}

#[test]
fn format_payload_text_non_utf8() {
    assert_eq!(
        format_payload(PayloadFormat::Text, b"\xf1\xf2\xf4\xf7"),
        "f1f2f4f7"
    );
}

#[test]
#[ignore]
fn format_payload_text_non_printable() {
    assert_eq!(
        format_payload(PayloadFormat::Text, b"\tc\ni\0a\ro"),
        "\\tc\\ni\\u{0}a\\ro"
    );
}

#[test]
fn format_payload_base64_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Base64, b"ciao"), "Y2lhbw==");
}

#[test]
fn format_payload_ascii_non_empty() {
    assert_eq!(format_payload(PayloadFormat::Escape, "ciao❤".as_bytes(),), "ciao\\u{2764}");
}
