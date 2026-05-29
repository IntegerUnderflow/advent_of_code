mod year2025;

use std::env;


fn main() {
	println!(">>>	Starting Advent of Code");

	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 {
		println!(">> No runtime args provided");
		println!(">>>	Exiting Advent of Code");
		return;
	}

	year2025::day1::run();
	// parse_args(&args);

}

fn parse_args(args: &Vec<String>) {
	println!(">> Parsing Args");
	let mut i = 1;

	while i < args.len() {
		match args[i].as_str() {
			// template args
			"-d" => {
				println!(">> Parsing -d arg");
				
				if let Some(_template_name) = args.get(i + 1) {
					year2025::day1::run();
				} else {
					println!("Expected day in format YYYY-DD.");
				}
				i += 1;
			}

			"-y" => {
				println!(">> Parsing -y arg");
				if let Some(_template_name) = args.get(i + 1) {
					year2025::day1::run();
				} else {
					println!("Expected year in format YYYY.");
				}
				i += 1;
			}

			_ => i += 1,
		}
	}
}