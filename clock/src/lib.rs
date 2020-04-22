use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hours * 60) + self.minutes == (other.hours * 60) + other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut calcuated_hours = match hours {
            x if x >= 0  => x,
            x if x < 0 => 24 + (hours % 24),
            _ => 0
        };

        let calculated_minutes = match minutes {
            x if x >= 0  => x,
            x if x < 0 => {
                calcuated_hours += (minutes / 60) % 24;

                if minutes % 60 < 0 {
                    calcuated_hours -= 1;
                }

                if calcuated_hours < 0 {
                    calcuated_hours = 24 + calcuated_hours;
                } else if calcuated_hours >= 24 {
                    calcuated_hours %= 24;
                }

                (60 + (x % 60)) % 60
            },
            _ => 0
        };

        Clock {
            hours: (calcuated_hours + (calculated_minutes / 60)) % 24,
            minutes:  calculated_minutes % 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut calcuated_hours = self.hours;
        let total_minutes = minutes + self.minutes;

        let calculated_minutes = match total_minutes {
            x if x >= 0  => x,
            x if x < 0 => {
                calcuated_hours += (total_minutes / 60) % 24;

                if total_minutes % 60 < 0 {
                    calcuated_hours -= 1;
                }

                if calcuated_hours < 0 {
                    calcuated_hours = 24 + calcuated_hours;
                } else if calcuated_hours >= 24 {
                    calcuated_hours %= 24;
                }

                (60 + (x % 60)) % 60
            },
            _ => 0
        };

        Clock {
            hours: (calcuated_hours + (calculated_minutes / 60)) % 24,
            minutes: calculated_minutes % 60
        }
    }
}
