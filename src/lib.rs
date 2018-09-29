pub fn process(str: &str) -> &str {
	str
}

#[macro_use]
extern crate nom;

mod hrdb;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Day {
	Mo,
	Di,
	Mi,
	Do,
	Fr
}

impl Day {
	// TODO: Is there a sensible trait I should implement instead?
	fn next(&self) -> Day {
		match self {
			Day::Mo => Day::Di,
			Day::Di => Day::Mi,
			Day::Mi => Day::Do,
			Day::Do => Day::Fr,
			Day::Fr => panic!()
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Time {
	hours: u8,
	minutes: u8
}

impl Time {
	fn new(hours: u8, minutes: u8) -> Time {
		assert!(hours <= 23);
		assert!(minutes <= 59);
		Time {hours, minutes}
	}
}

#[derive(Debug)]
struct TimeRange {
	begin: Time,
	end: Time
}

impl TimeRange {
	fn new(begin: Time, end: Time) -> TimeRange {
		TimeRange {begin, end}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn after_monday() {
		assert_eq!(Day::Mo.next(), Day::Di);
	}

	#[test]
	#[should_panic]
	fn after_friday() {
		Day::Fr.next();
	}

	#[test]
	fn time_works() {
		assert_eq!(Time::new(11, 23), Time {hours: 11, minutes: 23});
	}

	#[test]
	#[should_panic]
	fn time_fail_hours() {
		Time::new(24, 13);
	}

	#[test]
	#[should_panic]
	fn time_fail_minutes() {
		Time::new(20, 60);
	}
}
