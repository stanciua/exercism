pub fn sum_of_multiples(num: i32, multiples: &[i32]) -> i32 {
    (1..num).into_iter().filter(|n| multiples.iter().map(|nn| n % nn == 0).any(|nnn| nnn)).sum()
}
