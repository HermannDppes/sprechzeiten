use std::cmp::{Ord, Ordering};

/// A day of the week (Mon – Fri, since noone can be reached on the weekend).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Day {
	Mon,
	Tue,
	Wed,
	Thu,
	Fri,
}

impl Day {
	// TODO: Is there a sensible trait I should implement instead?
	pub fn next(&self) -> Day {
		match self {
			Day::Mon => Day::Tue,
			Day::Tue => Day::Wed,
			Day::Wed => Day::Thu,
			Day::Thu => Day::Fri,
			Day::Fri => panic!(),
		}
	}
}

/// A time of the day in single-minute precision.
/// (Though one would expect that much lower resolutions should be sufficient
/// for the vast majority of cases. Phone times so rarely start at 13:07.)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clock {
	hours: u8,
	minutes: u8,
}

impl Clock {
	pub fn new(hours: u8, minutes: u8) -> Clock {
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

/// A time of the week.
#[derive(Debug)]
pub struct Time {
	day: Day,
	clock: Clock,
}

/// The timing information of a single contiguous reachability by phone.
#[derive(Debug, Clone)]
pub struct OfficeHour {
	day: Day,
	begin: Clock,
	end: Clock,
}

impl OfficeHour {
	pub fn new(day: Day, begin: Clock, end: Clock) -> OfficeHour {
		OfficeHour { day, begin, end }
	}

	fn contains(&self, time: &Time) -> bool {
		let same_day = self.day == time.day;
		let after_begin = self.begin <= time.clock;
		let before_end = self.end > time.clock;
		same_day && after_begin && before_end
	}
}

/// A set of `OfficeHour`s.
#[derive(Debug)]
pub struct OfficeHours {
	// TODO: Probably want to not have that public.
	/// The internal data which is only temporarily even a public field.
	pub data: Vec<OfficeHour>,
}

impl OfficeHours {
	// TODO: Why return `None` and `Some(non-empty-list)` instead of just
	//       returning the empty list when no office hours match?
	// TODO: I don't even see why this function would be useful at all.
	//       Would I not want to filter `Offices` by their associated
	//       office hour having `.filter_time(t).is_some() == true`?
	/// Filter out the subset of `OfficeHour`s taking place at `time`.
	pub fn filter_time(&self, time: &Time) -> Option<OfficeHours> {
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
		assert_eq!(Day::Mon.next(), Day::Tue);
	}

	#[test]
	#[should_panic]
	fn after_friday() {
		Day::Fri.next();
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
