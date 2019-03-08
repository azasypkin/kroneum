/// Represents Time in hours, minutes and seconds. Max value is 24 hours.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BCDTime {
    pub hours: u8,
    pub hours_tens: u8,
    pub minutes: u8,
    pub minutes_tens: u8,
    pub seconds: u8,
    pub seconds_tens: u8,
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
            self.add_hours(new_value / 60);
        } else {
            self.minutes = new_value as u8;
        }
    }

    /// Adds specified number of hours to the current time. Rolls over after 24h.
    pub fn add_hours(&mut self, hours: u32) {
        let hours = self.hours as u32 + hours;
        self.hours = if hours >= 24 { hours % 24 } else { hours } as u8
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
    pub fn from_hours(hours: u32) -> Self {
        Time::from_minutes(hours * 60)
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

impl Default for BCDTime {
    fn default() -> Self {
        BCDTime {
            hours_tens: 0,
            hours: 0,
            minutes_tens: 0,
            minutes: 0,
            seconds_tens: 0,
            seconds: 0,
        }
    }
}

impl From<&BCDTime> for Time {
    fn from(bcd_time: &BCDTime) -> Self {
        Time {
            hours: bcd_time.hours_tens * 10 + bcd_time.hours,
            minutes: bcd_time.minutes_tens * 10 + bcd_time.minutes,
            seconds: bcd_time.seconds_tens * 10 + bcd_time.seconds,
        }
    }
}

impl From<&Time> for BCDTime {
    fn from(time: &Time) -> Self {
        BCDTime {
            hours_tens: time.hours / 10,
            hours: time.hours % 10,
            minutes_tens: time.minutes / 10,
            minutes: time.minutes % 10,
            seconds_tens: time.seconds / 10,
            seconds: time.seconds % 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_seconds() {
        let mut time = Time::default();

        time.add_seconds(10);
        assert_eq!(time, Time::from_seconds(10));

        time.add_seconds(75);
        assert_eq!(time, Time::from_seconds(85));

        time.add_seconds(6700);
        assert_eq!(time, Time::from_seconds(6785));
    }

    #[test]
    fn add_minutes() {
        let mut time = Time::default();

        time.add_minutes(10);
        assert_eq!(time, Time::from_minutes(10));

        time.add_minutes(75);
        assert_eq!(time, Time::from_minutes(85));

        time.add_minutes(125);
        assert_eq!(time, Time::from_minutes(210));
    }

    #[test]
    fn add_hours() {
        let mut time = Time::default();

        time.add_hours(2);
        assert_eq!(time, Time::from_hours(2));

        time.add_hours(26);
        assert_eq!(time, Time::from_hours(4));
    }

    #[test]
    fn from_seconds() {
        assert_eq!(
            Time::from_seconds(10),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 10,
            }
        );

        assert_eq!(
            Time::from_seconds(125),
            Time {
                hours: 0,
                minutes: 2,
                seconds: 5,
            }
        );

        assert_eq!(
            Time::from_seconds(7403),
            Time {
                hours: 2,
                minutes: 3,
                seconds: 23,
            }
        );

        assert_eq!(
            Time::from_seconds(94573),
            Time {
                hours: 2,
                minutes: 16,
                seconds: 13,
            }
        );
    }

    #[test]
    fn from_minutes() {
        assert_eq!(
            Time::from_minutes(10),
            Time {
                hours: 0,
                minutes: 10,
                seconds: 0,
            }
        );

        assert_eq!(
            Time::from_minutes(125),
            Time {
                hours: 2,
                minutes: 5,
                seconds: 0,
            }
        );

        assert_eq!(
            Time::from_minutes(7403),
            Time {
                hours: 3,
                minutes: 23,
                seconds: 0,
            }
        );
    }

    #[test]
    fn from_hours() {
        assert_eq!(
            Time::from_hours(10),
            Time {
                hours: 10,
                minutes: 0,
                seconds: 0,
            }
        );

        assert_eq!(
            Time::from_hours(125),
            Time {
                hours: 5,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn from_bcd() {
        assert_eq!(
            Time::from(&BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 4,
                seconds_tens: 5,
                seconds: 1,
            }),
            Time {
                hours: 13,
                minutes: 34,
                seconds: 51,
            },
        );
    }

    #[test]
    fn to_bcd() {
        assert_eq!(
            BCDTime::from(&Time {
                hours: 13,
                minutes: 34,
                seconds: 51,
            }),
            BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 4,
                seconds_tens: 5,
                seconds: 1,
            },
        );
    }
}
