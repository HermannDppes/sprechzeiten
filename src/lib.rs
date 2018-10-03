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

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Name {
	data: String,
}

impl<T: AsRef<str>> From<T> for Name {
	fn from(str: T) -> Name {
		let data = String::from(str.as_ref());
		Name { data }
	}
}

impl fmt::Display for Name {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}", self.data)
	}
}

#[derive(Debug, Clone)]
struct Names {
	data: Vec<Name>,
}

impl From<Vec<Name>> for Names {
	fn from(data: Vec<Name>) -> Names {
		Names { data }
	}
}

impl fmt::Display for Names {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.data.iter().peekable();
		if None == iter.peek() {
			write!(fmt, "")
		} else {
			loop {
				// This cannot panic! due to the peeks
				let next = iter.next().unwrap();
				let res = write!(fmt, "{}", next);
				if None == iter.peek() {
					break res;
				}
				write!(fmt, ", ");
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Phone {
	data: String,
}

#[derive(Debug)]
enum PhoneErr {
	InvalidChar,
}

impl FromStr for Phone {
	type Err = PhoneErr;

	fn from_str(src: &str) -> Result<Phone, Self::Err> {
		if src.chars().all(|c| c.is_digit(10)) {
			let data = String::from(src);
			Ok(Phone { data })
		} else {
			Err(PhoneErr::InvalidChar)
		}
	}
}

impl fmt::Display for Phone {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}", self.data)
	}
}

#[derive(Debug, Clone)]
struct Phones {
	data: Vec<Phone>,
}

impl From<Vec<Phone>> for Phones {
	fn from(data: Vec<Phone>) -> Phones {
		Phones { data }
	}
}

impl fmt::Display for Phones {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.data.iter().peekable();
		if None == iter.peek() {
			write!(fmt, "")
		} else {
			loop {
				// This cannot panic! due to the peeks
				let next = iter.next().unwrap();
				let res = write!(fmt, "{}", next);
				if None == iter.peek() {
					break res;
				}
				write!(fmt, ", ");
			}
		}
	}
}

#[derive(Debug, Clone)]
struct Comment {
	data: String,
}

impl<T: AsRef<str>> From<T> for Comment {
	fn from(src: T) -> Comment {
		let data = String::from(src.as_ref());
		Comment { data }
	}
}

#[derive(Debug, Clone)]
struct Comments {
	data: Vec<Comment>,
}

impl From<Vec<Comment>> for Comments {
	fn from(data: Vec<Comment>) -> Comments {
		Comments { data }
	}
}

impl Comments {
	fn push(&mut self, comment: Comment) {
		self.data.push(comment)
	}
}

#[derive(Debug)]
pub struct Office {
	names: Names,
	phones: Phones,
	times: OfficeHours,
	comments: Comments,
}

impl Office {
	fn new(names: Names, phones: Phones) -> Office {
		let times = OfficeHours { data: Vec::new() };
		let comments = Comments { data: Vec::new() };
		Office {
			names,
			phones,
			times,
			comments,
		}
	}

	fn add_times(&mut self, mut new_times: Vec<OfficeHour>) {
		self.times.data.append(&mut new_times);
	}

	fn add_comment(&mut self, comment: Comment) {
		self.comments.push(comment);
	}

	fn filter_time(&self, time: &Time) -> Option<Office> {
		let names = self.names.clone();
		let phones = self.phones.clone();
		let maybe_times = self.times.filter_time(&time);
		let comments = self.comments.clone();
		if let Some(times) = maybe_times {
			Some(Office {
				names,
				phones,
				times,
				comments,
			})
		} else {
			None
		}
	}
}

pub struct Offices {
	data: Vec<Office>,
}

impl Offices {
	fn filter_time(&self, time: &Time) -> Offices {
		let mut data = Vec::new();
		for office in &self.data {
			if let Some(office) = office.filter_time(&time) {
				data.push(office);
			}
		}
		Offices { data }
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

impl PartialOrd for Clock {
	fn partial_cmp(&self, other: &Clock) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Clock {
	fn cmp(&self, other: &Clock) -> Ordering {
		if self.hours < other.hours {
			Ordering::Less
		} else if self.hours > other.hours {
			Ordering::Greater
		} else {
			self.minutes.cmp(&other.minutes)
		}
	}
}

#[derive(Debug)]
struct Time {
	day: Day,
	clock: Clock,
}

#[derive(Debug, Clone)]
struct OfficeHour {
	day: Day,
	begin: Clock,
	end: Clock,
}

impl OfficeHour {
	fn new(day: Day, begin: Clock, end: Clock) -> OfficeHour {
		OfficeHour { day, begin, end }
	}

	fn contains(&self, time: &Time) -> bool {
		let same_day = self.day == time.day;
		let after_begin = self.begin <= time.clock;
		let before_end = self.end > time.clock;
		same_day && after_begin && before_end
	}
}

#[derive(Debug)]
struct OfficeHours {
	data: Vec<OfficeHour>,
}

impl OfficeHours {
	fn filter_time(&self, time: &Time) -> Option<OfficeHours> {
		let mut data = Vec::<OfficeHour>::new();
		for oh in self.data.iter().filter(|oh| oh.contains(&time)) {
			data.push(oh.clone());
		}
		if data.len() > 0 {
			Some(OfficeHours { data })
		} else {
			None
		}
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
