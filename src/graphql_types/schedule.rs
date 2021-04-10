use crate::graphql_types::time_of_week::*;
use crate::data_sources::postgres;
use core::ops::Bound::*;
use crate::graphql_types::teal;
use async_graphql::*;


#[derive(Debug, SimpleObject)]
struct ScheduleID {
    year: i32,
    season: String,
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
struct TimeSlot {
	pub shortname: String,

    #[graphql(skip)]
    pub start: TimeOfWeek,

    #[graphql(skip)]
    pub end: TimeOfWeek,

    #[graphql(skip)]
    pub schedule: ScheduleID,
}


use crate::data_sources::redis;

#[ComplexObject]
impl TimeSlot {
    pub async fn program(&self) -> teal::Program {
        redis::Program::get(&self.shortname).into()
    }
}


#[derive(Default)]
pub struct ScheduleQuery;

#[Object]
impl ScheduleQuery {
    pub async fn schedule(&self) -> Vec<TimeSlot> {
       let schedule: Schedule = postgres::query_schedule(2021, "SPRING")
           .await.into();
       schedule.timeslots()
    }
}

use core::ops::Bound;

/// Unwraps a Bound enum, returning the value it wraps if there is one,
/// and panicing if it is Unbounded
fn unwrap_bound<T>(bound: Bound<T>) -> T {
    match bound {
        Included(inner) => inner,
        Excluded(inner) => inner,
        Unbounded => panic!("cannot unwrap unbounded"),
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

        let schedule = ScheduleID{
            season: timeslot.season,
            year: timeslot.year,
        };  
        let shortname = timeslot.shortname;

        Self { shortname, start, end, schedule }
    }
}

/// A wrapper type around a vector of timeslots to make operating on them 
/// cleaner. Using a wrapper type like this allows us to implement the From
/// trait.
struct Schedule(Vec<TimeSlot>);
impl Schedule {
    fn timeslots(self) -> Vec<TimeSlot> {
        self.0
    }
}

impl From<Vec<postgres::TimeSlot>> for Schedule {
    fn from(timeslots: Vec<postgres::TimeSlot>) -> Self {
        let mut output: Vec<TimeSlot> = vec![];
        for timeslot in timeslots {
            output.push(timeslot.into());
        }
        Schedule(output) 
    }
}


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
