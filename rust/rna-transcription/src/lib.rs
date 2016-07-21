// Rna Transcription
// Write a program that, given a DNA strand, returns its RNA complement (per RNA transcription).

// Both DNA and RNA strands are a sequence of nucleotides.

// The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).

// The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).

// Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:

// G -> C
// C -> G
// T -> A
// A -> U
#[derive(Debug, PartialEq, Eq)]
pub struct RibonucleicAcid{
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeoxyribonucleicAcid{
    strand: String,
}

impl RibonucleicAcid{
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid { strand: s.to_owned()}
    }
}

impl DeoxyribonucleicAcid{
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: s.to_owned()}
    }
    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid {strand: self.strand.chars().map(DeoxyribonucleicAcid::dna_to_rna_map_table).collect::<String>() }
               
    }
    fn dna_to_rna_map_table(ch: char) -> char {
        match ch {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => panic!("Unsupported value!")
        }
    }
}