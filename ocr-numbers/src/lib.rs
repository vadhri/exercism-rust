// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut hm_ocr_2d_array = HashMap::new();
    let mut ocr_recognized_string = String::new();

    let mut output:Result<String, Error> = Ok(String::new());

    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec!['|', ' ', '|'], vec!['|', '_', '|'], vec![' ', ' ', ' ']], 0);
    hm_ocr_2d_array.insert(vec![vec![' ', ' ', ' '], vec![' ', ' ', '|'], vec![' ', ' ', '|'], vec![' ', ' ', ' ']], 1);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec![' ', '_', '|'], vec!['|', '_', ' '], vec![' ', ' ', ' ']], 2);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec![' ', '_', '|'], vec![' ', '_', '|'], vec![' ', ' ', ' ']], 3);
    hm_ocr_2d_array.insert(vec![vec![' ', ' ', ' '], vec!['|', '_', '|'], vec![' ', ' ', '|'], vec![' ', ' ', ' ']], 4);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec!['|', '_', ' '], vec![' ', '_', '|'], vec![' ', ' ', ' ']], 5);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec!['|', '_', ' '], vec!['|', '_', '|'], vec![' ', ' ', ' ']], 6);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec![' ', ' ', '|'], vec![' ', ' ', '|'], vec![' ', ' ', ' ']], 7);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec!['|', '_', '|'], vec!['|', '_', '|'], vec![' ', ' ', ' ']], 8);
    hm_ocr_2d_array.insert(vec![vec![' ', '_', ' '], vec!['|', '_', '|'], vec![' ', '_', '|'], vec![' ', ' ', ' ']], 9);

    let mut col_count = Vec::new();

    let input_2d_matrix = input
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|row| {
            let row_chars = row.chars().collect::<Vec<_>>();
            col_count.push(row_chars.len());
            row_chars
        })
        .collect::<Vec<_>>();

    let mut column_count = col_count.clone();
    column_count.dedup();
    let col_count_check =  column_count.len() == 1 && column_count[0] % 3 == 0;
    let no_rows = input_2d_matrix.len();

    if no_rows % 4 == 0 && col_count_check {
        for digit_col_iterator in (0..no_rows).step_by(4) {
            if digit_col_iterator % 4 == 0 && digit_col_iterator != 0 {
                ocr_recognized_string.push(',');
            }
            for digit_iterator in (0..column_count[0]).step_by(3) {
                let mut matrix: Vec<Vec<char>> = Vec::new();

                for row in 0..4 {
                    let start = digit_iterator;
                    let end = digit_iterator + 3;
                    let row_vector = Vec::from_iter(input_2d_matrix[digit_col_iterator + row].iter().cloned());

                    matrix.push(Vec::from_iter(row_vector[start..end].iter().cloned()));
                }

                let recognized_value = match hm_ocr_2d_array.contains_key(&matrix) {
                    true => {
                        hm_ocr_2d_array.get(&matrix).unwrap()
                    },
                    false => &-1
                };

                if *recognized_value >= 0 {
                    ocr_recognized_string.push_str(&recognized_value.to_string());
                } else {
                    ocr_recognized_string.push('?');
                }
            }
        }
    } else {
        if col_count_check == false {
            output = Err(Error::InvalidColumnCount(column_count[0]));
        } else {
            output = Err(Error::InvalidRowCount(no_rows));
        }
    }

    if ocr_recognized_string.len() > 0 {
        output = Ok(ocr_recognized_string);
    }

    output
}
