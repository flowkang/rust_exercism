use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let all_minutes = (hours * 60 + minutes) % 1440;
        let all_minutes =  if all_minutes < 0 { all_minutes + 1440 } else { all_minutes };
        Clock {
            hours: (all_minutes / 60) % 24,
            minutes: all_minutes % 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}


impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}