use std::iter::FromIterator;
use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut ch = Vec::from_iter(dna.chars());
    ch.sort();
    ch.dedup();

    let unwanted_dna_chars: Vec<char> = ch.into_iter().filter(|char| {
        *char != 'A' && *char != 'C' && *char != 'G' && *char != 'T'
    }).collect();

    match unwanted_dna_chars.len() {
        0 =>  match nucleotide {
                'A' | 'C' | 'T' | 'G' => Ok(dna.chars().filter(|char| *char == nucleotide).count()),
                _ => Err(nucleotide)
            },
        _ => Err('X')
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm = HashMap::new();

    hm.insert('A', match count('A', dna) {
        Ok(x) => x,
        Err(_) => 0
    });

    hm.insert('C', match count('C', dna) {
        Ok(x) => x,
        Err(_) => 0
    });

    hm.insert('G', match count('G', dna) {
        Ok(x) => x,
        Err(_) => 0
    });

    hm.insert('T', match count('T', dna) {
        Ok(x) => x,
        Err(_) => 0
    });

    match hm.values().fold(0, |sum, x| sum + x) == dna.len() {
        true => Ok(hm),
        _ => Err('X')
    }
}
