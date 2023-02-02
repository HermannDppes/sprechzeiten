use nom;

use super::*;
use super::time::{Day, Clock, OfficeHour};

use std::str::FromStr;

named!(names<&str, Names>,
	map!(
		separated_list1!(tag!(", "), map!(is_not!(",\n"), Name::from)),
		Names::from
	)
);

named!(phone_number<&str, Phone>,
	map!(nom::character::complete::digit1, |s| Phone::from_str(&s.as_ref()).unwrap())
);

named!(phone_numbers<&str, Phones>,
	map!(separated_list0!(tag!(", "), phone_number), Phones::from)
);

named!(day<&str, Day>,
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

named!(day_range<&str, Vec<Day>>,
	do_parse!(
		begin: day >>
		tag!(" – ") >>
		end: day >>
		(days_from_range(begin, end))
	)
);

named!(day_list_elem<&str, Vec<Day>>,
	alt!(day_range | map!(day, single_day))
);

fn merge_days(mut a: Vec<Day>, mut b: Vec<Day>) -> Vec<Day> {
	a.append(&mut b);
	a
}

named!(day_list_continuation<&str, Vec<Day>>,
	do_parse!(
		tag!(", ") >>
		e: day_list_elem >>
		(e)
	)
);

named!(day_list<&str, Vec<Day>>,
	do_parse!(
		first: day_list_elem >>
		days: fold_many0!(
			day_list_continuation,
			first,
			merge_days
		) >>
		(days)
	)
);

named!(days<&str, Vec<Day>>,
	alt!(value!(days_from_range(Day::Mo, Day::Fr), tag!("Tgl")) | day_list)
);

named!(small_number<&str, u8>,
	map!(nom::character::complete::digit1, |str| FromStr::from_str(&str).unwrap())
);

named!(time<&str, Clock>,
	do_parse!(
		hours: small_number >>
		tag!(":") >>
		minutes: small_number >>
		(Clock::new(hours, minutes))
	)
);

named!(time_pair<&str, (Clock, Clock)>,
	do_parse!(
		begin: time >>
		tag!(" – ") >>
		end: time >>
		((begin, end))
	)
);

fn ranges_from_days_times(
	days: Vec<Day>,
	times: Vec<(Clock, Clock)>,
) -> Vec<OfficeHour> {
	let mut ranges = Vec::with_capacity(days.len() * times.len());
	for day in days {
		for (begin, end) in &times {
			ranges.push(OfficeHour::new(
				day.clone(),
				begin.clone(),
				end.clone(),
			));
		}
	}
	ranges
}

named_args!(add_times<'a>(office: &mut Office) <&'a str, ()>,
	do_parse!(
		tag!("\n") >>
		days: days >>
		tag!(": ") >>
		times: separated_list1!(tag!(", "), time_pair) >>
		(office.add_times(ranges_from_days_times(days, times)))
	)
);

named_args!(add_comment<'a>(office: &mut Office) <&'a str, ()>,
	do_parse!(
		tag!("\n") >>
		comment: map!(is_not!("\n"), Comment::from) >>
		(office.add_comment(comment))
	)
);

fn add_info<'a>(
	input: &'a str,
	office: &mut Office,
) -> nom::IResult<&'a str, ()> {
	if let Ok((rest, _)) = add_times(input, office) {
		Ok((rest, ()))
	} else if let Ok((rest, _)) = add_comment(input, office) {
		Ok((rest, ()))
	} else {
		use nom::*;
		// FIXME: `ErrorKind::Tag` is not the correct error
		Err(Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::Tag}))
	}
}

named!(base_office<&str, Office>,
	do_parse!(
		names: names >>
		tag!("\n") >>
		phones: phone_numbers >>
		(Office::new(names, phones))
	)
);

fn office(input: &str) -> nom::IResult<&str, Office> {
	let (mut input, mut office) = base_office(input).unwrap();
	loop {
		let res = add_info(input, &mut office);
		if let Ok((rest, _)) = res {
			input = rest;
		} else {
			break;
		}
	}
	Ok((input, office))
}

named!(pub offices<&str, Vec<Office>>,
	separated_list0!(tag!("\n\n"), office)
);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day() {
		let (_, res) = day("Mo").unwrap();
		assert_eq!(res, Day::Mo);
	}

	#[test]
	fn test_day_range() {
		let (_, res) = day_range("Di – Do").unwrap();
		assert_eq!(res, vec![Day::Di, Day::Mi, Day::Do]);
	}

	#[test]
	fn test_day_list() {
		let (_, res) = day_list("Mo, Mi – Fr").unwrap();
		assert_eq!(res, vec![Day::Mo, Day::Mi, Day::Do, Day::Fr]);
	}

	#[test]
	fn test_days() {
		let (_, res) = days("Tgl").unwrap();
		assert_eq!(
			res,
			vec![Day::Mo, Day::Di, Day::Mi, Day::Do, Day::Fr]
		);
	}

	#[test]
	fn test_time() {
		let (_, res) = time("10:38").unwrap();
		assert_eq!(res, Clock::new(10, 38));
	}
}
