use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

enum PathType {
	Normal,
	Portal(Coordinate),
}

fn main() {
	let mut start_point: Option<Coordinate> = None;
	let mut end_point: Option<Coordinate> = None;
	let mut path_tiles: HashMap<Coordinate, PathType> = HashMap::new();

	{
		let mut input = fs::read_to_string("input.txt").expect("Failed to read input file");
		if let Some(better_input) = input.strip_suffix('\n') {
			input = better_input.to_owned();
		}
		let mut labels: HashMap<String, Coordinate> = HashMap::new();
		let mut label_temp: HashMap<Coordinate, char> = HashMap::new();
		let mut x = 0;
		let mut y = 0;
		for input_char in input.chars() {
			if input_char == '.' {
				path_tiles.insert(Coordinate { x, y }, PathType::Normal);
			} else if input_char.is_ascii_uppercase() {
				label_temp.insert(Coordinate { x, y }, input_char);
			}
			if input_char == '\n' {
				y += 1;
				x = 0;
			} else {
				x += 1;
			}
		}
		while !label_temp.is_empty() {
			let label_part = label_temp.keys().into_iter().next().unwrap().clone();
			if label_part.x > 0 {
				let mut label_other_part = label_part.clone();
				label_other_part.x -= 1;
				if label_temp.contains_key(&label_other_part) {
					let mut label_name = String::with_capacity(2);
					label_name.push(label_temp[&label_other_part]);
					label_name.push(label_temp[&label_part]);
					let mut label_path_coordinate = label_part.clone();
					label_path_coordinate.x += 1;
					if !path_tiles.contains_key(&label_path_coordinate) {
						label_path_coordinate.x -= 3;
					}
					if label_name == "AA" {
						start_point = Some(label_path_coordinate);
						label_temp.remove(&label_part);
						label_temp.remove(&label_other_part);
						continue;
					}
					if label_name == "ZZ" {
						end_point = Some(label_path_coordinate);
						label_temp.remove(&label_part);
						label_temp.remove(&label_other_part);
						continue;
					}
					match labels.entry(label_name) {
						Entry::Occupied(label_entry) => {
							path_tiles.insert(
								label_path_coordinate.clone(),
								PathType::Portal(label_entry.get().clone()),
							);
							path_tiles.insert(label_entry.remove(), PathType::Portal(label_path_coordinate));
						}
						Entry::Vacant(label_entry) => {
							label_entry.insert(label_path_coordinate);
						}
					}
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
			}
			let mut label_other_part = label_part.clone();
			label_other_part.x += 1;
			if label_temp.contains_key(&label_other_part) {
				let mut label_name = String::with_capacity(2);
				label_name.push(label_temp[&label_part]);
				label_name.push(label_temp[&label_other_part]);
				let mut label_path_coordinate = label_other_part.clone();
				label_path_coordinate.x += 1;
				if !path_tiles.contains_key(&label_path_coordinate) {
					label_path_coordinate.x -= 3;
				}
				if label_name == "AA" {
					start_point = Some(label_path_coordinate);
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
				if label_name == "ZZ" {
					end_point = Some(label_path_coordinate);
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
				match labels.entry(label_name) {
					Entry::Occupied(label_entry) => {
						path_tiles.insert(
							label_path_coordinate.clone(),
							PathType::Portal(label_entry.get().clone()),
						);
						path_tiles.insert(label_entry.remove(), PathType::Portal(label_path_coordinate));
					}
					Entry::Vacant(label_entry) => {
						label_entry.insert(label_path_coordinate);
					}
				}
				label_temp.remove(&label_part);
				label_temp.remove(&label_other_part);
				continue;
			}
			label_other_part.x -= 1;
			if label_other_part.y > 0 {
				let mut label_other_part = label_other_part.clone();
				label_other_part.y -= 1;
				if label_temp.contains_key(&label_other_part) {
					let mut label_name = String::with_capacity(2);
					label_name.push(label_temp[&label_other_part]);
					label_name.push(label_temp[&label_part]);
					let mut label_path_coordinate = label_part.clone();
					label_path_coordinate.y += 1;
					if !path_tiles.contains_key(&label_path_coordinate) {
						label_path_coordinate.y -= 3;
					}
					if label_name == "AA" {
						start_point = Some(label_path_coordinate);
						label_temp.remove(&label_part);
						label_temp.remove(&label_other_part);
						continue;
					}
					if label_name == "ZZ" {
						end_point = Some(label_path_coordinate);
						label_temp.remove(&label_part);
						label_temp.remove(&label_other_part);
						continue;
					}
					match labels.entry(label_name) {
						Entry::Occupied(label_entry) => {
							path_tiles.insert(
								label_path_coordinate.clone(),
								PathType::Portal(label_entry.get().clone()),
							);
							path_tiles.insert(label_entry.remove(), PathType::Portal(label_path_coordinate));
						}
						Entry::Vacant(label_entry) => {
							label_entry.insert(label_path_coordinate);
						}
					}
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
			}
			label_other_part.y += 1;
			if label_temp.contains_key(&label_other_part) {
				let mut label_name = String::with_capacity(2);
				label_name.push(label_temp[&label_part]);
				label_name.push(label_temp[&label_other_part]);
				let mut label_path_coordinate = label_other_part.clone();
				label_path_coordinate.y += 1;
				if !path_tiles.contains_key(&label_path_coordinate) {
					label_path_coordinate.y -= 3;
				}
				if label_name == "AA" {
					start_point = Some(label_path_coordinate);
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
				if label_name == "ZZ" {
					end_point = Some(label_path_coordinate);
					label_temp.remove(&label_part);
					label_temp.remove(&label_other_part);
					continue;
				}
				match labels.entry(label_name) {
					Entry::Occupied(label_entry) => {
						path_tiles.insert(
							label_path_coordinate.clone(),
							PathType::Portal(label_entry.get().clone()),
						);
						path_tiles.insert(label_entry.remove(), PathType::Portal(label_path_coordinate));
					}
					Entry::Vacant(label_entry) => {
						label_entry.insert(label_path_coordinate);
					}
				}
				label_temp.remove(&label_part);
				label_temp.remove(&label_other_part);
				continue;
			}
			panic!("Orphaned half-label, found at ({}, {})", label_part.x, label_part.y);
		}
	}

	let start_point = start_point.expect("Failed to determine start point");
	let end_point = end_point.expect("Failed to determine end point");

	let mut steps_taken: usize = 0;
	let mut current_locations: Vec<Coordinate> = vec![start_point.clone()];
	let mut visited: HashSet<Coordinate> = HashSet::new();
	visited.insert(start_point);
	'search: loop {
		steps_taken += 1;
		let mut next_locations: Vec<Coordinate> = Vec::new();
		for location in current_locations {
			visited.insert(location.clone());
			if location.y > 0 {
				let mut north_location = location.clone();
				north_location.y -= 1;
				if north_location == end_point {
					break 'search;
				}
				if !visited.contains(&north_location) && path_tiles.contains_key(&north_location) {
					next_locations.push(north_location);
				}
			}
			if location.x > 0 {
				let mut west_location = location.clone();
				west_location.x -= 1;
				if west_location == end_point {
					break 'search;
				}
				if !visited.contains(&west_location) && path_tiles.contains_key(&west_location) {
					next_locations.push(west_location);
				}
			}
			let mut south_location = location.clone();
			south_location.y += 1;
			if south_location == end_point {
				break 'search;
			}
			if !visited.contains(&south_location) && path_tiles.contains_key(&south_location) {
				next_locations.push(south_location);
			}
			let mut east_location = location.clone();
			east_location.x += 1;
			if east_location == end_point {
				break 'search;
			}
			if !visited.contains(&east_location) && path_tiles.contains_key(&east_location) {
				next_locations.push(east_location);
			}
			if let PathType::Portal(teleport_location) = &path_tiles[&location] {
				if !visited.contains(teleport_location) {
					next_locations.push(teleport_location.clone());
				}
			}
		}
		current_locations = next_locations;
	}

	println!("{}", steps_taken);
}
