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

named!(day_range<CompleteStr, Days>,
	do_parse!(
		begin: day >>
		tag!(" – ") >>
		end: day >>
		// FIXME
		({let mut days: Days = Days::new(); days.insert(begin, end); days})
	)
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
}
