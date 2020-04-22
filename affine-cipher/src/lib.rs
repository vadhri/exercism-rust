use modinverse::modinverse;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const MODULO: i32 = 26;
const CHAR_START: char = 'a';

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut output:String = "".to_string();
    let mut char_counter: i32 = 0;
    let mut res: Result<String, AffineCipherError> = Ok("".to_string());

    let a_inverse = match  modinverse(a, MODULO) {
        None => {
            res = Err(AffineCipherError::NotCoprime(a));
            -1
        },
        Some(x) => x
    };

    if a_inverse >= 0 {
        for (_index, chr) in plaintext.to_lowercase().chars().enumerate() {
            if chr >= 'a' && chr <= 'z' {
                let char_diff = chr as i32 - CHAR_START as i32;
                let encode_char_raw = (a * char_diff) + b;
                let encoded_char: u8 = ((encode_char_raw % MODULO) + CHAR_START as i32) as u8;

                if char_counter !=0 && char_counter % 5 == 0 {
                    output.push(' ');
                }
 
                output.push(encoded_char as char);
                char_counter += 1;
            } else if chr >= '0' && chr <= '9' {
                output.push(chr);
                char_counter += 1;
            }
        }
        res = Ok(output);
    }

    res
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.



pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut output:String = "".to_string();
    let mut res: Result<String, AffineCipherError> = Ok("".to_string());

    let a_inverse = match  modinverse(a, MODULO) {
        None => {
            res = Err(AffineCipherError::NotCoprime(a));
            -1
        },
        Some(x) => x
    };

    if a_inverse >= 0 {
        for chr in ciphertext.chars() {
            if chr >= 'a' && chr <= 'z'
            {
                let chr_diff = chr as i32 - CHAR_START as i32;
                let chr_diff_a_b = a_inverse * (chr_diff - b);

                let decoded_char: u8 = match chr_diff_a_b {
                    x if x >= 0 => (x % MODULO + CHAR_START as i32) as u8,
                    rest => {
                        if rest % MODULO == 0 {
                            (CHAR_START as i32) as u8
                        } else {
                            (MODULO + (rest % MODULO) + CHAR_START as i32) as u8
                        }
                    }
                };

                output.push(decoded_char as char);
            } else if chr >= '0' && chr <= '9' {
                output.push(chr);
            }
        }
        res = Ok(output);
    }
    res
}
