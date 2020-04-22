#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
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

    println!("get_value_rev -> {:?} {:?}", num, num.len());

    for (index, item) in num.iter().rev().enumerate() {
        sum += 2_u64.pow(index as u32) as u64 * (*item).to_digit(2).unwrap() as u64;
    }

    sum
}
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bit_string = "".to_string();
    let mut output: Vec<u8> = Vec::new();

    for item in values.iter() {
        let bit_string_value = format!("{:08b}", item);
        println!("to_bytes: ({:x?}) bit_string_value -> {:?}", item, bit_string_value);
        bit_string.push_str(&bit_string_value.chars().collect::<String>());
     }

    println!("to_bytes: full bit string -> {:?}", bit_string);

    let vec_slices = bit_string.chars().rev().collect::<Vec<_>>();

    for (index, c) in vec_slices.chunks(7).enumerate() {
        let value = get_value(c) as u8;

        match index {
            0 => output.push(value),
            _ => {
                if index == vec_slices.chunks(7).len() - 1 && value == 0 {
                    println!("Avoided - last zero !");
                } else {
                    output.push(value|128);
                }
            }
        }

        println!("to_bytes: (index = {:?}) Chunk -> {:x?} chunk_value -> ({:x?})", index, c, output);
        println!("to_bytes: index {:?} != vec.slices.len() = {:?}", index, vec_slices.chunks(7).len());
    }

    output.reverse();
    output
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut bit_string = "".to_string();
    let mut output: Vec<u32> = Vec::new();
    let mut res: Result<Vec<u32>, Error>;

    for (index, item) in bytes.iter().enumerate() {
        let bit_string_value = format!("{:08b}", item );
        println!("from_bytes item -> bit_string_value = {:?}", bit_string_value);
        bit_string.push_str(&bit_string_value.chars().collect::<String>());
    }

    println!(" from_bytes Full bit string -> {:?} len = ({:?})", bit_string, bit_string.len());

    if bit_string.len() < 10000 {
        let vec_slices = bit_string.chars().collect::<Vec<_>>();
        println!(" from_bytes Full bit string -> {:?} len = ({:?})", vec_slices, vec_slices.len());
        // let mut array_value: Vec<char> = Vec::new();
        let mut array_value_bytes: Vec<char> = Vec::new();

        for (index, c) in vec_slices.chunks(8).rev().enumerate() {
            println!("chunks -> {:?}", c);

            if c[0] == '0' {
                println!("End of a value -> {:?}", get_value_rev(&array_value_bytes));
            }
        }

        // for (index, c) in vec_slices.iter().enumerate() {
        //     println!("from_bytes chunk {:?} value = {:?}", c, get_value_rev(&array_value));
        //     array_value.push(*c);
        //     println!("from_bytes chunk {:?} new value = {:?}", c, get_value_rev(&array_value));
        //     if get_value_rev(&array_value) > u32::max_value().into() {
        //         let temp: u32 = 0xffffffff;
        //
        //         println!("u32 max value crossed {:?} u32::max_value()= {:?} outcome = {:?}", index, u32::max_value(), temp);
        //
        //         array_value.pop();
        //         output.push(get_value_rev(&array_value) as u32);
        //         array_value.clear();
        //     } else {
        //         // println!("u32 max value not crossed {:?} u32::max_value()= {:?} outcome = {:?}", index, u32::max_value(), temp);
        //     }
        // }

        // output.push(get_value_rev(&array_value) as u32);
        // output.reverse();

        res = Ok(output);
    } else {
        res = Err(Error::Overflow);
    }

    res
}
