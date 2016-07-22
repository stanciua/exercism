use std::collections::HashMap;

pub fn count(nucleotide: char, dna_string: &str) -> i32 {
    dna_string.chars().filter(|x| *x == nucleotide).count() as i32
}

pub fn nucleotide_counts(dna_string: &str) -> HashMap<char, usize> {
    "ACGT".chars().map(|x| (x, count(x, dna_string) as usize)).collect()
}