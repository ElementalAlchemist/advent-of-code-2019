use std::fs;

fn main() {
	let module_weights: Vec<u64> = {
		let input = fs::read_to_string("input.txt").unwrap();
		input
			.split('\n')
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect()
	};

	let fuel: u64 = module_weights.iter().map(|x| (x / 3) - 2).sum();
	println!("{}", fuel);
}
