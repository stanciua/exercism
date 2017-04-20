use std::collections::HashMap;
use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    let encode_cipher = "0123456789abcdefghijklmnopqrstuvwxyz"
        .chars()
        .zip("0123456789zyxwvutsrqponmlkjihgfedcba".chars())
        .collect::<HashMap<_, _>>();

    let encoded_text = input.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| encode_cipher[&c])
        .collect::<String>();

    let mut rslt = String::new();
    let mut encoded_text_slice = encoded_text.as_str();
    while encoded_text_slice.len() >= 5 {
        rslt.push_str(&encoded_text_slice[..5]);
        rslt.push(' ');
        encoded_text_slice = &encoded_text_slice[5..];
    }

    let remaining_bytes = encoded_text_slice.len();
    if remaining_bytes > 0 {
        rslt.push_str(&encoded_text_slice[..remaining_bytes]);
    } else {
        rslt.pop();
    }

    rslt
}

pub fn decode(input: &str) -> String {
    let decode_cipher = "0123456789zyxwvutsrqponmlkjihgfedcba"
        .chars()
        .zip("0123456789abcdefghijklmnopqrstuvwxyz".chars())
        .collect::<HashMap<_, _>>();

    input.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| decode_cipher[&c])
        .collect::<String>()
}
