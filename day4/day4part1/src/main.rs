use std::fs;

fn main() {
	let data = match fs::read_to_string("src/input.txt") {
		Ok(data) => data,
		Err(e) => panic!("File not read correctly : {e}"),
	};
	let mut line_len = 0;
	for char in data.chars() {
		if char == '\n' {
			break;
		}
		line_len += 1;
	}
	let mut xmas_counter = 0;
	for (position, char) in data.chars().enumerate() {
		if char == 'X' {
			// XMAS
			if position + 3 < data.len()
				&& data.as_bytes()[position + 1] == b'M'
				&& data.as_bytes()[position + 2] == b'A'
				&& data.as_bytes()[position + 3] == b'S'
			{
				xmas_counter += 1;
			}
			//    S
			//   A
			//  M
			// X
			if position.checked_sub(line_len * 3).is_some()
				&& data.as_bytes()[position - line_len] == b'M'
				&& data.as_bytes()[position - (line_len * 2)] == b'A'
				&& data.as_bytes()[position - (line_len * 3)] == b'S'
			{
				xmas_counter += 1;
			}
			// S
			//  A
			//   M
			//    X
			if position.checked_sub(line_len * 3).is_some()
				&& data.as_bytes()[position - line_len - 2] == b'M'
				&& data.as_bytes()[position - (line_len * 2) - 4] == b'A'
				&& data.as_bytes()[position - (line_len * 3) - 6] == b'S'
			{
				xmas_counter += 1;
			}
			// X
			//  M
			//   A
			//    S
			if position + (line_len * 3) + 4 < data.len()
				&& data.as_bytes()[position + line_len + 2] == b'M'
				&& data.as_bytes()[position + (line_len * 2) + 4] == b'A'
				&& data.as_bytes()[position + (line_len * 3) + 6] == b'S'
			{
				xmas_counter += 1;
			}
			//    X
			//   M
			//  A
			// S
			if position + (line_len * 3) - 4 < data.len()
				&& data.as_bytes()[position + line_len] == b'M'
				&& data.as_bytes()[position + (line_len * 2)] == b'A'
				&& data.as_bytes()[position + (line_len * 3)] == b'S'
			{
				xmas_counter += 1;
			}
			// X
			// M
			// A
			// S
			if position + (line_len * 3) + 3 < data.len()
				&& data.as_bytes()[position + line_len + 1] == b'M'
				&& data.as_bytes()[position + (line_len * 2) + 2] == b'A'
				&& data.as_bytes()[position + (line_len * 3) + 3] == b'S'
			{
				xmas_counter += 1;
			}
			// S
			// A
			// M
			// X
			if position.checked_sub((line_len * 3) - 3).is_some()
				&& data.as_bytes()[position - line_len - 1] == b'M'
				&& data.as_bytes()[position - (line_len * 2) - 2] == b'A'
				&& data.as_bytes()[position - (line_len * 3) - 3] == b'S'
			{
				xmas_counter += 1;
			}
			//SMAX
			if position.checked_sub(3).is_some()
				&& data.as_bytes()[position - 1] == b'M'
				&& data.as_bytes()[position - 2] == b'A'
				&& data.as_bytes()[position - 3] == b'S'
			{
				xmas_counter += 1;
			}
		}
	}
	println!("{xmas_counter}");
}
