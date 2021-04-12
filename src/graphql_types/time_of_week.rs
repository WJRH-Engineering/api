/// ------------
/// TIME OF WEEK
/// ------------

use chrono::offset::Utc;
use chrono::{Datelike, Timelike};
use std::convert::From;

use chrono::Weekday;

use std::fmt::Display;
use std::fmt;

use async_graphql::*;


#[derive(Debug)]
pub struct TimeOfWeek {
    day: Weekday,
    hour: u32,
    minute: u32,
    second: u32,
}

#[Object]
impl TimeOfWeek {
    pub async fn value(&self) -> String { self.to_string() }
    pub async fn day(&self) -> u32 { self.day.number_from_monday() }
    pub async fn hour(&self) -> u32 { self.hour }
    pub async fn minute(&self) -> u32 { self.minute }
    pub async fn second(&self) -> u32 { self.second }
}

impl TimeOfWeek {
    pub fn new(day: Weekday, hour: u32, minute: u32, second: u32) -> Self {
        Self { day, hour, minute, second }
    }
}

impl From<chrono::DateTime<Utc>> for TimeOfWeek {
    fn from(datetime: chrono::DateTime<Utc>) -> Self {
        let day = datetime.weekday();
        let hour = datetime.hour();
        let minute = datetime.minute();
        let second = datetime.second();
        Self { day, hour, minute, second }
    }
}

impl Display for TimeOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let weekday = self.day.number_from_monday();
        let hour = self.hour;
        let minute = self.minute;
        let second = self.second;
        write!(f, "{}w{}h{}m{}s", weekday, hour, minute, second )
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use chrono::Utc;
    use chrono::TimeZone;

    #[test]
    fn time_conversion_and_display() {
        let test_date = Utc.ymd(1996, 1, 2).and_hms_milli(10, 10, 10, 0);        
        let time_of_week: TimeOfWeek = test_date.into();
        let display_string = format!("{}", time_of_week);
        assert_eq!(display_string, "2w10h10m10s");
    }
}
