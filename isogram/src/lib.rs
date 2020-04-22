pub fn check(candidate: &str) -> bool {
    let mut char_vector: Vec<char> = candidate.to_uppercase().chars()
        .filter(|x| x.is_alphabetic())
        .collect();

    let char_vector_len = char_vector.len();

    char_vector.sort(); 
    char_vector.dedup();

    char_vector_len == char_vector.len()
}
