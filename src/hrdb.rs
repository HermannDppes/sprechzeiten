// TODO: Since Nom 7 the macros with which most of this was implemented
//       have been removed. The code resulting from rewriting all of it
//       to use the available functions is highly inconsistent – especially
//       everything that used to be `do_parse!`. At some point, decide on
//       best practices and go back to apply them consistently.
//
//       Is it better to have lots of `let (input, ` and `(input)?`
//       boilerplate? Or is it better to have that in a single tuple parser?
//       The disadvantages of boilerplate are obvious, but at least
//       the variable to which stuff is bound and the parsers generating
//       the stuff are on the same line – with tuples no such connection
//       exists and one has to count out the n-th position in each tuple
//       to know which variables belong to which parsers.
//
//       Is it better to rename `input` to `i` to shorten the boilerplate,
//       at the expense of a standardized, higly-self-explanatory parameter
//       name?
//
//       Should the results of tuple parsers be mapped to the final result
//       (at the cost of one layer indentation/structure)?
//       Or is it better to have `Ok((input, ` boilerplate
//       that at least fits the linear structure of the code?

use nom;
use nom::IResult;

use super::*;
use super::time::{Day, Clock, OfficeHour};

use std::str::FromStr;

fn names(input: &str) -> IResult<&str, Names> {
	nom::combinator::map(
		nom::multi::separated_list1(
			nom::bytes::complete::tag(", "),
			nom::combinator::map(
				nom::bytes::complete::is_not(",\n"),
				Name::from
			)
		),
		Names::from
	)(input)
}

fn phone_number(input: &str) -> IResult<&str, Phone> {
	nom::combinator::map(
		nom::character::complete::digit1,
		|s: &str| Phone::from_str(&s.as_ref()).unwrap()
	)(input)
}

fn phone_numbers(input: &str) -> IResult<&str, Phones> {
	nom::combinator::map(
		nom::multi::separated_list0(
			nom::bytes::complete::tag(", "),
			phone_number
		),
		Phones::from
	)(input)
}

fn day(input: &str) -> IResult<&str, Day> {
	nom::branch::alt((
		nom::combinator::value(Day::Mo, nom::bytes::complete::tag("Mo")),
		nom::combinator::value(Day::Di, nom::bytes::complete::tag("Di")),
		nom::combinator::value(Day::Mi, nom::bytes::complete::tag("Mi")),
		nom::combinator::value(Day::Do, nom::bytes::complete::tag("Do")),
		nom::combinator::value(Day::Fr, nom::bytes::complete::tag("Fr"))
	))(input)
}

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

fn day_range(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, begin) = day(input)?;
	let (input, _) = nom::bytes::complete::tag(" – ")(input)?;
	let (input, end) = day(input)?;
	Ok((input, days_from_range(begin, end)))
}

fn day_list_elem(input: &str) -> IResult<&str, Vec<Day>> {
	nom::branch::alt((
		day_range,
		nom::combinator::map(day, single_day)
	))(input)
}

fn merge_days(mut a: Vec<Day>, mut b: Vec<Day>) -> Vec<Day> {
	a.append(&mut b);
	a
}

fn day_list_continuation(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, _) = nom::bytes::complete::tag(", ")(input)?;
	day_list_elem(input)
}

fn day_list(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, first) = day_list_elem(input)?;
	nom::multi::fold_many0(
		day_list_continuation,
		first,
		merge_days
	)(input)
}

fn daily(input: &str) -> IResult<&str, Vec<Day>> {
	nom::combinator::value(
		days_from_range(Day::Mo, Day::Fr),
		nom::bytes::complete::tag("Tgl")
	)(input)
}

fn days(input: &str) -> IResult<&str, Vec<Day>> {
	nom::branch::alt((
		daily,
		day_list
	))(input)
}

fn small_number(input: &str) -> IResult<&str, u8> {
	nom::combinator::map(
		nom::character::complete::digit1,
		|str: &str| FromStr::from_str(str).unwrap()
	)(input)
}

fn time(input: &str) -> IResult<&str, Clock> {
	let (input, hours) = small_number(input)?;
	let (input, _) = nom::bytes::complete::tag(":")(input)?;
	let (input, minutes) = small_number(input)?;
	Ok((input, Clock::new(hours, minutes)))
}

fn time_pair(input: &str) -> IResult<&str, (Clock, Clock)> {
	nom::combinator::map(
		nom::sequence::tuple((
			time,
			nom::bytes::complete::tag(" - "),
			time
		)),
		|(a, _, b)| (a, b)
	)(input)
}

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

fn add_times<'a>(
	input: &'a str,
	office: &mut Office
) -> nom::IResult<&'a str, ()> {
	let (input, (_, days, _, times)) = nom::sequence::tuple((
		nom::bytes::complete::tag("\n"),
		days,
		nom::bytes::complete::tag(": "),
		nom::multi::separated_list1(
			nom::bytes::complete::tag(", "),
			time_pair
		)
	))(input)?;
	Ok((input, office.add_times(ranges_from_days_times(days, times))))
}

fn add_comment<'a>(input: &'a str, office: &mut Office) -> IResult<&'a str, ()> {
	let (input, _) = nom::bytes::complete::tag("\n")(input)?;
	let (input, comment) = nom::combinator::map(
			nom::bytes::complete::is_not("\n"),
			Comment::from
	)(input)?;
	Ok((input, office.add_comment(comment)))
}

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

fn base_office(input: &str) -> IResult<&str, Office> {
	let (input, (names, _, phones)) = nom::sequence::tuple((
		names,
		nom::bytes::complete::tag("\n"),
		phone_numbers
	))(input)?;
	Ok((input, Office::new(names, phones)))
}

fn office(input: &str) -> IResult<&str, Office> {
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

pub fn offices(input: &str) -> IResult<&str, Vec<Office>> {
	nom::multi::separated_list0(
		nom::bytes::complete::tag("\n\n"),
		office
	)(input)
}

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
