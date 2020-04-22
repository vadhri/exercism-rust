pub fn rotate(input: &str, key: i8) -> String {
    let mut output: String = String::new();

    let rotate_key = match key {
        x if x >= 0 => key,
        _rest => {
            26 + key
        }
    };

    let start_lowercase = b'a';
    let end_lowercase = b'z';

    let lowercase_letters = (start_lowercase..=end_lowercase).collect::<Vec<u8>>();

    let start_uppercase = b'A';
    let end_uppercase = b'Z';

    let uppercase_letters = (start_uppercase..=end_uppercase).collect::<Vec<u8>>();

    for (_index, chr) in input.chars().enumerate() {
        if chr.is_alphabetic() {
            match chr.is_lowercase() {
                true => {
                    let mut circular_iter_lowercase = lowercase_letters.iter().cycle();
                    let distance_from_a = chr as u8 - 'a' as u8;
                    let value = circular_iter_lowercase.nth(distance_from_a as usize + rotate_key as usize).unwrap();

                    output.push(*value as char);
                },
                false => {
                    let mut circular_iter_uppercase = uppercase_letters.iter().cycle();
                    let distance_from_a = chr as u8 - 'A' as u8;
                    let value = circular_iter_uppercase.nth(distance_from_a as usize + rotate_key as usize).unwrap();

                    output.push(*value as char);
                }
            }
        } else {
            output.push(chr);
        }
    }

    output
}
