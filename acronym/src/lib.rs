use inflector::cases::camelcase::is_camel_case;

pub fn abbreviate(phrase: &str) -> String {
    // Filter out any special character requirements like - to be treated as space to consider abbrevation.
    let phrase_filtered = phrase.replace("-", " ");

    // Split word by space.
    let word_iter = phrase_filtered.split_whitespace();
    let mut chars = Vec::new();

    for item in word_iter {
        // Filter out non-alphabetic chars like - or any other special characters
        let mut char_iter = item.chars().filter(|x| x.is_alphabetic());

        let char = match char_iter.next() {
            Some(x) => x,
            None => ' '
        };

        // Error corner case when the word only contains special characters. If we need to check the length of iter, we need collect() above.
        if char == ' ' {
            continue;
        }

        // Push the first letter that is an alphabet in the word.
        chars.push(char);

        // Combine the string back again from iterator to check if the rest of the string is camelcase - special treatment for HyperText usecase.
        let residue_chars_vec = char_iter.collect::<Vec<_>>();
        let residue_string: String = residue_chars_vec.into_iter().collect();

        // Check for camecase an dif if camelcase, filterout the rest of the uppercase leters to form a string.
        if is_camel_case(&residue_string) {
            let residue_string_camecase_chars: String = residue_string.chars().filter(|ch| ch.is_uppercase()).collect();

            if residue_string_camecase_chars.len() > 0 {
                chars.extend(residue_string_camecase_chars.chars());
            }
        }
    }

    // collect all the characters we have been accumlating, convert to string and to upper case. 
    let result = chars.into_iter().collect::<Vec<char>>();
    let result_string: String = result.into_iter().collect();

    result_string.to_uppercase()
}
