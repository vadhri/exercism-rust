pub fn encode(n: u64) -> String {
    let mut temp = n;
    let mut output: String = "".to_string();

    let less_than_twenty = |num| {
        let value = match num {
            0 => "zero",
             1 => "one",
             2 => "two",
             3 => "three",
             4 => "four",
             5 => "five",
             6 => "six",
             7 => "seven",
             8 => "eight",
             9 => "nine",
             10 => "ten",
             11 => "eleven",
             12 => "twelve",
             13 => "thirteen",
             14 => "fourteen",
             15 =>"fifteen",
             16 =>"sixteen",
             17 =>"seventeen",
             18 =>"eighteen",
             19 =>"nineteen",
             _rest => "problem!"
        };

        value
    };

    if n < 20 {
        output.push_str(less_than_twenty(n));
    } else {
        while temp != 0 {
            match temp {
                val if val >= 1000000000000000000 => {
                    let m = val / 1000000000000000000;

                    if m < 20 {
                        output.push_str(less_than_twenty(m));
                    } else {
                        output.push_str(&encode(m));
                    }

                    output.push_str(" quintillion ");

                    temp = val % 1000000000000000000;
                },
                val if val >= 1000000000000000 => {
                    let m = val / 1000000000000000;

                    if m < 20 {
                        output.push_str(less_than_twenty(m));
                    } else {
                        output.push_str(&encode(m));
                    }

                    output.push_str(" quadrillion ");

                    temp = val % 1000000000000000;
                },
                val if val >= 1000000000000 => {
                    let m = val / 1000000000000;

                    if m < 20 {
                        output.push_str(less_than_twenty(m));
                    } else {
                        output.push_str(&encode(m));
                    }

                    output.push_str(" trillion ");

                    temp = val % 1000000000000;
                },
                val if val >= 1000000000 => {
                    let m = val / 1000000000;

                    if m < 20 {
                        output.push_str(less_than_twenty(m));
                    } else {
                        output.push_str(&encode(m));
                    }

                    output.push_str(" billion ");

                    temp = val % 1000000000;
                },
                val if val >= 1000000 => {
                    let m = val / 1000000;

                    if m < 20 {
                        output.push_str(less_than_twenty(m));
                    } else {
                        output.push_str(&encode(m));
                    }

                    output.push_str(" million ");

                    temp = val % 1000000;
                },
                val if val >= 1000 => {
                    output.push_str(&encode(val / 1000));
                    output.push_str(" thousand ");
                    temp = val % 1000;
                },
                val if val >= 100 => {
                    output.push_str(less_than_twenty(val / 100));
                    output.push_str(" hundred ");
                    temp = val % 100;
                },
                val if val >= 20 => {
                    match val/10 {
                        2 => output.push_str("twenty-"),
                        3 => output.push_str("thirty-"),
                        4 => output.push_str("forty-"),
                        5 => output.push_str("fifty-"),
                        rest => {
                            output.push_str(less_than_twenty(rest).trim_end_matches('t'));
                            output.push_str("ty-");
                        }
                    }
                    temp = val % 10;
                },
                val if val < 20 => {
                    output.push_str(less_than_twenty(val));
                    temp = 0;
                }
                _ => {
                    temp = 0;
                }
            };
        }
    }

    output.trim_end().trim_end_matches('-').to_string()
}
