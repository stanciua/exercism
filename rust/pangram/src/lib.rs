pub fn is_pangram(sentence: &str) -> bool {
    "abcdefghijklmnopqrstuvwxyz".chars().map(|x| sentence.to_lowercase().contains(x)).all(|x| x)
}