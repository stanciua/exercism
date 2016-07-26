extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;

// Symbol[2]	Description	Bases represented
// A	Adenine	A				1
// C	Cytosine		C
// G	Guanine			G
// T	Thymine				T
// U	Uracil				U
// W	Weak	A			T	2
// S	Strong		C	G
// M	aMino	A	C
// K	Keto			G	T
// R	puRine	A		G
// Y	pYrimidine		C		T
// B	not A (B comes after A)		C	G	T	3
// D	not C (D comes after C)	A		G	T
// H	not G (H comes after G)	A	C		T
// V	not T (V comes after T and U)	A	C	G
// N or -	any Nucleotide (not a gap)	A	C	G	T	4
// Z	Zero					0


fn iupac_encoding_for(symbol: char) -> Option<Vec<char>> {
    let iupac_table: HashMap<char, Vec<char>> = vec![('A', vec!['A']),
                                                     ('C', vec!['C']),
                                                     ('G', vec!['G']),
                                                     ('T', vec!['T']),
                                                     ('U', vec!['U']),
                                                     ('W', vec!['A', 'T']),
                                                     ('S', vec!['C', 'G']),
                                                     ('M', vec!['A', 'C']),
                                                     ('K', vec!['G', 'T']),
                                                     ('R', vec!['A', 'G']),
                                                     ('Y', vec!['C', 'T']),
                                                     ('B', vec!['C', 'G', 'T']),
                                                     ('D', vec!['A', 'G', 'T']),
                                                     ('H', vec!['A', 'C', 'T']),
                                                     ('V', vec!['A', 'C', 'G']),
                                                     ('N', vec!['A', 'C', 'G', 'T'])]
        .into_iter()
        .collect();

    if let Some(v) = iupac_table.get(&symbol) {
        Some(v.clone())
    } else {
        None
    }
}

pub struct Codons<'a> {
    codons: HashMap<&'a str, &'a str>,
}

pub fn parse<'a>(codons: Vec<(&'a str, &'a str)>) -> Codons<'a> {
    Codons { codons: codons.into_iter().collect() }
}

impl<'a> Codons<'a> {
    pub fn name_for(&self, symbol: &str) -> Result<&str, String> {
        if symbol.len() < 3 {
            return Err("symbol is too short!".to_owned());
        }
        if symbol.len() > 3 {
            return Err("symbol is too long!".to_owned());
        }

        let mappings = symbol.chars()
            .map(|ch| (ch, iupac_encoding_for(ch)))
            .collect::<Vec<(char, Option<Vec<_>>)>>();

        // validate symbols
        for &(ref ch, ref map) in &mappings {
            if map.is_none() {
                return Err(format!("{} is not a shorthand!", ch));
            }
        }

        let mut it = mappings.into_iter();
        let ((_, s1), (_, s2), (_, s3)) =
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());

        let codon_combinations = s1.unwrap()
            .into_iter()
            .cartesian_product(s2.unwrap().into_iter())
            .cartesian_product(s3.unwrap().into_iter());

        let codon_combinations_vec: Vec<String> = codon_combinations.into_iter()
            .map(|((s1, s2), s3)| format!("{}{}{}", s1, s2, s3))
            .collect();

        for codon in codon_combinations_vec {
            if let Some(&val) = self.codons.get(&*codon) {
                return Ok(val);
            }
        }

        Err("invalid codon found!".to_owned())
    }
}
