/// Type implementing arbitrary-precision decimal arithmetic
extern crate num_bigint;
extern crate num_traits;
extern crate num;

use core::cmp::Ordering;
use crate::num_bigint::ToBigInt;
use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Decimal {
    i: BigInt,
    dot_index: i64
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut dot_index_input: i64 = 0;
        let mut input_string = input.clone().to_string();

        input_string = match input.find(".") {
               Some(_idx) => {
                   input_string
                       .trim_start_matches("0")
                       .trim_end_matches("0")
                       .to_string()
               }, None => {
                   input_string
               }
           };

        match input_string.chars().last() {
            Some('.') => {
                input_string.push('0');
            },
            Some(_) => {

            },
            None => {

            }
        };

        match input_string.chars().next() {
            Some('.') => {
                input_string.insert(0, '0');
            },
            Some(_) => {

            },
            None => {

            }
        };

        dot_index_input = match input_string.find(".") {
            Some(idx) => {
                (input_string.len() - 1 - idx) as i64
            }, None => {
                input_string.push_str(".0");
                1
            }
        };

        if dot_index_input > 0 {
            input_string.remove(input_string.len() - 1 - dot_index_input as usize);
        }

        if input_string.is_empty() {
            input_string.push('0');
        }

        let i_d_bigint = input_string.parse::<BigInt>().unwrap();

        Some(Decimal {
            i: i_d_bigint,
            dot_index: dot_index_input
        })
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        let ten_to_power = 10_i32.to_bigint().unwrap();
        let self_i = self.i.clone();
        let other_i = other.i.clone();
        let mut self_i_modified: BigInt = self_i.clone();
        let mut output_i_modified: BigInt = other_i.clone();

        match self.dot_index < other.dot_index {
            true => {
                self_i_modified = self_i * num::pow(ten_to_power, (other.dot_index - self.dot_index) as usize);
            },
            false => {
                if self.dot_index == other.dot_index {
                } else {
                    output_i_modified = other_i * num::pow(ten_to_power, (self.dot_index - other.dot_index) as usize);
                }
            }
        };

        if self_i_modified < output_i_modified {
            Some(Ordering::Less)
        } else if self_i_modified == output_i_modified {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let ten_to_power = 10_i32.to_bigint().unwrap();
        let self_i = self.i.clone();
        let other_i = other.i.clone();

        match self.dot_index < other.dot_index {
            true => {
                let self_i_modified = self_i * num::pow(ten_to_power, (other.dot_index - self.dot_index) as usize);
                self_i_modified == other.i
            },
            false => {
                if self.dot_index == other.dot_index {
                    other.i == self.i
                } else {
                    let output_i_modified = other_i * num::pow(ten_to_power, (self.dot_index - other.dot_index) as usize);
                    output_i_modified == self.i
                }
            }
        }
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut dot_index: i64 = 0;
        let mut output_i = self.i.clone();
        let ten_to_power = 10_i32.to_bigint().unwrap();

        let _value = match self.dot_index < other.dot_index {
            true => {
                dot_index = other.dot_index;
                output_i = other.i + (self.i * num::pow(ten_to_power, (other.dot_index - self.dot_index) as usize));
            },
            false => {
                if self.dot_index == other.dot_index {
                    dot_index = self.dot_index;
                    output_i = self.i + other.i;
                } else {
                    dot_index = self.dot_index;
                    output_i = self.i + (other.i * num::pow(ten_to_power, (self.dot_index - other.dot_index) as usize));
                }
            }
        };

        Decimal {
            i: output_i,
            dot_index: dot_index
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut dot_index: i64 = 0;
        let mut output_i = self.i.clone();
        let ten_to_power = 10_i32.to_bigint().unwrap();

        let _value = match self.dot_index < other.dot_index {
            true => {
                dot_index = other.dot_index;
                output_i = (self.i * num::pow(ten_to_power, (other.dot_index - self.dot_index) as usize)) - other.i;
            },
            false => {
                if self.dot_index == other.dot_index {
                    dot_index = self.dot_index;
                    output_i = self.i - other.i;
                } else {
                    dot_index = self.dot_index;
                    output_i = self.i - (other.i * num::pow(ten_to_power, (self.dot_index - other.dot_index) as usize));
                }
            }
        };
        
        Decimal {
            i: output_i,
            dot_index: dot_index
        }
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let output_calc = self.i.clone() * other.i.clone();

        let output_calc_string = output_calc.clone().to_string();
        let output_calc_string_trimmed = output_calc_string.trim_end_matches('0');
        let chopped_zeros_length = (output_calc_string.len() - output_calc_string_trimmed.len()) as i64;

        let dot_index_output = match other.dot_index {
            0 => {
                0
            },
            rest => rest
        };

        Decimal {
            i: output_calc_string_trimmed.parse::<BigInt>().unwrap(),
            dot_index: self.dot_index + dot_index_output - chopped_zeros_length
        }
    }
}
