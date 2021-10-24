use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

struct FormulaData {
	amount_made: u64,
	inputs: HashMap<String, u64>,
}

struct FullFormula {
	material: String,
	data: FormulaData,
}

impl FromStr for FullFormula {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(" => ");
		let inputs = parts
			.next()
			.unwrap()
			.split(", ")
			.map(|s| {
				let mut parts = s.split(' ');
				let quantity: u64 = parts.next().unwrap().parse().unwrap();
				let material = parts.next().unwrap().to_owned();
				(material, quantity)
			})
			.collect();
		let mut output_parts = parts.next().unwrap().split(' ');
		let amount_made = output_parts.next().unwrap().parse().unwrap();
		let material = output_parts.next().unwrap().to_owned();
		let data = FormulaData { amount_made, inputs };
		Ok(Self { material, data })
	}
}

fn ore_use_for_fuel(formulas: &HashMap<String, FormulaData>, used_for_products: &HashMap<&str, Vec<&str>>, fuel_total: u64) -> u64 {
	let mut number_required: HashMap<&str, u64> = HashMap::new();
	number_required.insert("FUEL", fuel_total);
	let mut extra_produced: HashMap<&str, u64> = HashMap::new();

	while number_required.len() != formulas.len() + 1 {
		'ingredient_loop: for (ingredient, products) in used_for_products.iter() {
			if number_required.contains_key(ingredient) {
				continue;
			}
			let mut number_required_for_ingredient: u64 = 0;
			for product in products.iter() {
				if let Some(product_required) = number_required.get(product) {
					let mut multiplier = product_required / formulas[*product].amount_made;
					let mut extra = product_required % formulas[*product].amount_made;
					if extra > 0 {
						multiplier += 1;
						extra = formulas[*product].amount_made - extra;
						*extra_produced.entry(product).or_default() += extra;
					}
					let number_used_per_reaction = formulas[*product].inputs[*ingredient];
					number_required_for_ingredient += number_used_per_reaction * multiplier;
				} else {
					continue 'ingredient_loop;
				}
			}
			number_required.insert(ingredient, number_required_for_ingredient);
		}
	}

	number_required["ORE"]
}

fn main() {
	let formulas: HashMap<String, FormulaData> = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		input
			.lines()
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.map(|data: FullFormula| (data.material, data.data))
			.collect()
	};

	let mut used_for_products: HashMap<&str, Vec<&str>> = HashMap::new();
	for (product, formula_data) in formulas.iter() {
		for (input_material, _) in formula_data.inputs.iter() {
			let use_data = used_for_products.entry(input_material.as_str()).or_default();
			if !use_data.contains(&product.as_str()) {
				use_data.push(product);
			}
		}
	}

	let ore_use_for_one = ore_use_for_fuel(&formulas, &used_for_products, 1);
	let mut fuel_total = 1_000_000_000_000 / ore_use_for_one;
	loop {
		let ore_use = ore_use_for_fuel(&formulas, &used_for_products, fuel_total);

		if ore_use > 1_000_000_000_000 {
			fuel_total -= 1;
			println!("{}", fuel_total);
			break;
		}
		fuel_total += 1;
	}
}
