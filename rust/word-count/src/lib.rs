use std::collections::HashMap;

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    let words: Vec<String> = sentence.split(|c: char| !c.is_alphanumeric()).filter(|x| !x.is_empty()).map(|x| x.to_lowercase()).collect();
    let mut m: HashMap<String, u32> = HashMap::new();
    for word in words {
        let count  = m.entry(word).or_insert(0);
        *count += 1;
    }

    m
}