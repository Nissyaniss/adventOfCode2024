use std::fs;

fn main() {
	let data = match fs::read_to_string("src/input.txt") {
		Ok(data) => data,
		Err(e) => panic!("File not read correctly : {e}"),
	};
	let mut right_column = Vec::new();
	let mut left_column = Vec::new();
	let mut count: u32 = 0;
	let mut res = 0;

	for string in data.replace(['\r', '\n'], " ").split(' ') {
		if !string.is_empty() {
			if count % 2 != 0 {
				right_column.push(string.parse::<u32>().unwrap());
			} else {
				left_column.push(string.parse::<u32>().unwrap());
			}
			count += 1;
		}
	}
	left_column.sort_unstable();
	right_column.sort_unstable();

	for (left_column_current, right_column_current) in left_column.iter().zip(right_column.iter()) {
		if left_column_current > right_column_current {
			res += left_column_current - right_column_current;
		} else {
			res += right_column_current - left_column_current;
		}
	}
	println!("{res}");
}
