pub fn is_leap_year(year: u64) -> bool {
    //unimplemented!("true if {} is a leap year", year)
    match year {
        _ if year % 4 == 0 => {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        },
        _ => false
    }
}
