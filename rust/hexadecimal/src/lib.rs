fn hex_char_to_int(hex_ch: char) -> Option<i32> {
    match hex_ch {
        '0' => Some(0x0),
        '1' => Some(0x1),
        '2' => Some(0x2),
        '3' => Some(0x3),
        '4' => Some(0x4),
        '5' => Some(0x5),
        '6' => Some(0x6),
        '7' => Some(0x7),
        '8' => Some(0x8),
        '9' => Some(0x9),
        'a' => Some(0xA),
        'b' => Some(0xB),
        'c' => Some(0xC),
        'd' => Some(0xD),
        'e' => Some(0xE),
        'f' => Some(0xF),
        _ => None
    }
}

pub fn hex_to_int(hex_string: &str) -> Option<i32> {
    let hex_str_lowered = hex_string.to_lowercase();
    let lgth = hex_str_lowered.len();

    let mut num = 0;
    for (idx, ch) in hex_str_lowered.chars().enumerate() {
        if let Some(ch_int) = hex_char_to_int(ch) {
            num += ch_int * (16 as i32).pow((lgth - idx - 1) as u32);
        } else {
            return None;
        }
    }
    Some(num)
    
}
