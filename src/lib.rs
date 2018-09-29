pub fn process(str: &str) -> &str {
	let (input, offices) =
		hrdb::offices(nom::types::CompleteStr(str)).unwrap();
	println!("{:?}", offices);
	println!("{}", input.as_ref());
	""
}

#[macro_use]
extern crate nom;

mod hrdb;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Office {
	names: Vec<String>,
	phones: Vec<String>,
	times: Vec<TimeRange>,
}

impl Office {
	fn new(names: Vec<String>, phones: Vec<String>) -> Office {
		let times = Vec::new();
		Office {
			names,
			phones,
			times,
		}
	}

	fn add_times(&mut self, mut new_times: Vec<TimeRange>) {
		self.times.append(&mut new_times);
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Day {
	Mo,
	Di,
	Mi,
	Do,
	Fr,
}

impl Day {
	// TODO: Is there a sensible trait I should implement instead?
	fn next(&self) -> Day {
		match self {
			Day::Mo => Day::Di,
			Day::Di => Day::Mi,
			Day::Mi => Day::Do,
			Day::Do => Day::Fr,
			Day::Fr => panic!(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Time {
	hours: u8,
	minutes: u8,
}

impl Time {
	fn new(hours: u8, minutes: u8) -> Time {
		assert!(hours <= 23);
		assert!(minutes <= 59);
		Time { hours, minutes }
	}
}

#[derive(Debug)]
struct TimeRange {
	day: Day,
	begin: Time,
	end: Time,
}

impl TimeRange {
	fn new(day: Day, begin: Time, end: Time) -> TimeRange {
		TimeRange { day, begin, end }
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
		assert_eq!(
			Time::new(11, 23),
			Time {
				hours: 11,
				minutes: 23
			}
		);
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
