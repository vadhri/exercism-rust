extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    factors_tuple: Vec<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let mut ft = Vec::new();
        ft.push((a, b));
        Palindrome {
            factors_tuple: ft,
            value: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors_tuple.push((a, b));
    }
}

fn is_palindrome(mut number: u64) -> bool {
    // multiple of 10 cannot be a palindrome
    if number % 10 == 0 {
        return false;
    }
    // assemble the reverse of the right half of number
    let mut rebmun: u64 = 0;
    while rebmun < number {
        // push the rightmost digit onto the reverse
        rebmun = 10 * rebmun + number % 10;
        // pop the rightmost digit from the number
        number /= 10;
    }
    // match the left half with the reverse of the right
    // accounting for the possibility of an odd-length number
    number == rebmun || number == rebmun / 10
}


pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut hashmap = HashMap::new();
    let mut result: Option<(Palindrome, Palindrome)> = None;

    for item in (min..=max).combinations_with_replacement(2) {
        let value = item[0] * item[1];
        let is_palindrome: bool = is_palindrome(value);

        if is_palindrome {
            match hashmap.contains_key(&value) {
                false => {
                    hashmap.insert(value, Palindrome::new(item[0], item[1]));
                },
                true => {
                    hashmap.get_mut(&value).unwrap().insert(item[0], item[1]);
                }
            };
        }
    }

    if hashmap.len() > 0 {
        let min = hashmap.keys().min().unwrap();
        let p = hashmap.get(&min).unwrap();

        let max = hashmap.keys().max().unwrap();
        let q = hashmap.get(&max).unwrap();

        // println!("{:?} {:?}", min, max);

        result = Some((p.clone(), q.clone()));
    }

    result
}
