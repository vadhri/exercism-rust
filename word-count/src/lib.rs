use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let vector = words.to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric() || *ch == '\'' || *ch == ' ' || *ch == '\n' || *ch == '\t' || *ch == ',')
        .collect::<String>();

    let mut hm: HashMap<String, u32> = HashMap::new();
    let vector_words = vector.split(&[',', ' ', '\n', '\t'][..]).collect::<Vec<_>>();

    for word in vector_words.iter().filter(|word| word.len() > 0).collect::<Vec<_>>() {
        let word_filtered = word.trim_start_matches('\'').trim_end_matches('\'');
        match hm.contains_key(word_filtered) {
            true => {
                if let Some(value) = hm.get_mut(&word_filtered.to_string()) {
                    *value += 1;
                }
            },
            false => {
                hm.insert(word_filtered.to_string(), 1);
            }
        }
    }
    hm
}
