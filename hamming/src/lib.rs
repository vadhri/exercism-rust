pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let mut count: u32 = 0;
    let mut result: Option<usize> = None;

    if s1_chars.len() == s2_chars.len() {
        for x in  0..s1_chars.len() {
            if s1_chars[x] != s2_chars[x] {
                count += 1;
            }
        }

        result = Some(count as usize)
    }

    result
}
