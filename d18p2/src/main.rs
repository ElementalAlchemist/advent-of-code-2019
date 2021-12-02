use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
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

#[derive(Clone)]
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

#[derive(Eq, Hash, PartialEq)]
struct TravelState {
	distance_traveled: usize,
	key_chains: Vec<Vec<char>>,
}

impl TravelState {
	fn new(number_of_sectors: usize) -> Self {
		let key_chains = vec![Vec::new(); number_of_sectors];
		Self {
			distance_traveled: 0,
			key_chains,
		}
	}

	fn complete(&self, sector_map: &[HashMap<char, HashMap<char, MapPathData>>]) -> bool {
		for (sector_index, key_chain) in self.key_chains.iter().enumerate() {
			if key_chain.len() != sector_map[sector_index][&'@'].len() {
				return false;
			}
		}
		true
	}

	fn add_travel(
		&self,
		sector_map: &[HashMap<char, HashMap<char, MapPathData>>],
		sector: usize,
		destination: char,
	) -> Option<Self> {
		if key_in_chain(&self.key_chains[sector], destination) {
			return None;
		}
		let current_location = self.key_chains[sector].last().unwrap_or(&'@');
		let path_data = sector_map[sector][current_location].get(&destination)?;
		let required_keys = &path_data.required_keys;
		for key in required_keys.iter() {
			if !key_obtained(&self.key_chains, *key) {
				return None;
			}
		}
		let distance_traveled = self.distance_traveled + path_data.distance;
		let mut key_chains = self.key_chains.clone();
		key_chains[sector].push(destination);
		Some(Self {
			distance_traveled,
			key_chains,
		})
	}
}

impl Ord for TravelState {
	// We need to reverse the ordering to make the heap work as a min-heap
	fn cmp(&self, other: &Self) -> Ordering {
		other
			.distance_traveled
			.cmp(&self.distance_traveled)
			.then_with(|| self.key_chains.cmp(&other.key_chains))
	}
}

impl PartialOrd for TravelState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn key_in_chain(key_chain: &[char], key: char) -> bool {
	for chain_key in key_chain.iter() {
		if *chain_key == key {
			return true;
		}
	}
	false
}

fn key_obtained(key_chains: &[Vec<char>], key: char) -> bool {
	for chain in key_chains.iter() {
		for chain_key in chain.iter() {
			if *chain_key == key {
				return true;
			}
		}
	}
	false
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

	let mut sector_maps: Vec<HashMap<char, HashMap<char, MapPathData>>> = Vec::new();
	let mut key_positions: HashMap<char, usize> = HashMap::new();
	for (location, tile_data) in map_data.iter().enumerate() {
		if let SectorType::Entrance = *tile_data {
			let path_map = map_from(&map_data, location, row_length);
			let mut sector_map: HashMap<char, HashMap<char, MapPathData>> = HashMap::new();
			sector_map.insert('@', path_map);
			sector_maps.push(sector_map);
		} else if let SectorType::Key(key_id) = tile_data {
			key_positions.insert(*key_id, location);
		}
	}
	for sector_map in sector_maps.iter_mut() {
		let mut add_maps: Vec<(char, HashMap<char, MapPathData>)> = Vec::new();
		for key in sector_map[&'@'].keys() {
			let key_map = map_from(&map_data, key_positions[key], row_length);
			add_maps.push((*key, key_map));
		}
		for (key, map) in add_maps {
			sector_map.insert(key, map);
		}
	}

	let mut priority_queue: BTreeMap<usize, HashSet<TravelState>> = BTreeMap::new();
	let first_travel_state = TravelState::new(sector_maps.len());
	let first_travel_distance = first_travel_state.distance_traveled;
	let mut first_travel_set = HashSet::new();
	first_travel_set.insert(first_travel_state);
	priority_queue.insert(first_travel_distance, first_travel_set);

	let lowest_distance = 'per_distance: loop {
		let current_distance = *priority_queue.keys().next().unwrap();
		let current_distance_set = priority_queue.remove(&current_distance).unwrap();
		for current_state in current_distance_set {
			if current_state.complete(&sector_maps) {
				break 'per_distance current_state.distance_traveled;
			}
			for (sector_index, sector_data) in sector_maps.iter().enumerate() {
				for destination in sector_data[&'@'].keys() {
					let next_state = current_state.add_travel(&sector_maps, sector_index, *destination);
					if let Some(state) = next_state {
						let next_entry = priority_queue.entry(state.distance_traveled).or_default();
						next_entry.insert(state);
					}
				}
			}
		}
	};

	println!("{}", lowest_distance);
}
