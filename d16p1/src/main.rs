use std::fs;

fn generate_pattern(index: usize) -> Vec<i32> {
	let base_pattern = [0, 1, 0, -1];
	let mut pattern: Vec<i32> = Vec::with_capacity((index + 1) * 4);
	for pattern_value in base_pattern {
		for _ in 0..=index {
			pattern.push(pattern_value);
		}
	}
	pattern
}

fn main() {
	let mut signal: Vec<i32> = fs::read_to_string("input.txt")
		.unwrap()
		.chars()
		.map(|x| x.to_digit(10))
		.filter(|x| x.is_some())
		.map(|x| x.unwrap() as i32)
		.collect();

	for _ in 0..100 {
		let start_signal = signal.clone();
		for (index, value) in signal.iter_mut().enumerate() {
			let mut new_value = 0;
			let pattern = generate_pattern(index);
			let mut pattern_iter = pattern.iter().cycle();
			pattern_iter.next(); // We need to skip the first value of the whole cycle
			for start_value in start_signal.iter() {
				new_value += *start_value * *pattern_iter.next().unwrap();
			}
			let new_value = new_value % 10;
			*value = new_value.abs();
		}
		println!("{:?}", signal);
	}
}
