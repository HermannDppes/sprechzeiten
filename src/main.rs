extern crate sprechzeiten;

use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut f = File::open("data/therapeuten.hrdb").expect("Not found.");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("Misread.");
	let output = sprechzeiten::process(&contents);
	println!("{}", output);
}
