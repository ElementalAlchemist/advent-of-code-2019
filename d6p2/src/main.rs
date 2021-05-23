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

	let mut you_orbit_lineage: Vec<&str> = Vec::new();
	let mut current_orbit = "YOU";
	while let Some(orbit) = orbit_parents.get(current_orbit) {
		you_orbit_lineage.push(orbit);
		current_orbit = orbit;
	}
	let mut santa_orbit_lineage: Vec<&str> = Vec::new();
	let mut current_orbit = "SAN";
	while let Some(orbit) = orbit_parents.get(current_orbit) {
		santa_orbit_lineage.push(orbit);
		current_orbit = orbit;
	}

	let mut orbit_count: usize = 0;
	let mut orbit_shared_root: &str = "";
	for (index, object) in you_orbit_lineage.iter().enumerate() {
		if santa_orbit_lineage.contains(object) {
			orbit_count = index;
			orbit_shared_root = object;
			break;
		}
	}
	for (index, object) in santa_orbit_lineage.iter().enumerate() {
		if *object == orbit_shared_root {
			orbit_count += index;
			break;
		}
	}

	println!("{}", orbit_count);
}
