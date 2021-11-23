use std::fs;

fn main() {
	let signal_template: Vec<i32> = fs::read_to_string("input.txt")
		.unwrap()
		.chars()
		.map(|x| x.to_digit(10))
		.filter(|x| x.is_some())
		.map(|x| x.unwrap() as i32)
		.collect();

	let mut signal: Vec<i32> = Vec::with_capacity(signal_template.len() * 10_000);
	for _ in 0..10_000 {
		for value in signal_template.iter() {
			signal.push(*value);
		}
	}

	let offset = (signal[0] * 1_000_000
		+ signal[1] * 100_000
		+ signal[2] * 10_000
		+ signal[3] * 1_000
		+ signal[4] * 100
		+ signal[5] * 10
		+ signal[6]) as usize;

	for _ in 0..100 {
		let start_signal = signal.clone();
		signal = vec![0; offset];
		let mut new_value = 0;
		for start_value in start_signal[offset..].iter() {
			new_value += *start_value;
		}
		signal.push(new_value % 10);

		for start_value in start_signal[offset..start_signal.len() - 1].iter() {
			new_value -= *start_value;
			signal.push(new_value % 10);
		}
	}

	let mut result_iter = signal.iter().skip(offset);
	let mut result: Vec<i32> = Vec::with_capacity(8);
	for _ in 0..8 {
		result.push(*result_iter.next().unwrap());
	}
	println!("{:?}", result);
}
