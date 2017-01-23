pub fn is_valid(num: &str) -> bool {
    if num.len() <= 1 {
        return false;
    }
    let no_whitespace = num.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if !no_whitespace.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    no_whitespace.chars()
        .map(|c| c.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .map(|(idx, d)| {
            let mut p = d;
            if idx >= 1 && idx % 2 != 0 {
                p *= 2;
                if p > 9 {
                    p -= 9;
                }
            }
            p
        })
        .sum::<u32>() % 10 == 0

}
