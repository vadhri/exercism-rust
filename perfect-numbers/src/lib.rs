#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let result: Option<Classification> = None;
    if num == 0 {
        result
    } else {
        match (1..num).filter(|x| num % x == 0).fold(0, |sum, x| sum + x) {
            x if x == num => Some(Classification::Perfect),
            x if x > num => Some(Classification::Abundant),
            x if x < num => Some(Classification::Deficient),
            _ => None
        }
    }
}
