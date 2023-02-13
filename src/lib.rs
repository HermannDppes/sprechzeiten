pub mod hrdb;
pub mod time;

use std::str::FromStr;

use std::fmt;

use crate::time::{OfficeHour, OfficeHours, Time};

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
			write!(fmt, "{}", next)?;
			if iter.peek().is_none() {
				break Ok(());
			}
			write!(fmt, ", ")?;
		}
	}
}

/// The name of a person.
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

/// A set of `Name`s.
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
	fn empty() -> Comments {
		let data = Vec::new();
		Comments { data }
	}

	fn push(&mut self, comment: Comment) {
		self.data.push(comment)
	}
}

#[derive(Debug, Clone)]
pub struct Office {
	names: Names,
	phones: Phones,
	times: OfficeHours,
	comments: Comments,
}

impl Office {
	fn new(names: Names, phones: Phones) -> Office {
		let times = OfficeHours::empty();
		let comments = Comments::empty();
		Office {
			names,
			phones,
			times,
			comments,
		}
	}

	fn add_times(&mut self, new_times: Vec<OfficeHour>) {
		self.times.append(new_times);
	}

	fn add_comment(&mut self, comment: Comment) {
		self.comments.push(comment);
	}

	fn reachable(&self, time: &Time) -> bool {
		self.times.contain(time)
	}
}

impl fmt::Display for Office {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{}\n{}\n", self.names, self.phones)?;
		for c in &self.comments.data {
			write!(fmt, "{}\n", c.data)?;
		}
		Ok(())
	}
}

pub struct Offices {
	data: Vec<Office>,
}

impl Offices {
	pub fn filter_time(&self, time: &Time) -> Offices {
		let data = self.data.iter()
			.filter(|x| x.reachable(time))
			.cloned()
			.collect();
		Offices { data }
	}
}

impl fmt::Display for Offices {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		for o in &self.data {
			write!(fmt, "{}\n", o)?;
		}
		Ok(())
	}
}

impl From<Vec<Office>> for Offices {
	fn from(data: Vec<Office>) -> Self {
		Offices { data }
	}
}
