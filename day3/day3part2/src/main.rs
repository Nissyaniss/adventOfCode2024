use std::fs;

use regex::Regex;

fn main() {
	let data = match fs::read_to_string("src/input.txt") {
		Ok(data) => data,
		Err(e) => panic!("File not read correctly : {e}"),
	};
	let re = Regex::new(r"(?<do_or_dont>don't\(\)|do\(\))|mul\((?<number1>\d+),(?<number2>\d+)\)")
		.unwrap();

	let mut res = 0;
	let mut r#do = true;
	for (number1, number2, do_or_dont) in re.captures_iter(&data).map(
		|capture| -> (
			Option<regex::Match<'_>>,
			Option<regex::Match<'_>>,
			Option<regex::Match<'_>>,
		) {
			(
				capture.name("number1"),
				capture.name("number2"),
				capture.name("do_or_dont"),
			)
		},
	) {
		let number1 = number1.map_or(-1, |r#match| r#match.as_str().parse::<i64>().unwrap());
		let number2 = number2.map_or(-1, |r#match| r#match.as_str().parse::<i64>().unwrap());
		r#do = do_or_dont.map_or_else(
			|| r#do,
			|r#match| {
				if r#match.as_str() == "do()" {
					true
				} else if r#match.as_str() == "don't()" {
					false
				} else {
					r#do
				}
			},
		);

		if r#do && (number1 != -1 || number2 != -1) {
			res += number1 * number2;
		}
	}
	println!("{res}",);
}
