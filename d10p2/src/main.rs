use num::integer::gcd;
use std::fs;

fn difference(lhs: usize, rhs: usize) -> usize {
	if lhs > rhs {
		lhs - rhs
	} else {
		rhs - lhs
	}
}

/// Given a current coordinate and a place to go to, outputs the next coordinate
fn move_toward(current_coord: usize, dest_coord: usize, jump_by: usize) -> usize {
	if current_coord > dest_coord {
		current_coord - jump_by
	} else {
		current_coord + jump_by
	}
}

fn count_raytrace_from(grid: &[Vec<bool>], start_x: usize, start_y: usize) -> u32 {
	let mut count: u32 = 0;
	for check_y in 0..grid.len() {
		for check_x in 0..grid[check_y].len() {
			if !grid[check_y][check_x] {
				continue;
			}
			if check_x == start_x && check_y == start_y {
				continue;
			}
			let diff_x = difference(start_x, check_x);
			let diff_y = difference(start_y, check_y);
			let diff_gcd = gcd(diff_x, diff_y);
			let diff_x = diff_x / diff_gcd;
			let diff_y = diff_y / diff_gcd;
			let mut current_x = start_x;
			let mut current_y = start_y;
			loop {
				current_x = move_toward(current_x, check_x, diff_x);
				current_y = move_toward(current_y, check_y, diff_y);
				if grid[current_y][current_x] {
					if (current_x, current_y) == (check_x, check_y) {
						count += 1;
					}
					break;
				}
			}
		}
	}
	count
}

fn check_raytrace_clear(grid: &[Vec<bool>], start_x: usize, start_y: usize, check_x: usize, check_y: usize) -> bool {
	let diff_x = difference(start_x, check_x);
	let diff_y = difference(start_y, check_y);
	let diff_gcd = gcd(diff_x, diff_y);
	let diff_x = diff_x / diff_gcd;
	let diff_y = diff_y / diff_gcd;
	let mut current_x = start_x;
	let mut current_y = start_y;
	loop {
		current_x = move_toward(current_x, check_x, diff_x);
		current_y = move_toward(current_y, check_y, diff_y);
		if grid[current_y][current_x] {
			break (current_x, current_y) == (check_x, check_y);
		}
	}
}

fn main() {
	let mut asteroid_grid: Vec<Vec<bool>> = {
		let input = fs::read_to_string("input.txt").unwrap();
		input
			.lines()
			.filter(|l| !l.is_empty())
			.map(|line| {
				let mut asteroid_row: Vec<bool> = Vec::new();
				for position in line.chars() {
					if position == '#' {
						asteroid_row.push(true);
					} else {
						asteroid_row.push(false);
					}
				}
				asteroid_row
			})
			.collect()
	};

	let mut scanner_x: usize = 0;
	let mut scanner_y: usize = 0;
	let mut greatest_count: u32 = 0;

	for y in 0..asteroid_grid.len() {
		for x in 0..asteroid_grid[y].len() {
			if asteroid_grid[y][x] {
				let count = count_raytrace_from(&asteroid_grid, x, y);
				if count > greatest_count {
					greatest_count = count;
					scanner_x = x;
					scanner_y = y;
				}
			}
		}
	}

	println!("Scanner: ({}, {})", scanner_x, scanner_y);

	let mut current_angle: f64 = 0.0;
	let mut match_angle = true;
	let mut destroyed_asteroids: u32 = 0;
	loop {
		let mut smallest_angle_difference: f64 = 720.0;
		let mut next_angle: f64 = 0.0;
		let mut smallest_angle_x: usize = 0;
		let mut smallest_angle_y: usize = 0;
		let mut found_any_asteroids = false;
		let mut set_new_angle = false;
		for (y, row) in asteroid_grid.iter().enumerate() {
			for x in 0..row.len() {
				if y == scanner_y && x == scanner_x {
					continue;
				}
				if !asteroid_grid[y][x] {
					continue;
				}
				found_any_asteroids = true;
				if !check_raytrace_clear(&asteroid_grid, scanner_x, scanner_y, x, y) {
					continue;
				}
				let mut angle = -(((x as f64 - scanner_x as f64) / (y as f64 - scanner_y as f64))
					.atan()
					.to_degrees());
				if y >= scanner_y {
					angle += 180.0;
				}
				while angle < current_angle {
					angle += 360.0;
				}
				let angle_difference = angle - current_angle;
				if angle_difference < smallest_angle_difference
					&& (match_angle || (angle - current_angle).abs() > f64::EPSILON)
				{
					smallest_angle_difference = angle_difference;
					next_angle = angle;
					smallest_angle_x = x;
					smallest_angle_y = y;
					set_new_angle = true;
				}
			}
		}
		if !found_any_asteroids {
			break;
		}
		if !set_new_angle {
			match_angle = true; // There are no asteroids except in the last line
			continue;
		}
		asteroid_grid[smallest_angle_y][smallest_angle_x] = false;
		match_angle = false;
		current_angle = next_angle;
		if current_angle >= 360.0 {
			current_angle -= 360.0;
		}
		destroyed_asteroids += 1;
		println!(
			"{}: ({}, {}) ({}, {}) {}",
			destroyed_asteroids,
			smallest_angle_x,
			smallest_angle_y,
			smallest_angle_x as i64 - scanner_x as i64,
			smallest_angle_y as i64 - scanner_y as i64,
			current_angle
		);
	}
}
