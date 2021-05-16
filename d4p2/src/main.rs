const RANGE_LOW: u32 = 246540;
const RANGE_HIGH: u32 = 787419;

fn main() {
	let mut password_count: usize = 0;
	'pass: for password in RANGE_LOW..=RANGE_HIGH {
		let password = password.to_string();
		let mut pass_chars = password.chars();
		let mut last_digit: u8 = pass_chars.next().unwrap().to_string().parse().unwrap();
		let mut had_double = false;
		let mut run_length: u8 = 1;
		for pass_char in pass_chars {
			let next_digit: u8 = pass_char.to_string().parse().unwrap();
			if next_digit == last_digit {
				run_length += 1;
			} else {
				if run_length == 2 {
					had_double = true;
				}
				run_length = 1;

				if next_digit < last_digit {
					continue 'pass;
				}
			}
			last_digit = next_digit;
		}
		if run_length == 2 {
			had_double = true;
		}
		if !had_double {
			continue;
		}
		password_count += 1;
	}
	println!("{}", password_count);
}
