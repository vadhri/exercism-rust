use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();

    for (score, vector) in h {
        for iter in vector {
            let mut char = iter.clone();
            char.make_ascii_lowercase();
            output.insert(char , *score);
        }
    }
    output 
}
