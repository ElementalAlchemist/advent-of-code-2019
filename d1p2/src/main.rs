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
	let fuel_weight: u64 = module_weights
		.iter()
		.map(|weight| {
			let mut total_fuel_weight: u64 = 0;
			let mut new_fuel_weight: u64 = (weight / 3) - 2;

			while new_fuel_weight > 0 {
				total_fuel_weight += new_fuel_weight;
				new_fuel_weight /= 3;
				new_fuel_weight = if new_fuel_weight >= 2 { new_fuel_weight - 2 } else { 0 };
			}
			total_fuel_weight
		})
		.sum();
	println!("{}", fuel_weight);
}
