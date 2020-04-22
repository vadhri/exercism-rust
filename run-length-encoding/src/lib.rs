pub fn encode(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();

    let mut last_char: char = '-';
    let mut last_char_count: u32 = 1;
    let mut output:String = "".to_string();

    for ch in chars {
        if last_char == '-' {
            last_char = ch;
        } else if last_char == ch {
            last_char_count += 1;
        } else {
            if last_char_count != 1 {
                output.push_str(&last_char_count.to_string());
            }
            output.push(last_char);
            last_char = ch; 
            last_char_count = 1;
        }

    }

    if last_char_count != 1 {
        output.push_str(&last_char_count.to_string());
    }
    if last_char != '-' {
        output.push(last_char);
    }

    output
}

pub fn decode(source: &str) -> String {
    let mut digit: String = "0".to_string();
    let mut output_string = "".to_string();

    for chr in source.chars() {
        if chr.is_numeric() {
            digit.push(chr);
        }
        else if !chr.is_numeric() {
            let number:u32 = digit.parse().unwrap();
            if number == 0 {
                output_string.push(chr);
            } else {
                output_string.push_str(&(0..number).map(|_| chr).collect::<String>());
                digit= "0".to_string();
            }
        }
    }

    output_string
}
