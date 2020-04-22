pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_digits: Vec<char> = isbn.chars().enumerate()
        .filter(|(index, ch)| {
            ch.is_numeric() || (ch.is_alphabetic() && *ch == 'X' && *index == isbn.len() - 1 as usize)
        })
        .map(|(_, ch)| ch)
        .collect();

    let mut result: bool = isbn_digits.len() == 10;

    if result == true {
        let isbn_value = isbn_digits.iter().enumerate()
            .fold(0, |sum, (index, value)| {
                let val = match value {
                    'X' => 10,
                    rest => rest.to_digit(10).unwrap()
                };
                sum + ((10 - index as u32) * val)
            });

        result = isbn_value % 11 == 0;
    }

    result
}
