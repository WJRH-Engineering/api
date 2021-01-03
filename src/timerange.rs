/// Custom Time Range type for representing the postgres timerange type, used to store information
/// about a show's start and end time on the schedule. Times are stored as the NaiveDateTime type
/// from the chrono module. Naive indicating that they do not store time zone information.

use std::str::FromStr;
use std::fmt;
use std::fmt::Display;

use chrono::NaiveDateTime;
use std::ops::Range;

use std::string::ParseError;

use regex::Regex;

pub struct TimeRange {
	pub start: NaiveDateTime,
	pub end: NaiveDateTime,
}

// rust documentation recommends wrapping regex defenitions in a lazy static block. This is because
// they can be computationally expensive, and the lazy static block ensures that they are compiled
// exactly once and not recompiled every time they are needed.
use lazy_static::*;
lazy_static! {
	static ref REGEX: Regex = Regex::new(r#"^\["(?P<start>.*?)","(?P<end>.*?)"\)$"#).unwrap();
}

impl FromStr for TimeRange {
	type Err = chrono::ParseError;
	fn from_str(input: &str) -> Result<Self, Self::Err> {

		// split timerange expression into start and end time using a regular expression
		// TODO: replace unwrap statements with proper error handling
		let captures = REGEX.captures(input).unwrap();
		let start_string = captures.name("start").unwrap().as_str();
		let end_string = captures.name("end").unwrap().as_str();

		// parse the start and end time into a NaiveDateTime struct using the parse_from_str method
		let format = "%Y-%m-%d %H:%M:%S";
		let start = chrono::NaiveDateTime::parse_from_str(start_string, format)?;
		let end = chrono::NaiveDateTime::parse_from_str(end_string, format)?;

		return Ok(TimeRange{start, end})
	}
}

impl Display for TimeRange {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[\"{}\",\"{}\")", self.start, self.end)
	}
}

// unit tests
#[cfg(test)]
mod tests {
	use super::*;

	use chrono::*;

	#[test]
	fn test_1() {
		let testcase = "[\"1996-01-08 00:00:00\",\"1996-01-08 00:00:00\")";
		let timerange = TimeRange::from_str(testcase)
			.expect("Error Parsing String");

		assert_eq!(testcase, timerange.to_string());
		assert_eq!(timerange.start.hour(), 0);
		assert_eq!(timerange.start.day(), 8);
		assert_eq!(timerange.end.hour(), 0);
		assert_eq!(timerange.end.day(), 8);
	}
}
