use std::fs;

fn main() {
	let data = match fs::read_to_string("src/input.txt") {
		Ok(data) => data,
		Err(e) => panic!("File not read correctly : {e}"),
	};

	let lines = data.split('\n');

	let mut safe: u32 = 0;
	for line in lines {
		let mut numbers = Vec::new();
		line.split(' ').for_each(|number| {
			numbers.push(number.parse::<i64>().unwrap());
		});
		let mut numbers_iter = numbers.iter();

		let Some(mut number_current) = numbers_iter.next() else {
			continue;
		};
		let Some(mut number_next) = numbers_iter.next() else {
			continue;
		};
		let is_decreasing = (number_current - number_next).is_positive();
		let mut is_safe = check_numbers_safe(is_decreasing, *number_current, *number_next);

		for _ in 0..numbers.len() {
			is_safe = check_numbers_safe(is_decreasing, *number_current, *number_next);
			if !is_safe {
				break;
			}
			number_current = number_next;
			number_next = match numbers_iter.next() {
				Some(number) => number,
				None => break,
			}
		}
		if is_safe {
			safe += 1;
		}
	}
	println!("{safe}");
}

fn check_numbers_safe(is_decreasing: bool, number1: i64, number2: i64) -> bool {
	let result = number1 - number2;
	if !(1..=3).contains(&result.abs()) {
		return false;
	}
	if is_decreasing != result.is_positive() {
		return false;
	}
	true
}
