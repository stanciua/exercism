pub fn reply(sentence: &str) -> &str {
    match sentence {
        _ if sentence.ends_with('?') => "Sure.",
        _ if sentence.find(char::is_lowercase) != None => "Whatever.",
        "" => "Fine. Be that way!",
        _ => "Whoa, chill out!"
    }
}