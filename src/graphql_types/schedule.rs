use crate::graphql_types::time_of_week::*;
use crate::data_sources::postgres;
use core::ops::Bound::*;
use crate::graphql_types::teal;
use async_graphql::*;
use std::fmt;
use crate::data_sources::redis;

#[derive(Debug)]
struct Schedule {
    year: i32,
    season: Season,
}

/// ## Schedule
/// 
/// The 'Schedule' type represents WJRH's weekly programming schedule for a
/// given year and season. 
#[Object]
impl Schedule {

    /// Schedules are uniquely identified by a combination of a year and a 
    /// season. Season's are typically either FALL or SPRING, although
    /// potentially any value could be used.
    ///
    /// Schedule ids will be formatted "{year} {season}"
    /// eg. 2020 FALL, 2021 SPRING, 2022 SUMMER, etc.
    ///
    /// The year and season can also be accessed individually through the 
    /// `year` and `season` fields.
    pub async fn id(&self) -> String {
        self.to_string()
    } 

    /// Schedules are defined by a set of timeslots. Timeslots contain a 
    /// `Program` as well as a weekly start and end time. These timeslots
    /// represent both remote shows and shows done in the studio.
    pub async fn timeslots(&self) -> Vec<TimeSlot> {
       let season = self.season.to_string();
       let timeslots = postgres::query_schedule(self.year, &season);
       let schedule: TimeSlots = timeslots.await.into();
       schedule.timeslots()
    }

    pub async fn year(&self) -> i32 { self.year }
    pub async fn season(&self) -> String { self.season.to_string() }
}

impl Schedule { 
    fn new(year: i32, season_str: &str) -> Self {
        Self { year, season: season_str.into() }
    }
}

impl fmt::Display for Schedule { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.year, self.season)
    }
}

// --------
// TIMESLOT
// --------

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
struct TimeSlot {
	pub shortname: String,
    pub start: TimeOfWeek,
    pub end: TimeOfWeek,
    pub schedule: Schedule,
}

#[ComplexObject]
impl TimeSlot {
    pub async fn program(&self) -> teal::Program {
        redis::Program::get(&self.shortname).into()
    }
}

// -----
// QUERY
// -----

#[derive(Default)]
pub struct ScheduleQuery;

#[Object]
impl ScheduleQuery {
    pub async fn schedule(&self) -> Schedule {
        Schedule::new(2021, "SPRING")
    }
}

use core::ops::Bound;


// -----------
// SEASON ENUM
// -----------

/// An ENUM representing the different seasons that a schedule can take place
/// in. In theory, this could be any String, but in practice the vast majority
/// will correspond to one of Lafayette's semesters, which is typically how
/// long the station will run shows for.
///
/// The Season type implements Display and From<&str> so that it can be easily
/// converted to and from strings.
#[derive(Debug)]
enum Season {
    Spring,
    Fall,
    Summer,
    Other(String),
}

impl From<&str> for Season {
   fn from(string: &str) -> Self {
       match string.to_uppercase().as_str() { 
            "SPRING" => Self::Spring,
            "FALL" => Self::Fall,
            "SUMMER" => Self::Summer,
            _ => Self::Other(string.to_string()),
       }
   } 
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Spring => "SPRING",
            Self::Fall => "FALL",
            Self::Summer => "SUMMER",
            Self::Other(val) => val,
        };
        write!(f, "{}", out)
    }
}


// ----------------
// TYPE CONVERSIONS
// ----------------

impl From<postgres::TimeSlot> for TimeSlot {
    fn from(timeslot: postgres::TimeSlot) -> Self {

        //convert time_range
        let start: TimeOfWeek = unwrap_bound(timeslot.time_range.start).into();
        let end: TimeOfWeek = unwrap_bound(timeslot.time_range.end).into();
        let schedule = Schedule::new(timeslot.year, &timeslot.season);

        let shortname = timeslot.shortname;

        Self { shortname, start, end, schedule }
    }
}

/// Unwraps a Bound enum, returning the value it wraps if there is one,
/// and panicing if it is Unbounded
fn unwrap_bound<T>(bound: Bound<T>) -> T {
    match bound {
        Included(inner) => inner,
        Excluded(inner) => inner,
        Unbounded => panic!("cannot unwrap unbounded"), }
}

/// A wrapper type around a vector of timeslots to make operating on them 
/// cleaner. Using a wrapper type like this allows us to implement the From
/// trait.
struct TimeSlots(Vec<TimeSlot>);
impl TimeSlots {
    fn timeslots(self) -> Vec<TimeSlot> {
        self.0
    }
}

impl From<Vec<postgres::TimeSlot>> for TimeSlots {
    fn from(timeslots: Vec<postgres::TimeSlot>) -> Self {
        let mut output: Vec<TimeSlot> = vec![];
        for timeslot in timeslots {
            output.push(timeslot.into());
        }
        Self(output) 
    }
}

// ----------
// UNIT TESTS
// ----------

#[cfg(test)]
mod tests{
    use super::*;
    use chrono::Utc;

    #[async_std::test]
    async fn test() {
        let schedule_raw = postgres::query_schedule(2020, "FALL").await;
        let mut schedule: Vec<TimeSlot> = vec![];
        for timeslot in schedule_raw {
            schedule.push(timeslot.into());
        }

        // panic!("{:#?}", schedule);
    }
}
