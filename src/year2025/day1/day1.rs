use csv::Reader;

const TEST_DATA: &str = "src/year2025/day1/testdata";
const INPUT_DATA: &str = "src/year2025/day1/inputdata";

pub fn run() {
	println!(">>>	Running 2025 Day 1...");
	println!(">>	Initializing Dial with value 50...");
	let mut dial = Dial::new(50, 99);

	let mut rdr = Reader::from_path(INPUT_DATA).unwrap(); 
	let mut counter = 0;
	
	for result in rdr.records() {
		let record = result.unwrap()[0].to_string();

		let direction = record.chars().next().unwrap();
		let clicks = record[1..].parse().unwrap();
		
		dial.turn(
			direction,
			clicks,
		);

		if dial.pointer == 0 {
			counter += 1;
		}
	}

	println!(">>	Finished turning with counter of: {}", counter);
	println!(">>	Finished turning with zero counter of: {}", dial.zero_counter);
	println!(">>>	Finished 2025 Day 1...");

}

pub struct Dial {
	pointer: i32,
	max: i32,
	zero_counter: i32
}

impl Dial {
	pub fn new(pointer: i32, max: i32) -> Self {
		Self {
			pointer,
			max,
			zero_counter: 0,
		}
	}

	pub fn turn(&mut self, direction: char, clicks: i32) {
		for _ in 0..clicks {
			match direction {
				'L' => self.pointer += 1,
				'R' => self.pointer -= 1,
				_ => panic!("Invalid direction!"),
			}
			
			if self.pointer == 0 { self.zero_counter += 1; }

			if self.pointer > self.max {
				self.pointer = 0;
				self.zero_counter += 1;
			} else if self.pointer < 0 {
				self.pointer = self.max;
			}
		}

	}
}
