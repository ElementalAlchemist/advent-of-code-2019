use num::integer::lcm;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Coordinates {
	x: i32,
	y: i32,
	z: i32,
}

impl Coordinates {
	fn new(x: i32, y: i32, z: i32) -> Self {
		Self { x, y, z }
	}

	fn coord_sum(&self) -> i32 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}

	fn move_by(&mut self, velocity: &Coordinates) {
		self.x += velocity.x;
		self.y += velocity.y;
		self.z += velocity.z;
	}
}

impl Add for Coordinates {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

#[derive(Default)]
struct CoordinatesCount {
	x: u64,
	y: u64,
	z: u64,
}

impl CoordinatesCount {
	fn all_have_values(&self) -> bool {
		self.x > 0 && self.y > 0 && self.z > 0
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Moon {
	position: Coordinates,
	velocity: Coordinates,
}

impl Moon {
	fn new(start_x: i32, start_y: i32, start_z: i32) -> Self {
		let position = Coordinates::new(start_x, start_y, start_z);
		let velocity = Coordinates::default();
		Self { position, velocity }
	}

	fn total_energy(&self) -> i32 {
		self.position.coord_sum() * self.velocity.coord_sum()
	}

	fn move_moon(&mut self) {
		self.position.move_by(&self.velocity);
	}

	fn x_state(&self) -> (i32, i32) {
		(self.position.x, self.velocity.x)
	}

	fn y_state(&self) -> (i32, i32) {
		(self.position.y, self.velocity.y)
	}

	fn z_state(&self) -> (i32, i32) {
		(self.position.z, self.velocity.z)
	}
}

fn main() {
	let mut moons = [
		// Moon coordinates from puzzle input
		Moon::new(5, 4, 4),
		Moon::new(-11, -11, -3),
		Moon::new(0, 7, 0),
		Moon::new(-13, 2, 10),
	];

	let mut existing_x: HashMap<[(i32, i32); 4], u64> = HashMap::new();
	let mut existing_y: HashMap<[(i32, i32); 4], u64> = HashMap::new();
	let mut existing_z: HashMap<[(i32, i32); 4], u64> = HashMap::new();
	let mut cycle_total = CoordinatesCount::default();
	let mut cycle_start = CoordinatesCount::default();
	let mut iterations: u64 = 0;
	loop {
		let mut curr_x_state = [(0, 0); 4];
		let mut curr_y_state = [(0, 0); 4];
		let mut curr_z_state = [(0, 0); 4];
		for (moon_index, moon) in moons.iter().enumerate() {
			curr_x_state[moon_index] = moon.x_state();
			curr_y_state[moon_index] = moon.y_state();
			curr_z_state[moon_index] = moon.z_state();
		}
		if cycle_total.x == 0 {
			if let Some(orig_iterations) = existing_x.get(&curr_x_state) {
				cycle_total.x = iterations - orig_iterations;
				cycle_start.x = *orig_iterations;
			} else {
				existing_x.insert(curr_x_state, iterations);
			}
		}

		if cycle_total.y == 0 {
			if let Some(orig_iterations) = existing_y.get(&curr_y_state) {
				cycle_total.y = iterations - orig_iterations;
				cycle_start.y = *orig_iterations;
			} else {
				existing_y.insert(curr_y_state, iterations);
			}
		}

		if cycle_total.z == 0 {
			if let Some(orig_iterations) = existing_z.get(&curr_z_state) {
				cycle_total.z = iterations - orig_iterations;
				cycle_start.z = *orig_iterations;
			} else {
				existing_z.insert(curr_z_state, iterations);
			}
		}

		if cycle_total.all_have_values() {
			break;
		}

		let mut adjustments = [Coordinates::default(); 4];
		for moon_index in 0..moons.len() {
			let moon = &moons[moon_index];
			for (other_moon_index, other_moon) in moons.iter().enumerate() {
				if moon_index == other_moon_index {
					continue;
				}

				adjustments[moon_index].x += match moon.position.x.cmp(&other_moon.position.x) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};

				adjustments[moon_index].y += match moon.position.y.cmp(&other_moon.position.y) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};

				adjustments[moon_index].z += match moon.position.z.cmp(&other_moon.position.z) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};
			}
		}

		for (moon_index, moon) in moons.iter_mut().enumerate() {
			moon.velocity.move_by(&adjustments[moon_index]);
			moon.move_moon();
		}

		iterations += 1;
		if iterations % 1000000 == 0 {
			println!("{}", iterations);
		}
	}

	let mut cycle_length = lcm(cycle_total.x, cycle_total.y);
	cycle_length = lcm(cycle_length, cycle_total.z);
	let offset = *[cycle_start.x, cycle_start.y, cycle_start.z].iter().max().unwrap();
	let first_duplicate = cycle_length + offset;
	println!("{}", first_duplicate);
}
