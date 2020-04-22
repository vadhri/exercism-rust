#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
    Unknown
}

pub fn get_value(num: &[char]) -> u64 {
    let mut sum: u64 = 0;

    for (index, item) in num.iter().enumerate() {
        sum += 2_u64.pow(index as u32) as u64 * (*item).to_digit(2).unwrap() as u64;
    }

    sum
}

pub fn get_value_rev(num: &[char]) -> u64 {
    let mut sum: u64 = 0;

    for (index, item) in num.iter().rev().enumerate() {
        sum += 2_u64.pow(index as u32) as u64 * (*item).to_digit(2).unwrap() as u64;
    }

    sum
}
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for item in values.iter() {
        let mut output_temp: Vec<u8> = Vec::new();
        let mut bit_string = "".to_string();
        let bit_string_value = format!("{:08b}", item);

        bit_string.push_str(&bit_string_value.chars().collect::<String>());
        let vec_slices = bit_string.chars().rev().collect::<Vec<_>>();

        for (index, c) in vec_slices.chunks(7).enumerate() {
            let value = get_value(c) as u8;

            match index {
                0 => output_temp.push(value),
                _ => {
                    if index == vec_slices.chunks(7).len() - 1 && value == 0 {
                        println!("Avoided - last zero !");
                    } else {
                        output_temp.push(value|128);
                    }
                }
            }
        }
        output_temp.reverse();
        for value in  output_temp {
            output.push(value);
        }
    }
    // output.reverse();
    output
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut bit_string = "".to_string();
    let mut output: Vec<u32> = Vec::new();
    let mut res: Result<Vec<u32>, Error> = Err(Error::Unknown);

    for (_index, item) in bytes.iter().enumerate() {
        let bit_string_value = format!("{:08b}", item );
        bit_string.push_str(&bit_string_value.chars().collect::<String>());
    }

    if bit_string.len() > 0 {
        let vec_slices = bit_string.chars().collect::<Vec<_>>();
        let mut array_value_bytes: Vec<char> = Vec::new();

        for (index, c) in vec_slices.chunks(8).rev().enumerate() {
            if index == 0 && c[0] != '0' {
                res = Err(Error::IncompleteNumber);
                break;
            }
            if c[0] == '0' && array_value_bytes.len() > 0 {
                output.push(get_value_rev(&array_value_bytes) as u32);
                array_value_bytes.clear();
            }

            for (index, p) in c.iter().rev().enumerate() {
                if index != c.len() - 1 {
                    array_value_bytes.insert(0, *p);
                }
                if get_value_rev(&array_value_bytes) > u32::max_value() as u64 {
                    res = Err(Error::Overflow);
                    break;
                }
            }

            if res == Err(Error::Overflow) {
                break;
            }
        }

        if res != Err(Error::IncompleteNumber) && res != Err(Error::Overflow) {
            output.push(get_value_rev(&array_value_bytes) as u32);
            output.reverse();

            res = Ok(output);
        }
    } else {
        res = Err(Error::Overflow);
    }

    res
}
