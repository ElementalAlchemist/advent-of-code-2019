use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Eq, PartialEq)]
enum SectorType {
	Wall,
	Path,
	Key(char),
	Door(char),
	Entrance,
}

impl SectorType {
	fn new_from_char(sector_char: char) -> Self {
		if sector_char == '#' {
			Self::Wall
		} else if sector_char == '.' {
			Self::Path
		} else if sector_char == '@' {
			Self::Entrance
		} else if sector_char.is_ascii_uppercase() {
			Self::Door(sector_char)
		} else if sector_char.is_ascii_lowercase() {
			Self::Key(sector_char.to_ascii_uppercase())
		} else {
			panic!("Unexpected map character found: {}", sector_char);
		}
	}
}

struct MapPathData {
	distance: usize,
	required_keys: Vec<char>,
}

struct TemporaryPathData {
	location: usize,
	distance: usize,
	required_keys: Vec<char>,
}

enum TravelCheckResult {
	Clear,
	Blocked,
	PassesDoor(char),
	CollectsKey(char),
}

impl From<SectorType> for TravelCheckResult {
	fn from(sector_type: SectorType) -> Self {
		match sector_type {
			SectorType::Wall => Self::Blocked,
			SectorType::Path => Self::Clear,
			SectorType::Entrance => Self::Clear,
			SectorType::Door(door_id) => Self::PassesDoor(door_id),
			SectorType::Key(key_id) => Self::CollectsKey(key_id),
		}
	}
}

fn check_travel(map_data: &[SectorType], destination: usize, visited: &HashSet<usize>) -> TravelCheckResult {
	if visited.contains(&destination) {
		return TravelCheckResult::Blocked;
	}
	map_data[destination].clone().into()
}

fn add_travel(
	map_data: &[SectorType],
	destination: usize,
	distance: usize,
	required_keys: &[char],
	visited: &HashSet<usize>,
	check_points: &mut VecDeque<TemporaryPathData>,
	distance_map: &mut HashMap<char, MapPathData>,
) {
	match check_travel(map_data, destination, visited) {
		TravelCheckResult::Blocked => (),
		TravelCheckResult::Clear => {
			let required_keys = required_keys.to_vec();
			check_points.push_back(TemporaryPathData {
				location: destination,
				distance,
				required_keys,
			});
		}
		TravelCheckResult::PassesDoor(door_id) => {
			let mut required_keys = required_keys.to_vec();
			required_keys.push(door_id);
			check_points.push_back(TemporaryPathData {
				location: destination,
				distance,
				required_keys,
			});
		}
		TravelCheckResult::CollectsKey(key_id) => {
			let required_keys = required_keys.to_vec();
			check_points.push_back(TemporaryPathData {
				location: destination,
				distance,
				required_keys: required_keys.clone(),
			});
			distance_map.insert(
				key_id,
				MapPathData {
					distance,
					required_keys,
				},
			);
		}
	}
}

fn map_from(map_data: &[SectorType], start_location: usize, row_length: usize) -> HashMap<char, MapPathData> {
	let mut visited: HashSet<usize> = HashSet::new();
	visited.insert(start_location);

	let mut distance_map: HashMap<char, MapPathData> = HashMap::new();

	let mut check_points: VecDeque<TemporaryPathData> = VecDeque::new();
	check_points.push_back(TemporaryPathData {
		location: start_location,
		distance: 0,
		required_keys: Vec::new(),
	});

	while let Some(point) = check_points.pop_front() {
		let current_location = point.location;
		visited.insert(current_location);
		if current_location > row_length {
			let next_location_north = current_location - row_length;
			add_travel(
				map_data,
				next_location_north,
				point.distance + 1,
				&point.required_keys,
				&visited,
				&mut check_points,
				&mut distance_map,
			);
		}
		if current_location % row_length > 0 {
			let next_location_west = current_location - 1;
			add_travel(
				map_data,
				next_location_west,
				point.distance + 1,
				&point.required_keys,
				&visited,
				&mut check_points,
				&mut distance_map,
			);
		}
		let next_location_south = current_location + row_length;
		if next_location_south < map_data.len() {
			add_travel(
				map_data,
				next_location_south,
				point.distance + 1,
				&point.required_keys,
				&visited,
				&mut check_points,
				&mut distance_map,
			);
		}
		let next_location_east = current_location + 1;
		if current_location % row_length != 0 {
			add_travel(
				map_data,
				next_location_east,
				point.distance + 1,
				&point.required_keys,
				&visited,
				&mut check_points,
				&mut distance_map,
			);
		}
	}

	distance_map
}

