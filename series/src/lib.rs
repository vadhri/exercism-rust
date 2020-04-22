pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    if len > 0 {
        for w in digits.chars().collect::<Vec<char>>().windows(len) {
            out.push(w.iter().collect::<String>());
        }
    } else {
        for _item in 0..=digits.len() {
            out.push("".to_string());
        }
    }

    out
}
