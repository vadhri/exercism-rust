pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut iter = 1;
    let mut vector_primes: Vec<u64> = (2..=upper_bound).collect();

    loop {
        if iter > upper_bound {
            break;
        } else {
            iter += 1;
        }
        vector_primes.retain(|x| {
            x % iter != 0 || *x == iter
        });
    }
    vector_primes
}
