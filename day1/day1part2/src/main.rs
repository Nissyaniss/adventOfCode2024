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

	let mut count = 0;
	for left_column_current in &left_column {
		for right_column_current in &right_column {
			if left_column_current == right_column_current {
				count += 1;
			}
		}
		res += count * left_column_current;
		count = 0;
	}
	println!("{res}");
}
