use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct DNA {
    sequence: String
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    sequence: String
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut ch = Vec::from_iter(dna.chars());
        ch.sort();
        ch.dedup();

        let unwanted_dna_chars_with_index: Vec<(usize, char)> = ch.into_iter().enumerate().filter(|(_index, char)| {
            *char != 'A' && *char != 'C' && *char != 'G' && *char != 'T'
        }).collect();

        match unwanted_dna_chars_with_index.len() {
            0 =>  Ok(DNA {
                sequence: dna.to_string()
            }),
            _ => Err(unwanted_dna_chars_with_index[0].0)
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            sequence: {
                let rna_sequence_chars = self.sequence.chars().map(|x| {
                    match x {
                        'G' => 'C',
                        'C' => 'G',
                        'T' => 'A',
                        'A' => 'U',
                        _ => 'X'
                     }
                }).collect::<Vec<char>>();

                let rna_strand: String = rna_sequence_chars.into_iter().collect();
                rna_strand
            }
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut ch = Vec::from_iter(rna.chars());
        ch.sort();
        ch.dedup();

        let unwanted_rna_chars: Vec<char> = ch.into_iter().filter(|char| {
            *char != 'C' && *char != 'G' && *char != 'A' && *char != 'U'
        }).collect();

        match unwanted_rna_chars.len() {
            0 =>  Ok(RNA {
                sequence: rna.to_string()
            }),
            _ => Err(0)
        }
    }
}
