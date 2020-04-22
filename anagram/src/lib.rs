use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();

    let mut word_chars = word.to_lowercase().chars().collect::<Vec<_>>();
    word_chars.sort();

    for anagram in possible_anagrams {
        let mut possible_word_chars = anagram.to_lowercase().chars().collect::<Vec<_>>();
        possible_word_chars.sort();

        if anagram.to_lowercase() != word.to_lowercase() {
            if possible_word_chars == word_chars {
                output.insert(*anagram);
            }
        }
    }

    output
}
