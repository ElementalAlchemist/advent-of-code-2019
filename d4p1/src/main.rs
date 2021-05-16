const RANGE_LOW: u32 = 246540;
const RANGE_HIGH: u32 = 787419;

fn main() {
	let mut password_count: usize = 0;
	'pass: for password in RANGE_LOW..=RANGE_HIGH {
		let password = password.to_string();
		let mut pass_chars = password.chars();
		let mut last_digit: u8 = pass_chars.next().unwrap().to_string().parse().unwrap();
		let mut had_double = false;
		for pass_char in pass_chars {
			let next_digit: u8 = pass_char.to_string().parse().unwrap();
			if next_digit == last_digit {
				had_double = true;
			} else if next_digit < last_digit {
				continue 'pass;
			}
			last_digit = next_digit;
		}
		if !had_double {
			continue;
		}
		password_count += 1;
	}
	println!("{}", password_count);
}
