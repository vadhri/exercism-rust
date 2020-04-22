// Rule 1: If a word begins with a vowel sound, add an "ay" sound to the end of the word. Please note that "xr" and "yt" at the beginning of a word make vowel sounds (e.g. "xray" -> "xrayay", "yttria" -> "yttriaay").
// Rule 2: If a word begins with a consonant sound, move it to the end of the word and then add an "ay" sound to the end of the word. Consonant sounds can be made up of multiple consonants, a.k.a. a consonant cluster (e.g. "chair" -> "airchay").
// Rule 3: If a word starts with a consonant sound followed by "qu", move it to the end of the word, and then add an "ay" sound to the end of the word (e.g. "square" -> "aresquay").
// Rule 4: If a word contains a "y" after a consonant cluster or as the second letter in a two letter word it makes a vowel sound (e.g. "rhythm" -> "ythmrhay", "my" -> "ymay").
use regex::Regex;

pub fn translate(input: &str) -> String {
    let mut output: String = String::new();

    for word in input.split_whitespace() {
        let vowels = Regex::new(r"^[aeiouAEIOU]").unwrap();
        let vowels_xr = Regex::new(r"^xr").unwrap();
        let vowels_yt = Regex::new(r"^yt").unwrap();
        let consonants = Regex::new(r"^[bcdfghjklmnpqrstvwxyz]").unwrap();
        let consonants_qu = Regex::new(r"^[bcdfghjklmnpqrstvwxyz]?qu").unwrap();
        let rule_4 = Regex::new(r"^[bcdfghjklmnpqrstvwxyz]+y").unwrap();

        if vowels.is_match(word) || vowels_xr.is_match(word) || vowels_yt.is_match(word) {
            output.push_str(word);
            output.push_str("ay");
        }  else if consonants_qu.is_match(word) {
            let (first, second) =  word.split_at(word.find("qu").unwrap() + 2);
            output.push_str(second);
            output.push_str(first);
            output.push_str("ay");
        } else if rule_4.is_match(word) {
            let (first, second) = word.split_at(word.find('y').unwrap());
            output.push_str(second);
            output.push_str(first);
            output.push_str("ay");
        } else if consonants.is_match(word) {
            let chrs = word.chars().collect::<Vec<_>>();
            let mut vowel_index = 0;
            for (index, _chr) in chrs.iter().enumerate() {
                vowel_index = match consonants.is_match(&word[index ..]) {
                    true => 0,
                    false => {
                        index
                    }
                };

                if vowel_index > 0 {
                    break;
                }
            }
            if vowel_index > 0 {
                let (first, second) = word.split_at(vowel_index);
                output.push_str(second);
                output.push_str(first);
                output.push_str("ay");
            }
        }
        output.push_str(" ");
    }

    output.trim_end().to_string()
}
