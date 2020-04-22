use std::fmt::{Display, Formatter, Result};

pub struct Roman (String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let vector = vec![
          ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
          ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
          ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
          ["", "M", "MM", "MMM", "", "", "", "", "", ""]
        ];

        let number = &self.0;
        let mut output = Vec::new();

        for (index, ch) in number.chars().rev().enumerate() {
            let digit = ch.to_digit(10).unwrap() as usize;
            output.push(vector[index][digit]);
        }

        output.reverse();
        write!(f, "{}", output.join(""))
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num.to_string())
    }
}
