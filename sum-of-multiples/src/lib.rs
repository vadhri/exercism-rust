pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut mult = Vec::new();

    for num in 1..limit {
        let multiples = factors.iter().map(|f| {
            if *f == 0 {
                0
            } else {
            match num % f {
                0 => 1,
                _ => 0
            }}
        }).fold(0, |sum, x| sum + x);

        if multiples > 0 {
            mult.push(num);
        }
    }
    mult.iter().fold(0, |sum, x| sum + x)
}
