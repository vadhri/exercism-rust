use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_count = input.len();
    let col_count = input[0].len();

    let mut c_hmap: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();
    let mut r_hmap: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();

    for rcnt in 0..row_count {
        let mut min: u64 = 0;
        let mut index: Vec<(usize, usize)> = Vec::new();

        for ccnt in 0..col_count {
            if min < input[rcnt][ccnt] {
                min = input[rcnt][ccnt];
                index.clear(); // new min'
                index.push((rcnt, ccnt));
            } else if min == input[rcnt][ccnt] {
                index.push((rcnt, ccnt));
            }
        }

        if r_hmap.contains_key(&min) {
            let val = r_hmap.get_mut(&min).unwrap();
            for item in index {
                val.push(item);
            }
        } else {
            r_hmap.insert(min, index);
        }
    }

    for ccnt in 0..col_count {
        let mut max: u64 = 10000000;
        let mut index: Vec<(usize, usize)> = Vec::new();

        for rcnt in 0 .. row_count {
            if max > input[rcnt][ccnt] {
                max = input[rcnt][ccnt];
                index.clear(); // new max.
                index.push((rcnt, ccnt));
            } else if max == input[rcnt][ccnt] {
                index.push((rcnt, ccnt));
            }
        }

        if c_hmap.contains_key(&max) {
            let val = c_hmap.get_mut(&max).unwrap();
            for item in index {
                val.push(item);
            }
        } else {
            c_hmap.insert(max, index);
        }
    }

    let mut output = Vec::new();
    for item in r_hmap.keys() {
        if c_hmap.contains_key(item) {
            let columnkeys =  c_hmap.get(item).unwrap();
            let rowkeys =  r_hmap.get(item).unwrap();
            for ckeys in columnkeys {
                for rkeys in rowkeys {
                    if ckeys == rkeys {
                        output.push(*ckeys);
                    }
                }
            }

        }
    }

   output
}
