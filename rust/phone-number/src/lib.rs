pub fn number(num: &str) -> Option<String> {
    let clean_num: String = num.chars().filter(|ch| ch.is_digit(10)).collect();

    match clean_num {
        _ if clean_num.len() == 10 => Some(clean_num),
        _ if clean_num.len() == 11 && clean_num.starts_with("1") => {
            Some(clean_num.chars().skip(1).collect())
        }
        _ => None,

    }
}

pub fn area_code(num: &str) -> Option<String> {
    if let Some(clean_num) = number(num) {
        Some(clean_num.chars().take(3).collect())
    } else {
        None
    }
}

pub fn pretty_print(num: &str) -> String {
    if let Some(clean_num) = number(num) {
        format!("({}) {}-{}",
                area_code(num).unwrap(),
                clean_num.chars().skip(3).take(3).collect::<String>(),
                clean_num.chars().skip(6).take(4).collect::<String>())
    } else {
        "invalid".to_owned()
    }
}
