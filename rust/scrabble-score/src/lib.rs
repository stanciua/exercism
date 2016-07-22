fn letter_value(letter: char) -> i32 {
    match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' |
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'D' | 'G' | 'd' | 'g' => 2,
        'B' | 'C' | 'M' | 'P' | 'b' | 'c' | 'm' | 'p' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' | 'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'K' | 'k' => 5,
        'J' | 'X' | 'j' | 'x' => 8,
        'Q' | 'Z' | 'q' | 'z' => 10,
        _ => 0
    }
}

pub fn score(word: &str) -> i32 {
    word.chars().fold(0, |acc, x| acc + letter_value(x))
}
