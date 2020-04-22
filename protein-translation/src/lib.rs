use std::collections::HashMap;

pub struct CodonsInfo {
    pairs: Box<HashMap<&'static str, &'static str>>
}

impl CodonsInfo {
    pub fn name_for(&self, codon: &str) -> Option<& str> {
        match self.pairs.get(codon) {
            Some(x) => Some(&x),
            _ => None
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<& str>> {
        let mut output: Option<Vec<&str>> = None;
        let mut output_vec: Vec<&str> = Vec::new();

        let rna_chars = rna.chars().collect::<Vec<_>>();
        let rna_chars_count_valid = rna_chars.len() % 3 == 0;

        if rna_chars_count_valid {
            for codon_chars in rna_chars.chunks(3) {

                let codon: String = codon_chars.iter().collect();
                match self.name_for(&codon) {
                    Some(x) => {
                        if x != "stop codon" {
                            output_vec.push(x)
                        } else {
                            break;
                        }
                    },
                    None => ()
                }
            }
            if output_vec.len() > 0 {
                output = Some(output_vec)
            }
        }

        output
    }
}

pub fn parse<>(pairs: std::vec::Vec<(&'static str, &'static str)>) -> CodonsInfo {
    let mut hm = Box::new(HashMap::new());

    for p in pairs {
        hm.insert(p.0, p.1);
    }

    CodonsInfo {
        pairs: hm
    }
}
