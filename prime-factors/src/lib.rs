pub fn factors(n: u64) -> Vec<u64> {
    let mut n_clone = n;
    let mut vec = Vec::new();
    let mut n_clone_orig = n_clone;

    for num_2 in 2..=2 {
        while n_clone % num_2 == 0 && n_clone != 1 {
            n_clone = n_clone / num_2;
            vec.push(num_2);
        }
        if n_clone_orig > n_clone {
            n_clone_orig = n_clone;
        }
    }

    for num in (3..=n).step_by(2) {
        while n_clone % num == 0 && n_clone != 1 {
            n_clone = n_clone / num;
            vec.push(num);
        }
        if n_clone_orig > n_clone {
            n_clone_orig = n_clone;
        }

        if n_clone == 1  {
            break;
        }
    }
    vec
}
