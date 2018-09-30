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

#[derive(Debug)]
pub struct Office {
	names: Vec<String>,
	phones: Vec<String>,
	times: Vec<TimeRange>,
	comments: Vec<String>,
}

impl Office {
	fn new(names: Vec<String>, phones: Vec<String>) -> Office {
		let times = Vec::new();
		let comments = Vec::new();
		Office {
			names,
			phones,
			times,
			comments,
		}
	}

	fn add_times(&mut self, mut new_times: Vec<TimeRange>) {
		self.times.append(&mut new_times);
	}

	fn add_comment(&mut self, comment: String) {
		self.comments.push(comment);
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
struct Clock {
	hours: u8,
	minutes: u8,
}

impl Clock {
	fn new(hours: u8, minutes: u8) -> Clock {
		assert!(hours <= 23);
		assert!(minutes <= 59);
		Clock { hours, minutes }
	}
}

#[derive(Debug)]
struct Time {
	day: Day,
	clock: Clock,
}

#[derive(Debug)]
struct TimeRange {
	day: Day,
	begin: Clock,
	end: Clock,
}

impl TimeRange {
	fn new(day: Day, begin: Clock, end: Clock) -> TimeRange {
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
	fn clock_works() {
		assert_eq!(
			Clock::new(11, 23),
			Clock {
				hours: 11,
				minutes: 23
			}
		);
	}

	#[test]
	#[should_panic]
	fn clock_fail_hours() {
		Clock::new(24, 13);
	}

	#[test]
	#[should_panic]
	fn clock_fail_minutes() {
		Clock::new(20, 60);
	}
}
