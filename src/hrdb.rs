use nom;
use nom::types::CompleteStr;

use super::*;

use std::str::FromStr;

named!(names<CompleteStr, Vec<CompleteStr>>,
	separated_list!(tag!(", "), is_not!(",\n"))
);

named!(phone_numbers<CompleteStr, Vec<CompleteStr>>,
	separated_list!(tag!(", "), nom::digit)
);

named!(day<CompleteStr, Day>,
	alt!(
		value!(Day::Mo, tag!("Mo")) |
		value!(Day::Di, tag!("Di")) |
		value!(Day::Mi, tag!("Mi")) |
		value!(Day::Do, tag!("Do")) |
		value!(Day::Fr, tag!("Fr"))
	)
);

fn single_day(day: Day) -> Vec<Day> {
	let mut days = Vec::new();
	days.push(day);
	days
}

fn days_from_range(begin: Day, end: Day) -> Vec<Day> {
	let mut days = single_day(begin.clone());
	let mut day = begin;
	while day != end {
		day = day.next();
		days.push(day.clone());
	}
	days
}

named!(day_range<CompleteStr, Vec<Day>>,
	do_parse!(
		begin: day >>
		tag!(" – ") >>
		end: day >>
		(days_from_range(begin, end))
	)
);

named!(day_list_elem<CompleteStr, Vec<Day>>,
	alt!(day_range | map!(day, single_day))
);

fn merge_days(mut a: Vec<Day>, mut b: Vec<Day>) -> Vec<Day> {
	a.append(&mut b);
	a
}

named!(day_list<CompleteStr, Vec<Day>>,
	do_parse!(
		first: day_list_elem >>
		days: fold_many0!(
			do_parse!(tag!(", ") >> e: day_list_elem >> (e)),
			first,
			merge_days
		) >>
		(days)
	)
);

named!(days<CompleteStr, Vec<Day>>,
	alt!(value!(days_from_range(Day::Mo, Day::Fr), tag!("Tgl")) | day_list)
);

named!(small_number<CompleteStr, u8>,
	map!(nom::digit, |str| FromStr::from_str(&str).unwrap())
);

named!(time<CompleteStr, Time>,
	do_parse!(
		hours: small_number >>
		tag!(":") >>
		minutes: small_number >>
		(Time::new(hours, minutes))
	)
);

named!(time_range<CompleteStr, TimeRange>,
	do_parse!(
		begin: time >>
		tag!(" – ") >>
		end: time >>
		(TimeRange::new(begin, end))
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day() {
		let (_, res) = day(CompleteStr("Mo")).unwrap();
		assert_eq!(res, Day::Mo);
	}

	#[test]
	fn test_day_range() {
		let (_, res) = day_range(CompleteStr("Di – Do")).unwrap();
		assert_eq!(res, vec![Day::Di, Day::Mi, Day::Do]);
	}

	#[test]
	fn test_day_list() {
		let (_, res) = day_list(CompleteStr("Mo, Mi – Fr")).unwrap();
		assert_eq!(res, vec![Day::Mo, Day::Mi, Day::Do, Day::Fr]);
	}

	#[test]
	fn test_days() {
		let (_, res) = days(CompleteStr("Tgl")).unwrap();
		assert_eq!(res, vec![Day::Mo, Day::Di, Day::Mi, Day::Do, Day::Fr]);
	}

	#[test]
	fn test_time() {
		let (_, res) = time(CompleteStr("10:38")).unwrap();
		assert_eq!(res, Time::new(10, 38));
	}
}
