use std::fs;
use std::mem;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Orientation {
	Horizontal,
	Vertical,
}

struct Movement {
	orientation: Orientation,
	distance: i32,
}

impl Movement {
	fn get_next_coordinate(&self, start_coord: &Coordinates) -> Coordinates {
		let mut next_coord = start_coord.clone();
		match self.orientation {
			Orientation::Horizontal => next_coord.x += self.distance,
			Orientation::Vertical => next_coord.y += self.distance,
		}
		next_coord
	}
}

impl FromStr for Movement {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut input_chars = s.chars();
		let direction = input_chars.next().unwrap();
		let mut distance: i32 = input_chars.as_str().parse().unwrap();
		let orientation = match direction {
			'R' => Orientation::Horizontal,
			'U' => Orientation::Vertical,
			'L' => {
				distance = -distance;
				Orientation::Horizontal
			}
			'D' => {
				distance = -distance;
				Orientation::Vertical
			}
			_ => panic!("Invalid direction"),
		};
		Ok(Self { orientation, distance })
	}
}

#[derive(Clone, Default)]
struct Coordinates {
	x: i32,
	y: i32,
}

impl Coordinates {
	fn distance_from_port(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}
}

struct WireLine {
	endpoint_low: i32,
	endpoint_high: i32,
	line_position: i32,
	orientation: Orientation,
}

impl WireLine {
	fn get_intersection(&self, other: &WireLine) -> Option<Coordinates> {
		if self.orientation == other.orientation {
			None
		} else if self.endpoint_low <= other.line_position
			&& self.endpoint_high >= other.line_position
			&& other.endpoint_low <= self.line_position
			&& other.endpoint_high >= self.line_position
		{
			match self.orientation {
				Orientation::Horizontal => Some(Coordinates {
					x: other.line_position,
					y: self.line_position,
				}),
				Orientation::Vertical => Some(Coordinates {
					x: self.line_position,
					y: other.line_position,
				}),
			}
		} else {
			None
		}
	}
}

impl From<(Coordinates, Movement)> for WireLine {
	fn from((start_coord, movement): (Coordinates, Movement)) -> Self {
		let (endpoint_start, line_position) = match movement.orientation {
			Orientation::Horizontal => (start_coord.x, start_coord.y),
			Orientation::Vertical => (start_coord.y, start_coord.x),
		};
		let endpoint_end = endpoint_start + movement.distance;
		let (endpoint_low, endpoint_high) = if endpoint_start < endpoint_end {
			(endpoint_start, endpoint_end)
		} else {
			(endpoint_end, endpoint_start)
		};
		WireLine {
			endpoint_low,
			endpoint_high,
			line_position,
			orientation: movement.orientation,
		}
	}
}

fn main() {
	let (wire_1, wire_2) = {
		let input = fs::read_to_string("input.txt").unwrap();
		let mut line_iter = input.split('\n');
		let wire_1 = line_iter.next().unwrap();
		let wire_2 = line_iter.next().unwrap();
		assert!(line_iter.next().is_none());

		let wire_1: Vec<String> = wire_1.split(',').map(|s| s.to_owned()).collect();
		let wire_2: Vec<String> = wire_2.split(',').map(|s| s.to_owned()).collect();
		(wire_1, wire_2)
	};

	let mut current_coordinates = Coordinates::default();
	let mut wire_1_lines: Vec<WireLine> = Vec::with_capacity(wire_1.len());
	for movement in wire_1.iter() {
		let movement: Movement = movement.parse().unwrap();
		let next_coordinates = movement.get_next_coordinate(&current_coordinates);
		let line: WireLine = (mem::replace(&mut current_coordinates, next_coordinates), movement).into();
		wire_1_lines.push(line);
	}

	let mut current_coordinates = Coordinates::default();
	let mut intersections: Vec<Coordinates> = Vec::new();
	for movement in wire_2.iter() {
		let movement: Movement = movement.parse().unwrap();
		let next_coordinates = movement.get_next_coordinate(&current_coordinates);
		let line: WireLine = (mem::replace(&mut current_coordinates, next_coordinates), movement).into();

		for other_line in wire_1_lines.iter() {
			if let Some(intersection) = line.get_intersection(other_line) {
				intersections.push(intersection);
			}
		}
	}

	let shortest_distance = intersections
		.iter()
		.map(|coord| coord.distance_from_port())
		.min()
		.unwrap();
	println!("{}", shortest_distance);
}
