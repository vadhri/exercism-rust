use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

pub fn encode(key: &str, s: &str) -> Option<String> {
    let mut res: Option<String> = None;
    let mut output: String = String::new();
    let key_char: Vec<char> = key.chars().collect();

    let lowercase_numeric_check = key.chars().filter(|ch| !ch.is_alphabetic() || !ch.is_lowercase()).count() == 0 && key.len() != 0;

    println!("encode : lowercase_numeric_check {:?} {:?}", lowercase_numeric_check,
        key.chars().filter(|ch| !ch.is_alphabetic() || !ch.is_lowercase()).collect::<Vec<char>>());

    if lowercase_numeric_check {
        let key_char_distance: Vec<u8> = key_char.iter().map(|ch| {
            *ch as u8 - 'a' as u8
        }).collect();

        for (index, chr) in s.chars().enumerate() {
            let index_key_distance = index % key_char_distance.len();
            let mut char_transformed = chr as u8 + key_char_distance[index_key_distance];
            println!("{:?}", (index, chr, index_key_distance));

            println!("encode {:?} + {:?}({:?}) = {:?} {:?}", chr, key_char[index_key_distance], key_char_distance[index_key_distance], char_transformed, 'z' as u8);

            char_transformed = match char_transformed {
                x if x > 'z' as u8 => 'a' as u8 + (x - 'z' as u8 - 1),
                rest => rest
            };

            println!("encode {:?} + {:?}({:?}) = {:?}", chr, key_char[index_key_distance], key_char_distance[index_key_distance], char_transformed as char);
            output.push(char_transformed as char);
        }
        res = Some(output);
    }

    res
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let mut res: Option<String> = None;
    let mut output: String = String::new();
    let key_char: Vec<char> = key.chars().collect();

    println!("{:?} {:?}", key, s);

    let lowercase_numeric_check = key.chars().filter(|ch| !ch.is_alphabetic() || !ch.is_lowercase()).count() == 0 && key.len() != 0;

    println!("lowercase_numeric_check {:?} {:?}", lowercase_numeric_check,
        key.chars().filter(|ch| !ch.is_alphabetic() || !ch.is_lowercase()).collect::<Vec<char>>());

    if lowercase_numeric_check {
        let key_char_distance: Vec<u8> = key_char.iter().map(|ch| {
            *ch as u8 - 'a' as u8
        }).collect();

        for (index, chr) in s.chars().enumerate() {
            let index_key_distance = index % key_char_distance.len();
            let mut char_transformed = chr as u8 - key_char_distance[index_key_distance];

            println!("{:?} + {:?}({:?}) = {:?} {:?}", chr, key_char[index_key_distance], key_char_distance[index_key_distance], char_transformed, 'a' as u8);

            char_transformed = match char_transformed {
                x if x < 'a' as u8 => 'z' as u8 - ('a' as u8 - x - 1),
                rest => rest
            };

            println!("{:?} + {:?}({:?}) = {:?}", chr, key_char[index_key_distance], key_char_distance[index_key_distance], char_transformed as char);
            output.push(char_transformed as char);
        }
        res = Some(output);
    }

    res
}

pub fn encode_random(s: &str) -> (String, String) {
    let rng = thread_rng();
    let random_string: String = rng.sample_iter(Alphanumeric).filter(|ch| ch.is_alphanumeric() && ch.is_lowercase()).take(100).collect();

    let rs = random_string.clone();

    println!("random string {:?}", random_string);

    (random_string, encode(&rs, s).unwrap())
}
