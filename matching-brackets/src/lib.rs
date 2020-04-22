pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vector: Vec<char> = Vec::new();
    let mut last_char: char; 

    for c in string.chars() {
        last_char = match vector.last() {
            Some(x) => *x,
            _ => ' '
        };

        if c == '[' || c == '{' ||  c == '(' {
            vector.push(c);
        } else if c ==  ']' && last_char == '[' {
            vector.pop();
        } else if c ==  '}' && last_char == '{' {
            vector.pop();
        } else if c == ')' && last_char == '(' {
            vector.pop();
        } else if c == ']' || c == '}' ||  c == ')' {
            vector.push(c);
        }
    }

    match vector.len() {
        0 => true,
        _ => false
    }
}
