use std::fmt::{Debug, Display, Error, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_in_day = 24 * 60;
        let total_minutes = (hours * 60 + minutes).rem_euclid(minutes_in_day);

        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}
