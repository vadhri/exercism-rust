pub fn is_pangram(sentence: &str) -> bool {
    let mut chrs: Vec<char> = sentence.to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphabetic() && ch.is_ascii())
        .collect();

    let lower_case_alphabets: Vec<char> = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

    chrs.sort();
    chrs.dedup(); 

    chrs == lower_case_alphabets
}
