use std::collections::HashSet;
use std::fs;

fn main() {
	let mut bug_grid = ['.'; 25];
	{
		let input = fs::read_to_string("input.txt").unwrap();
		let mut char_num: usize = 0;
		for character in input.chars() {
			if character == '#' || character == '.' {
				bug_grid[char_num] = character;
				char_num += 1;
			}
		}
	}

	let mut history: HashSet<[char; 25]> = HashSet::new();
	loop {
		for row in 0..5 {
			for column in 0..5 {
				print!("{}", bug_grid[row * 5 + column]);
			}
			println!();
		}
		println!();
		if history.contains(&bug_grid) {
			break;
		}

		let mut current_grid = bug_grid;
		history.insert(bug_grid);

		for (space_index, space) in current_grid.iter_mut().enumerate() {
			let mut surrounding_spaces: Vec<char> = Vec::with_capacity(4);
			if space_index % 5 > 0 {
				surrounding_spaces.push(bug_grid[space_index - 1]);
			}
			if space_index >= 5 {
				surrounding_spaces.push(bug_grid[space_index - 5]);
			}
			if space_index % 5 < 4 {
				surrounding_spaces.push(bug_grid[space_index + 1]);
			}
			if space_index < 20 {
				surrounding_spaces.push(bug_grid[space_index + 5]);
			}
			if *space == '#' {
				if surrounding_spaces.iter().filter(|v| **v == '#').count() != 1 {
					*space = '.';
				}
			} else {
				let bug_count = surrounding_spaces.iter().filter(|v| **v == '#').count();
				if bug_count == 1 || bug_count == 2 {
					*space = '#';
				}
			}
		}
		bug_grid = current_grid;
	}

	let mut biodiversity_rating: u32 = 0;
	for (tile_index, tile) in bug_grid.iter().enumerate() {
		if *tile == '#' {
			biodiversity_rating += 2u32.pow(tile_index as u32);
		}
	}
	println!("{}", biodiversity_rating);
}
