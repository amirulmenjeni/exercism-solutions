use std::{fmt::{self, Debug, Display}};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        
        let total_mins  = hours * 60 + minutes;
        
        Clock {
            hours: total_mins.div_euclid(60).rem_euclid(24),
            minutes: total_mins.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}