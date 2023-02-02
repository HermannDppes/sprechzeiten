pub fn process(str: &str) -> &str {
	let (input, offices) =
		hrdb::offices(str).unwrap();
	println!("{:?}", offices);
	println!("{}", input);
	""
}

#[macro_use]
extern crate nom;

mod hrdb;
mod time;

use std::str::FromStr;

use std::fmt;

use time::{OfficeHour, OfficeHours, Time};

fn display_simple_list<T: IntoIterator>(
	lst: T,
	fmt: &mut fmt::Formatter,
) -> fmt::Result
where
	<T as IntoIterator>::Item: fmt::Display,
{
	let mut iter = lst.into_iter().peekable();
	if iter.peek().is_none() {
		write!(fmt, "")
	} else {
		loop {
			// This cannot panic! due to the peeks
			let next = iter.next().unwrap();
			let res = write!(fmt, "{}", next);
			if iter.peek().is_none() {
				break res;
			}
			write!(fmt, ", ");
		}
	}
}

#[derive(Debug, Clone)]
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
		display_simple_list(&self.data, fmt)
	}
}

#[derive(Debug, Clone)]
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
		display_simple_list(&self.data, fmt)
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
