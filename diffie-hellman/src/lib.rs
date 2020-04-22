extern crate rand;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen_range(2u64, p);

    random_number
}

pub fn fast_exponentiation (p: u64, g: u64, a: u64) -> u64 {
    match a {
        0 => 1,
        value if value % 2 == 0 => {
            let t = fast_exponentiation(p, g, a/2);
            t.pow(2) % p
        },
        _rest => {
            let t = fast_exponentiation(p, g, (a-1)/2);
            (g * (t.pow(2) % p)) % p
        }
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    fast_exponentiation(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    fast_exponentiation(p, b_pub, a)
}
