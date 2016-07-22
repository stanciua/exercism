pub fn hamming_distance(strand1: &str, strand2: &str) -> Result<i32, &'static str> {
    if strand1.len() != strand2.len() {
        return Err("inputs of different length");
    }
    Ok(strand1.chars().zip(strand2.chars()).filter(|&(a, b)| a != b).count() as i32)

}