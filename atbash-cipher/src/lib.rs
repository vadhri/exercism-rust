/// "Encipher" with the Atbash cipher.
use std::collections::HashMap;

#[macro_use(lazy_static)]
extern crate lazy_static;

lazy_static!{
    static ref MAP: HashMap<char, char> = [
        ('a', 'z'),
        ('b', 'y'),
        ('c', 'x'),
        ('d', 'w'),
        ('e', 'v'),
        ('f', 'u'),
        ('g', 't'),
        ('h', 's'),
        ('i', 'r'),
        ('j', 'q'),
        ('k', 'p'),
        ('l', 'o'),
        ('m', 'n'),
        ('n', 'm'),
        ('o', 'l'),
        ('p', 'k'),
        ('q', 'j'),
        ('r', 'i'),
        ('s', 'h'),
        ('t', 'g'),
        ('u', 'f'),
        ('v', 'e'),
        ('w', 'd'),
        ('x', 'c'),
        ('y', 'b'),
        ('z', 'a')
    ].iter().copied().collect();
}

lazy_static!{
    static ref MAP_REVERSE: HashMap<char, char> = [
        ('z', 'a'),
        ('y', 'b'),
        ('x', 'c'),
        ('w', 'd'),
        ('v', 'e'),
        ('u', 'f'),
        ('t', 'g'),
        ('s', 'h'),
        ('r', 'i'),
        ('q', 'j'),
        ('p', 'k'),
        ('o', 'l'),
        ('n', 'm'),
        ('m', 'n'),
        ('l', 'o'),
        ('k', 'p'),
        ('j', 'q'),
        ('i', 'r'),
        ('h', 's'),
        ('g', 't'),
        ('f', 'u'),
        ('e', 'v'),
        ('d', 'w'),
        ('c', 'x'),
        ('b', 'y'),
        ('a', 'z')
    ].iter().copied().collect();
}

pub fn find_char_encoded(input: char) -> char {
    *MAP.get(&input).unwrap()
}

pub fn find_char_encoded_reverse(input: char) -> char {
    *MAP_REVERSE.get(&input).unwrap()
}

pub fn encode(plain: &str) -> String {
    let mut output: String = String::new();

    for (index, chr) in plain.to_lowercase().chars().filter(|x| x.is_alphanumeric()).enumerate() {
        if chr.is_numeric() {
            output.push(chr);
        } else {
            output.push(find_char_encoded(chr));
        }

        if index != 0 && (index + 1) % 5 == 0 {
            output.push(' ');
        }
    }

    output.trim_end().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut output: String = String::new();

    for chr in cipher.to_lowercase().chars().filter(|x| x.is_alphanumeric()) {
        if chr.is_numeric() { 
            output.push(chr);
        } else {
            output.push(find_char_encoded_reverse(chr));
        }
    }

    output
}
