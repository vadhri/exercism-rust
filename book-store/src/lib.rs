use itertools::Itertools;
use std::collections::BTreeMap;

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut hm: BTreeMap<u32, u32> = BTreeMap::new();
    let mut output_sum = 0;

    let mut output: Vec<Vec<u32>> = Vec::new();

    for book in books {
        match hm.contains_key(book) {
            true => {
                let book_count = hm.get_mut(book).unwrap();
                *book_count += 1;
            }
            false => {
                hm.insert(*book, 1);
            }
        }
    }

    for number_of_permutations in (2..6).rev() {
        let mut nop = number_of_permutations;
        let mut hm_temp = hm.clone();
        let mut hm_temp_mutable = hm.clone();
        let mut kv_nop = number_of_permutations;

        let mut findings: Vec<u32> = Vec::new();

        while kv_nop > 0 {
            nop = kv_nop;
            for c in hm_temp
                .iter()
                .filter(|(_, v)| *v > &0)
                .permutations(nop)
                .unique()
            {
                findings.clear();
                for item in &c {
                    match hm_temp_mutable.get_mut(item.0) {
                        Some(value) => {
                            *value -= 1;
                        }
                        None => {}
                    }
                    findings.push(*item.0);
                }

                output.push(findings.clone());

                let values = hm_temp_mutable.values().collect::<Vec<&u32>>();

                kv_nop = hm_temp_mutable.iter().filter(|(_, v)| *v == &0).count();

                if values.contains(&&0) {
                    break;
                }
            }

            hm_temp.clear();

            for item in &hm_temp_mutable {
                if *item.1 != 0 {
                    hm_temp.insert(*item.0, *item.1);
                }
            }

            hm_temp_mutable.clear();
            hm_temp_mutable = hm_temp.clone();
            kv_nop = hm_temp.keys().len();
        }

        let mut output_temp = 0;

        for combination in &output {
            output_temp += match combination.len() {
                1 => 800,
                2 => 1520,
                3 => 2160,
                4 => 2560,
                5 => 3000,
                _rest => 0
            };
        }

        if output_sum > output_temp || output_sum == 0 {
            output_sum = output_temp
        }

        output.clear();
    }
    output_sum
}
