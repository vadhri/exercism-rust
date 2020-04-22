pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    let mut out = n;

    while out > 0 {
        if out == 1 {
            break;
        }

        count += 1;

        match out % 2 == 0 {
            true => {
                out = out / 2;
            },
            false => {
                out = (3 * out) + 1
            }
        }
    }

    if out == 0 {
        None
    } else {
        Some(count)
    }
}
