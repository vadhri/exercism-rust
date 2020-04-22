pub fn is_prime_number(i: u32) -> bool {
    let mut is_prime = true;
    for x in 2..i {
        if i % x == 0 {
            is_prime = false;
            break;
        }
    }
    is_prime
}

pub fn nth(n: u32) -> u32 {
    println!("Nth prime = {:?}", n  + 1);
    let mut prime_number: u32 = 2;
    let mut prime_numbers: Vec<u32> = Vec::new();

    while prime_numbers.len() < (n as usize) + 1 {
        if is_prime_number(prime_number) {
            prime_numbers.push(prime_number);
        }
        prime_number += 1;
    }

    let result = match prime_numbers.pop() {
        Some(x) => x,
        None => 0
    };

    result
}
