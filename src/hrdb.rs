use nom;
use nom::types::CompleteStr;

use super::*;

use std::str::FromStr;

fn stringify(str: CompleteStr) -> String {
	String::from_str(str.as_ref()).unwrap()
}

named!(names<CompleteStr, Vec<String>>,
	separated_list!(tag!(", "), map!(is_not!(",\n"), stringify))
);

named!(phone_numbers<CompleteStr, Vec<String>>,
	separated_list!(tag!(", "), map!(nom::digit, stringify))
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

named!(time<CompleteStr, Clock>,
	do_parse!(
		hours: small_number >>
		tag!(":") >>
		minutes: small_number >>
		(Clock::new(hours, minutes))
	)
);

named!(time_pair<CompleteStr, (Clock, Clock)>,
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

named_args!(add_times<'a>(office: &mut Office) <CompleteStr<'a>, ()>,
	do_parse!(
		tag!("\n") >>
		days: days >>
		tag!(": ") >>
		times: separated_list!(tag!(", "), time_pair) >>
		(office.add_times(ranges_from_days_times(days, times)))
	)
);

named_args!(add_comment<'a>(office: &mut Office) <CompleteStr<'a>, ()>,
	do_parse!(
		tag!("\n") >>
		comment: is_not!("\n") >>
		(office.add_comment(String::from_str(comment.as_ref()).unwrap()))
	)
);

fn add_info<'a>(
	input: CompleteStr<'a>,
	office: &mut Office,
) -> nom::IResult<CompleteStr<'a>, ()> {
	if let Ok((rest, _)) = add_times(input, office) {
		Ok((rest, ()))
	} else if let Ok((rest, _)) = add_comment(input, office) {
		Ok((rest, ()))
	} else {
		use nom::*;
		Err(Err::Error(Context::Code(input, ErrorKind::Custom(0))))
	}
}

named!(base_office<CompleteStr, Office>,
	do_parse!(
		names: names >>
		tag!("\n") >>
		phones: phone_numbers >>
		(Office::new(names, phones))
	)
);

fn office(input: CompleteStr) -> nom::IResult<CompleteStr, Office> {
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

named!(pub offices<CompleteStr, Vec<Office>>,
	separated_list!(tag!("\n\n"), office)
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
		assert_eq!(
			res,
			vec![Day::Mo, Day::Di, Day::Mi, Day::Do, Day::Fr]
		);
	}

	#[test]
	fn test_time() {
		let (_, res) = time(CompleteStr("10:38")).unwrap();
		assert_eq!(res, Clock::new(10, 38));
	}
}