fn key_in_chain(key_chain: &[char], key: char) -> bool {
	for passed_key in key_chain.iter() {
		if key == *passed_key {
			return true;
		}
	}
	false
}

fn get_shortest_key_chain(
	path_map: &HashMap<char, HashMap<char, MapPathData>>,
	all_keys: &[char],
	key_chain: &[char],
	current_distance: usize,
	current_shortest_distance: usize,
	shortest_path_distance: usize,
) -> usize {
	let start_key = if let Some(key) = key_chain.last() { *key } else { '@' };
	let path_from_start = &path_map[&start_key];
	let mut shortest_distance = current_shortest_distance;
	let total_key_count = all_keys.len();
	let key_chain_len = key_chain.len();
	let remaining_jumps = total_key_count - key_chain_len - 1;
	let minimum_remaining_distance = remaining_jumps * shortest_path_distance;
	'keys: for (key, path_data) in path_from_start.iter() {
		let distance = current_distance + path_data.distance;
		if distance >= shortest_distance {
			continue;
		}
		if distance + minimum_remaining_distance >= shortest_distance {
			continue;
		}
		if key_in_chain(key_chain, *key) {
			continue;
		}
		for check_key in path_data.required_keys.iter() {
			if !key_in_chain(key_chain, *check_key) {
				continue 'keys;
			}
		}
		let mut new_chain: Vec<char> = key_chain.to_owned();
		new_chain.push(*key);
		if new_chain.len() == all_keys.len() {
			// There can be no other keys that aren't in the chain, as this chain is complete.
			println!("Complete chain ({}) {:?}", distance, new_chain);
			return distance;
		}
		shortest_distance = get_shortest_key_chain(
			path_map,
			all_keys,
			&new_chain,
			distance,
			shortest_distance,
			shortest_path_distance,
		);
	}
	shortest_distance
}

fn main() {
	let mut row_length: Option<usize> = None;
	let map_data: Vec<SectorType> = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		let mut map_sector_chars: Vec<char> = Vec::new();
		for (index, sector_char) in input.chars().enumerate() {
			if sector_char == '\n' {
				if row_length.is_none() {
					row_length = Some(index);
				}
				continue;
			}
			map_sector_chars.push(sector_char);
		}
		map_sector_chars.iter().map(|x| SectorType::new_from_char(*x)).collect()
	};
	let row_length = row_length.expect("Improperly formatted input");

	let mut maps_from: HashMap<char, HashMap<char, MapPathData>> = HashMap::new();
	for (location, tile_data) in map_data.iter().enumerate() {
		let path_map = match tile_data {
			SectorType::Entrance | SectorType::Key(_) => map_from(&map_data, location, row_length),
			_ => continue,
		};
		let key = match tile_data {
			SectorType::Entrance => '@',
			SectorType::Key(key_id) => *key_id,
			_ => unreachable!(),
		};
		maps_from.insert(key, path_map);
	}
	let shortest_path_distance = maps_from
		.values()
		.map(|x| x.values().map(|x| x.distance).min().expect("No path data"))
		.min()
		.expect("No path data");
	let keys: Vec<char> = maps_from.keys().filter(|k| **k != '@').copied().collect();

	let mut shortest_hops_distance: usize = 0;
	let mut shortest_hop_key_chain: Vec<char> = Vec::new();
	while shortest_hop_key_chain.len() < keys.len() {
		let mut shortest_hop: Option<(&char, &MapPathData)> = None;
		let current_node = if let Some(node) = shortest_hop_key_chain.last() {
			node
		} else {
			&'@'
		};
		'keys: for (node, data) in maps_from[current_node].iter() {
			if key_in_chain(&shortest_hop_key_chain, *node) {
				continue;
			}
			for key in data.required_keys.iter() {
				if !key_in_chain(&shortest_hop_key_chain, *key) {
					continue 'keys;
				}
			}
			if let Some((short_node, hop)) = shortest_hop.take() {
				if data.distance < hop.distance {
					shortest_hop = Some((node, data));
				} else {
					shortest_hop = Some((short_node, hop));
				}
			} else {
				shortest_hop = Some((node, data));
			}
		}
		let (short_hop_node, short_hop_data) = shortest_hop.unwrap();
		shortest_hop_key_chain.push(*short_hop_node);
		shortest_hops_distance += short_hop_data.distance;
	}

	let lowest_distance = get_shortest_key_chain(
		&maps_from,
		&keys,
		&Vec::new(),
		0,
		shortest_hops_distance,
		shortest_path_distance,
	);
	println!("{}", lowest_distance);
}
