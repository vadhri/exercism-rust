pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    for i in 1..list.len() {
        println!("({}, {}) {}", list[i], list[i-1], list.len());
        result.push_str(&format!("For want of a {} the {} was lost.\n", list[i-1], list[i]));
    }
    if list.len() > 0 {
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    }

    result
}
