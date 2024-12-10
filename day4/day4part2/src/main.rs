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
		if char == 'A'
			&& position.checked_sub(line_len).is_some()
			&& position.checked_add(line_len).is_some()
			&& position.checked_add(line_len + 2).is_some()
			&& position.checked_sub(line_len + 2).is_some()
			&& position - line_len > 0
			&& position - line_len - 2 > 0
			&& position + line_len < data.len()
			&& position + line_len + 2 < data.len()
			&& ((data.as_bytes()[position - line_len] == b'M'
				&& data.as_bytes()[position + line_len] == b'S')
				|| (data.as_bytes()[position - line_len] == b'S'
					&& data.as_bytes()[position + line_len] == b'M'))
			&& ((data.as_bytes()[position - line_len - 2] == b'M'
				&& data.as_bytes()[position + line_len + 2] == b'S')
				|| (data.as_bytes()[position - line_len - 2] == b'S'
					&& data.as_bytes()[position + line_len + 2] == b'M'))
		{
			xmas_counter += 1;
		}
	}
	println!("{xmas_counter}");
}
