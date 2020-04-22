pub fn number(user_number: &str) -> Option<String> {
    let mut number: String = user_number.chars()
        .filter(|ch| ch.is_digit(11))
        .collect::<Vec<_>>()
        .rchunks(11)
        .next()
        .unwrap()
        .to_vec()
        .into_iter()
        .collect::<String>();

    let mut international_code_vec = Vec::new();
    let mut international_code = ' ';

    if number.len() > 10 {
        international_code_vec.append(&mut number.drain(0..1).collect::<Vec<_>>());
        international_code = *international_code_vec.get(0).unwrap();
    }

    let area_code_digit = number.chars().nth(0).unwrap();
    let area_code_digit_check = area_code_digit != '1' && area_code_digit != '0';
    let exchange_code_digit = number.chars().nth(3).unwrap();
    let exchange_code_digit_check = exchange_code_digit != '1' && exchange_code_digit != '0';

    println!("{:?} number {:?} international_code = {:?}", area_code_digit_check, exchange_code_digit_check, international_code);

    if exchange_code_digit_check && area_code_digit_check {
        if international_code_vec.len() > 0 && international_code != '1' {
            None
        } else {
            Some(number)
        }
    } else {
        None
    }
}
