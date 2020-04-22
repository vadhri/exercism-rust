use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

pub struct Robot {
    name: String
}

impl Robot {
    pub fn generate_name() -> String {
        let rng = thread_rng();
        let mut s_char: String = rng.sample_iter(Alphanumeric)
            .filter(|ch| ch.is_uppercase())
            .take(2)
            .collect();
        let s_number: String = rng.sample_iter(Alphanumeric)
            .filter(|ch| ch.is_numeric())
            .take(3)
            .collect();

        s_char.push_str(&s_number);
        s_char
    }

    pub fn new() -> Self {
        Robot {
            name: Robot::generate_name()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }
}
