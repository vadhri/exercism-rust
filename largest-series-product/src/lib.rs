#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut max: u64 = 0;
    let mut output = Vec::new();
    let mut result:Result<u64, Error> = Ok(max);

    let vector = string_digits.chars()
        .map(|ch| {
            match ch.is_digit(10) {
                true => ch.to_digit(10).unwrap() as u64,
                false => {
                    result = Err(Error::InvalidDigit(ch));
                    u64::max_value()
                }
            }
        })
        .collect::<Vec<_>>();

    if !vector.contains(&u64::max_value()) {
        if span == 0 {
            result = Ok(1);
        } else if span > string_digits.len() {
            result = Err(Error::SpanTooLong);
        } else {
            for w in vector.windows(span) {
                let prod =  w.iter().fold(1, |mul, x| mul * x);

                if max < prod {
                    max = prod;
                    output.clear();
                    output.append(&mut w.clone().to_vec());
                }
            }
            result = Ok(max);
        }
    }

    result
}
