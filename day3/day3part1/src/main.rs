use std::fs;

use regex::Regex;

fn main() {
	let data = match fs::read_to_string("src/input.txt") {
		Ok(data) => data,
		Err(e) => panic!("File not read correctly : {e}"),
	};
	let re = Regex::new(r"mul\(([0-9]{1,5}),([0-9]{1,5})\)").unwrap();

	let mut res = 0;
	for (_, [number1, number2]) in re.captures_iter(&data).map(|capture| capture.extract()) {
		let number1 = number1.parse::<i64>().unwrap();
		let number2 = number2.parse::<i64>().unwrap();
		res += number1 * number2;
	}
	println!("{res}",);
}
