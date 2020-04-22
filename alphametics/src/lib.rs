extern crate itertools;

use itertools::Itertools;

use std::collections::HashMap;

pub fn convert_string_to_numbers(i: HashMap<char, u8>, input: &str) -> i32 {
    let mut output = input;
    let mut temp;

    for item in i.keys() {
        temp = output.replace(*item, &i.get(&item).unwrap().to_string());
        output = &temp;
    }
    output.parse::<i32>().unwrap()
}

pub fn convert_string_to_numbers_with_zero_check (i: HashMap<char, u8>, input: &str) -> i64 {
    let first_char = input.chars().next().unwrap();

    if *i.get(&first_char).unwrap() == 0 {
        -1 as i64
    } else {
        let mut output1 = 0 as i64;

        for (index, item) in input.chars().rev().enumerate() {
            output1 += (*i.get(&item).unwrap() as i64) * 10_i64.pow(index as u32)
        }
        output1
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let split_equation:Vec<&str> = input.split("==").collect();
    let inputs:Vec<&str>  = split_equation[0].split("+").map(|x| x.trim_end().trim_start()).collect();
    let output:&str  = split_equation[1].trim_end().trim_start();

    let mut unique_chars:Vec<char> = input.chars().filter(|ch| ch.is_alphabetic()).collect();

    unique_chars.sort();
    unique_chars.dedup();

    let combination_length = unique_chars.len();
    let it = (0..=9).permutations(combination_length);

    let mut output_map: Option<HashMap<char, u8>> = None;
    let mut value_map: HashMap<char, u8> = HashMap::new();
    let mut input_value_converted_vector = Vec::new();
    let mut word_value_map: HashMap<&str, i64> = HashMap::new();

    for item in it {
        for (index, value) in item.iter().enumerate() {
            value_map.insert(unique_chars[index], *value as u8);
        }

        for input_value in &inputs {
            match word_value_map.contains_key(input_value) {
                false => {
                    let value_vector = convert_string_to_numbers_with_zero_check(value_map.clone(), input_value);
                    input_value_converted_vector.push(value_vector);
                    word_value_map.insert(input_value, value_vector);
                },
                true => {
                    input_value_converted_vector.push(*word_value_map.get(input_value).unwrap());
                }
            }
        }

        let output_value_vector = convert_string_to_numbers_with_zero_check(value_map.clone(), output);

        if input_value_converted_vector.iter().fold(0, |sum, x| sum + x) == output_value_vector {
            output_map = Some(value_map);
            break;
        }
        value_map.clear();
        input_value_converted_vector.clear();
        word_value_map.clear();
    }

    output_map
}
