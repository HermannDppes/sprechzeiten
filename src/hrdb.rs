//! A module to parse a textual specification of offices and their offices
//! hours into the internal datatypes of this crate using Nom.
//!
//! The format is human writable enough for me but not very lenient, it expects
//! exact adherence to its not-always-obvious for everyone layout and does not
//! give sensible error messages so far. This is not expected to change unless
//! other people start using the software.
//!
//! Also, the format is based on German shorthands and conventions, which, too,
//! is not expected to change unless the programm – against my honest
//! expectations –, finds some international usage.
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

use super::time::{Clock, Day, OfficeHour};
use super::*;

use std::str::FromStr;

/// Nom parser for a list of names.
///
/// Names are expected to be in a single line, separated by a comma and space
/// and can be any valid UTF-8 sequence containing neither commas nor newlines,
/// which covers all names it needed to cover so far.
fn names(input: &str) -> IResult<&str, Names> {
	nom::combinator::map(
		nom::multi::separated_list1(
			nom::bytes::complete::tag(", "),
			nom::combinator::map(
				nom::bytes::complete::is_not(",\n"),
				Name::from,
			),
		),
		Names::from,
	)(input)
}

/// Nom parser for a single phone number.
///
/// Phone numbers are currenly expected to use only decimal digits
/// and no other symbols.
fn phone_number(input: &str) -> IResult<&str, Phone> {
	nom::combinator::map(
		nom::character::complete::digit1,
		|s: &str| Phone::from_str(&s.as_ref()).unwrap(),
	)(input)
}

/// Nom parser for a comma-and-space separated list of phone numbers.
fn phone_numbers(input: &str) -> IResult<&str, Phones> {
	nom::combinator::map(
		nom::multi::separated_list0(
			nom::bytes::complete::tag(", "),
			phone_number,
		),
		Phones::from,
	)(input)
}

/// Nom parser for a single day of the week.
///
/// The parser expects the day to be encoded by its standard German shorthand.
fn day(input: &str) -> IResult<&str, Day> {
	nom::branch::alt((
		nom::combinator::value(
			Day::Mon,
			nom::bytes::complete::tag("Mo"),
		),
		nom::combinator::value(
			Day::Tue,
			nom::bytes::complete::tag("Di"),
		),
		nom::combinator::value(
			Day::Wed,
			nom::bytes::complete::tag("Mi"),
		),
		nom::combinator::value(
			Day::Thu,
			nom::bytes::complete::tag("Do"),
		),
		nom::combinator::value(
			Day::Fri,
			nom::bytes::complete::tag("Fr"),
		),
	))(input)
}

/// Nom parser to parse a single day into a list of days.
fn single_day(day: Day) -> Vec<Day> {
	let mut days = Vec::new();
	days.push(day);
	days
}

/// Turning two days into the list of days between them (endpoints included).
fn days_from_range(begin: Day, end: Day) -> Vec<Day> {
	let mut days = single_day(begin.clone());
	let mut day = begin;
	while day != end {
		day = day.next();
		days.push(day.clone());
	}
	days
}

#[allow(rustdoc::invalid_rust_codeblocks)]
/// Nom parser to parse a range of days into a list.
///
/// A range may be specified by two days separated by a hyphen surround by a
/// space on either side.
///
/// # Example
///
/// ```ignore
/// day_range("Mo - Fr") ≈ Ok(("", vec![Day::Mon, Day::Tue, Day::Wed]));
/// ```
fn day_range(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, begin) = day(input)?;
	let (input, _) = nom::bytes::complete::tag(" – ")(input)?;
	let (input, end) = day(input)?;
	Ok((input, days_from_range(begin, end)))
}

/// Helper for `day_list`, parsing a single entry of the comma-and-space
/// separated list (and in fact being quite agonstic about any commas
/// and spaces).
fn day_list_elem(input: &str) -> IResult<&str, Vec<Day>> {
	nom::branch::alt((
		day_range,
		nom::combinator::map(day, single_day),
	))(input)
}

/// Merge two lists of `Day`s into a single list.
fn merge_days(mut a: Vec<Day>, mut b: Vec<Day>) -> Vec<Day> {
	a.append(&mut b);
	a
}

/// Helper for `day_list`, parsing the comma, the space and the next entry
/// in the list.
fn day_list_continuation(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, _) = nom::bytes::complete::tag(", ")(input)?;
	day_list_elem(input)
}

// TODO: Figure out how to do this without cloning. It was possible up to
//       Nom 6 but then fold was changed to take `FnMut() -> R` instead
//       of `R` as the initial value argument.
// TODO: Should this be ordered/deduplicated?
//       Probably not, since it makes parsing slower and we can probably
//       design the downstream uses in a way that does not depend on any
//       order or duplication of these but it needs to be reevaluated
//       when the use cases are clearer.
/// Nom parser for a comma-and-space separated list of days and day ranges
/// into a single list of `Day`s.
fn day_list(input: &str) -> IResult<&str, Vec<Day>> {
	let (input, first) = day_list_elem(input)?;
	let (input, list) = nom::multi::fold_many0(
		day_list_continuation,
		|| first.clone(),
		merge_days,
	)(input)?;
	Ok((input, list))
}

/// Nom parser for daily occurences.
///
/// Parses the shorthand `"Tgl"` to a list of all the five `Day`s in order.
fn daily(input: &str) -> IResult<&str, Vec<Day>> {
	nom::combinator::value(
		days_from_range(Day::Mon, Day::Fri),
		nom::bytes::complete::tag("Tgl"),
	)(input)
}

