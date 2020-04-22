pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let number_of_digits = digits.len() as u32;

    digits.iter().map(|x| x.pow(number_of_digits)).fold(0, |sum, x| sum + x) == num
}
