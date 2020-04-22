use itertools::Itertools;

pub fn find_square(n: u32) -> (u32, u32) {
    let mut r_c_combinations = (0..=n).combinations_with_replacement(2)
        .filter(|item| {
            (item[0] * item[1]) > n && (item[0] * item[1]) <= n + 15 && item[1] != item[0] ||
            (item[0] * item[1]) == n
        })
        .map(|x| (((x[0] as i32 - x[1] as i32)).abs(), x))
        .collect::<Vec<_>>();

    r_c_combinations.sort_by(|a, b| a.0.cmp(&b.0));
    (r_c_combinations[0].1[0],r_c_combinations[0].1[1])
}

pub fn encrypt(input: &str) -> String {
    let mut output: String = String::new();
    let mut square: Vec<Vec<char>> = Vec::new();

    if input.len() != 0 {
        let modified_string: Vec<char> = input.to_lowercase().chars().filter(|ch| ch.is_alphanumeric()).collect();
        let mut char_counter: usize = 0;
        let (cols, rows) = find_square(modified_string.len() as u32);

        for _row in 0..rows {
            let r = Vec::with_capacity(cols as usize);
            square.push(r);
        }

        for _col in 0..cols {
            for row in 0..rows {
                if char_counter >= modified_string.len() {
                    square[row as usize].push(' ');
                } else {
                    square[row as usize].push(modified_string[char_counter]);
                }
                char_counter += 1;
            }
        }

        for row in 0..rows {
            if row != 0 {
                output.push(' ');
            }
            output.push_str(&square[row as usize].iter().collect::<String>());
        }
    }
    output
}
