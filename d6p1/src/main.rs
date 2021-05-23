use std::collections::HashMap;
use std::fs;

fn main() {
	let orbit_parents: HashMap<String, String> = {
		let input = fs::read_to_string("input.txt").unwrap();
		let orbit_definitions: Vec<(String, String)> = input
			.lines()
			.filter(|s| !s.is_empty())
			.map(|s| {
				let mut parts = s.split(')');
				(parts.next().unwrap().to_owned(), parts.next().unwrap().to_owned())
			})
			.collect();

		let mut orbit_parent_map: HashMap<String, String> = HashMap::new();
		for (object, orbiter) in orbit_definitions {
			orbit_parent_map.insert(orbiter, object);
		}
		orbit_parent_map
	};
	let mut orbit_count: u32 = 0;
	for orbit_object in orbit_parents.keys() {
		let mut current_object = orbit_object;
		while let Some(object) = orbit_parents.get(current_object) {
			orbit_count += 1;
			current_object = object;
		}
	}
	println!("{}", orbit_count);
}
