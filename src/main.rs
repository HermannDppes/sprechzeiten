extern crate sprechzeiten;

use std::fs::File;
use std::io::prelude::*;

use crate::sprechzeiten::{hrdb, time};

fn main() {
	let mut f = File::open("data/therapeuten.hrdb").expect("Not found.");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("Misread.");
	let (_, offices) = hrdb::offices(contents.as_ref())
		.expect("Parsing unsuccessful.");
	let now = time::Time::now().expect("Unable to get current local time");
	let current_offices = offices.filter_time(&now);
	println!("{}", current_offices);
}
