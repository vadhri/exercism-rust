/// Check a Luhn checksum.
pub fn is_valid(code_orig: &str) -> bool {
    let mut result = false;
    
    let code = code_orig.replace(" ", "");
    if code.len() > 1 {
        if code.chars().filter(|ch| !ch.is_digit(10)).count() == 0usize {
            result = code.replace(" ", "").chars()
                .rev()
                .filter(|ch| ch.is_digit(10))
                .map(|ch| ch.to_digit(10).unwrap())
                .enumerate()
                .map(|(index, digit)| {
                    if index % 2 != 0 {
                        match digit * 2 {
                            value if value >= 10 => value - 9,
                            rest => rest
                        }
                    } else {
                        digit
                    }
                })
                .fold (0, |sum, v| sum + v) % 10 == 0;
        }
    }

    result
}
