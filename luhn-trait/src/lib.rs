pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

pub fn is_valid(input_string: String) -> bool {
    let mut result = false;

    let code = input_string.as_str().replace(" ", "");
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

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for String {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

impl<'a> Luhn for usize {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}