/// Nom parser for a days-of-the-week specification.
///
/// The input can either be the specification `"Tgl"` or a comma-and-space
/// separated list of day shorthands and day ranges. The result is the union
/// of these, containing all specified days.
fn days(input: &str) -> IResult<&str, Vec<Day>> {
	nom::branch::alt((daily, day_list))(input)
}

/// Nom parser for a small number.
///
/// Expects the input to be a decimal representation of an integer betwenn
/// 0 and 255 (inclusive).
fn small_number(input: &str) -> IResult<&str, u8> {
	nom::combinator::map(
		nom::character::complete::digit1,
		|str: &str| FromStr::from_str(str).unwrap(),
	)(input)
}

/// Nom parser for a time specification.
///
/// Expects the input to be of the form HH:MM in a 24 hour format.
fn time(input: &str) -> IResult<&str, Clock> {
	let (input, hours) = small_number(input)?;
	let (input, _) = nom::bytes::complete::tag(":")(input)?;
	let (input, minutes) = small_number(input)?;
	Ok((input, Clock::new(hours, minutes)))
}

/// Nom parser for a time range.
///
/// Expects the input to be a pair of times separated by space-dash-space
/// and returns the pair of these two times-of-day.
fn time_pair(input: &str) -> IResult<&str, (Clock, Clock)> {
	nom::combinator::map(
		nom::sequence::tuple((
			time,
			nom::bytes::complete::tag(" - "),
			time,
		)),
		|(a, _, b)| (a, b),
	)(input)
}

/// Turns a list of days-of-week and list of pairs of times-of-day
/// into the corresponding list of `OfficeHour`s.
fn office_hours_from_days_and_times(
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

/// Nom parser modifying an `Office` by adding the `OfficeHour`s specified
/// by the input.
///
/// The office hours should be specified by specifying the days, then a colon
/// and a space and then specifying the time ranges common to these days.
/// When not all days have the same time ranges, multiple such specifications
/// must be made on separate lines to be merged by a higher level parser.
fn add_times<'a>(
	input: &'a str,
	office: &mut Office,
) -> nom::IResult<&'a str, ()> {
	let (input, (_, days, _, times)) = nom::sequence::tuple((
		nom::bytes::complete::tag("\n"),
		days,
		nom::bytes::complete::tag(": "),
		nom::multi::separated_list1(
			nom::bytes::complete::tag(", "),
			time_pair,
		),
	))(input)?;
	// TODO: This line is over 80 characters in width.
	Ok((input, office.add_times(office_hours_from_days_and_times(days, times))))
}

/// Nom parser modifying an `Office` by adding the `Comment` specified
/// by the input.
///
/// A comment is any valid UTF-8 string not containing new lines and not
/// obeying the formatting rules for office hour specifications.
fn add_comment<'a>(
	input: &'a str,
	office: &mut Office,
) -> IResult<&'a str, ()> {
	let (input, _) = nom::bytes::complete::tag("\n")(input)?;
	let (input, comment) = nom::combinator::map(
			nom::bytes::complete::is_not("\n"),
			Comment::from,
		)(input)?;
	Ok((input, office.add_comment(comment)))
}

/// Nom parser modifying an `Office` by adding the `OfficeHour`s or `Comment`
/// specified by the input.
fn add_info<'a>(
	input: &'a str,
	office: &mut Office,
) -> nom::IResult<&'a str, ()> {
	// TODO: Why is this not a `nom::combinator::alt` application?
	if let Ok((rest, _)) = add_times(input, office) {
		Ok((rest, ()))
	} else if let Ok((rest, _)) = add_comment(input, office) {
		Ok((rest, ()))
	} else {
		use nom::*;
		// FIXME: `ErrorKind::Tag` is not the correct error
		Err(Err::Error(nom::error::Error {
			input,
			code: nom::error::ErrorKind::Tag,
		}))
	}
}

/// Nom parser for the basic information of an `Office`.
///
/// The input should be on two lines, the first containing the list of names,
/// the second the list of phone numbers.
fn base_office(input: &str) -> IResult<&str, Office> {
	let (input, (names, _, phones)) = nom::sequence::tuple((
		names,
		nom::bytes::complete::tag("\n"),
		phone_numbers,
	))(input)?;
	Ok((input, Office::new(names, phones)))
}

/// Nom parser for an entire `Office`.
///
/// An office should be specified by the basic information on the first two
/// lines and then any number of lines with office hours or comments. An
/// office ends with the first empty line after the first two.
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

/// Nom parser for a list of `Office`s.
///
/// Offices should be separated by a single empty line.
// TODO: Should this insist on parsing an EOF at the end?
pub fn offices(input: &str) -> IResult<&str, Offices> {
	nom::combinator::map(
		nom::multi::separated_list0(
			nom::bytes::complete::tag("\n\n"),
			office,
		),
		Offices::from
	)(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day() {
		let (_, res) = day("Mo").unwrap();
		assert_eq!(res, Day::Mon);
	}

	#[test]
	fn test_day_range() {
		let (_, res) = day_range("Di – Do").unwrap();
		assert_eq!(res, vec![Day::Tue, Day::Wed, Day::Thu]);
	}

	#[test]
	fn test_day_list() {
		let (_, res) = day_list("Mo, Mi – Fr").unwrap();
		assert_eq!(res, vec![Day::Mon, Day::Wed, Day::Thu, Day::Fri]);
	}

	#[test]
	fn test_days() {
		let (_, res) = days("Tgl").unwrap();
		assert_eq!(
			res,
			vec![Day::Mon, Day::Tue, Day::Wed, Day::Thu, Day::Fri]
		);
	}

	#[test]
	fn test_time() {
		let (_, res) = time("10:38").unwrap();
		assert_eq!(res, Clock::new(10, 38));
	}
}
