use num::integer::gcd;
use std::fs;

fn difference(x: usize, y: usize) -> usize {
	if x > y {
		x - y
	} else {
		y - x
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

fn main() {
	let asteroid_grid: Vec<Vec<bool>> = {
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

	let mut greatest_x: usize = 0;
	let mut greatest_y: usize = 0;
	let mut greatest_count: u32 = 0;

	for y in 0..asteroid_grid.len() {
		for x in 0..asteroid_grid[y].len() {
			if asteroid_grid[y][x] {
				let count = count_raytrace_from(&asteroid_grid, x, y);
				if count > greatest_count {
					greatest_count = count;
					greatest_x = x;
					greatest_y = y;
				}
			}
		}
	}

	println!("({}, {}) -> {}", greatest_x, greatest_y, greatest_count);
}
