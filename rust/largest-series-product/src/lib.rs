pub fn lsp(series: &str, digits: u32) -> Result<u32, ()> {
    // special case: number of digits is 0 return 1
    if digits == 0 {
        return Ok(1);
    }

    if (series.len() as u32) < digits {
        return Err(());
    }

    let mut slice = &series[..];
    let mut max = 0;
    while (slice.len() as u32) >= digits {
        let mut p = 1;
        for ch in slice.chars().take(digits as usize) {
            if let Some(digit) = ch.to_digit(10) {
                p *= digit;
            } else {
                return Err(());
            }
        }
        if p > max {
            max = p;
        }
        slice = &slice[1..];
    }

    Ok(max)
}
