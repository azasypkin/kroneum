/// Represents Time in hours, minutes and seconds. Max value is 24 hours.
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    /// Adds specified number of seconds to the current time, updates minutes and hours if needed.
    pub fn add_seconds(&mut self, seconds: u32) {
        let new_value = seconds + self.seconds as u32;

        if new_value >= 60 {
            self.seconds = (new_value % 60) as u8;
            self.add_minutes(new_value / 60);
        } else {
            self.seconds = new_value as u8;
        }
    }

    /// Adds specified number of minutes to the current time, updates hours if needed.
    pub fn add_minutes(&mut self, minutes: u32) {
        let new_value = minutes + self.minutes as u32;

        if new_value >= 60 {
            self.minutes = (new_value % 60) as u8;
            self.add_hours((new_value / 60) as u8);
        } else {
            self.minutes = new_value as u8;
        }
    }

    /// Adds specified number of hours to the current time.
    pub fn add_hours(&mut self, hours: u8) {
        self.hours += hours;

        if self.hours >= 24 {
            self.hours -= 24;
        }
    }

    /// Creates time from seconds.
    pub fn from_seconds(seconds: u32) -> Self {
        let mut time = Time::default();
        time.add_seconds(seconds);
        time
    }

    /// Creates time from minutes.
    pub fn from_minutes(minutes: u32) -> Self {
        Time::from_seconds(minutes * 60)
    }

    /// Creates time from hours.
    pub fn from_hours(hours: u8) -> Self {
        Time::from_minutes((hours * 60) as u32)
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            seconds: 0,
            minutes: 0,
            hours: 0,
        }
    }
}
