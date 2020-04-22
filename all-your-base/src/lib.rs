#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let filter_invalid_base_values: Vec<&u32> = number.iter()
                .filter(|val| **val >= from_base)
                .collect();

    let value = number.iter().rev()
        .enumerate()
        .fold(0, |sum, (index, digit)| {
            sum + (digit * from_base.pow(index as u32))
        });

    fn to_binary(mut decimal: u32, base: u32) -> Result<Vec<u32>, Error> {
        let res: Result<Vec<u32>, Error>;
        let mut bit_vector = Vec::new();

        if decimal == 0 {
            res = Ok(vec![]);
        } else {
            while decimal > 0 {
                bit_vector.push(decimal % base);
                decimal /= base;
            }
            bit_vector.reverse();
            res = Ok(bit_vector);
        }
        res
    }

    if filter_invalid_base_values.len() == 0 {
        match from_base {
            x if x == 0 || x == 1 => Err(Error::InvalidInputBase),
            _ => match to_base {
                x if x == 0 || x == 1 => Err(Error::InvalidOutputBase),
                _ => to_binary(value, to_base)
            }
        }
    } else {
        Err(Error::InvalidDigit(*filter_invalid_base_values[0]))
    }
}
