pub struct Luhn {
    input_string: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut result = false;

        let code = self.input_string.as_str().replace(" ", "");
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        Luhn {
            input_string: input.to_string()
        }
    }
}

impl<'a> From<std::string::String> for Luhn {
    fn from(input: std::string::String) -> Self {
        Luhn {
            input_string: input.clone()
        }
    }
}

impl<'a> From<u8> for Luhn {
    fn from(input: u8) -> Self {
        Luhn {
            input_string: format!("{:?}", input)
        }
    }
}

impl<'a> From<u16> for Luhn {
    fn from(input: u16) -> Self {
        Luhn {
            input_string: format!("{:?}", input)
        }
    }
}

impl<'a> From<u32> for Luhn {
    fn from(input: u32) -> Self {
        Luhn {
            input_string: format!("{:?}", input)
        }
    }
}

impl<'a> From<u64> for Luhn {
    fn from(input: u64) -> Self {
        Luhn {
            input_string: format!("{:?}", input)
        }
    }
}

impl<'a> From<usize> for Luhn {
    fn from(input: usize) -> Self {
        Luhn {
            input_string: format!("{:?}", input)
        }
    }
}
