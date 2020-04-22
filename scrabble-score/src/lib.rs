/// Compute the Scrabble score for a word.

use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
    let mut hm = HashMap::new();

    hm.insert('a', 1);
    hm.insert('b', 3);
    hm.insert('c', 3);
    hm.insert('d', 2);
    hm.insert('e', 1);
    hm.insert('f', 4);
    hm.insert('g', 2);
    hm.insert('h', 4);
    hm.insert('i', 1);
    hm.insert('j', 8);
    hm.insert('k', 5);
    hm.insert('l', 1);
    hm.insert('m', 3);
    hm.insert('n', 1);
    hm.insert('o', 1);
    hm.insert('p', 3);
    hm.insert('q', 10);
    hm.insert('r', 1);
    hm.insert('s', 1);
    hm.insert('t', 1);
    hm.insert('u', 1);
    hm.insert('v', 4);
    hm.insert('w', 4);
    hm.insert('x', 8);
    hm.insert('y', 4);
    hm.insert('z', 10);

    word.to_lowercase().chars().fold(0, |sum, x| {
        match hm.get(&x) {
            None => sum,
            Some(p) => sum + p
        }
    })
}
