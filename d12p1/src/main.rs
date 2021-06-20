use std::cmp::Ordering;
use std::ops::Add;

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

impl Default for Coordinates {
	fn default() -> Self {
		Self { x: 0, y: 0, z: 0 }
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
}

fn main() {
	let mut moons = vec![
		// Moon coordinates from puzzle input
		Moon::new(5, 4, 4),
		Moon::new(-11, -11, -3),
		Moon::new(0, 7, 0),
		Moon::new(-13, 2, 10),
	];

	for _ in 0..1000 {
		let mut adjustments: Vec<Coordinates> = Vec::new();
		for moon_index in 0..moons.len() {
			let moon = &moons[moon_index];
			let mut moon_adjustments = Coordinates::default();
			for (other_moon_index, other_moon) in moons.iter().enumerate() {
				if moon_index == other_moon_index {
					continue;
				}

				moon_adjustments.x += match moon.position.x.cmp(&other_moon.position.x) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};

				moon_adjustments.y += match moon.position.y.cmp(&other_moon.position.y) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};

				moon_adjustments.z += match moon.position.z.cmp(&other_moon.position.z) {
					Ordering::Less => 1,
					Ordering::Greater => -1,
					Ordering::Equal => 0,
				};
			}
			adjustments.push(moon_adjustments);
		}

		for moon_index in 0..moons.len() {
			moons[moon_index].velocity.move_by(&adjustments[moon_index]);
		}

		moons.iter_mut().for_each(|moon| moon.move_moon());
	}

	let total_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
	println!("{}", total_energy);
}
