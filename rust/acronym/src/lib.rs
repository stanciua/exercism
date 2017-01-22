pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .map(|w| w.split('-').collect::<Vec<_>>())
        .flat_map(|v| v)
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .map(|w| w.chars().filter(|c| c.is_uppercase()).collect::<String>())
        .map(|f| {
            match f.len() {
                2 => f.chars().take(2).collect::<String>(),
                _ => f.chars().take(1).collect::<String>(),
            }
        })
        .collect::<String>()
}
