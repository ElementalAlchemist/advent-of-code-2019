use std::collections::HashMap;
use std::fs;

const DEFAULT_GRID: [char; 25] = [
	'.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '?', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
	'.', '.',
];

fn main() {
	let mut start_grid = DEFAULT_GRID;
	{
		let input = fs::read_to_string("input.txt").unwrap();
		let mut char_num: usize = 0;
		for character in input.chars() {
			if character == '#' || character == '.' || character == '?' {
				start_grid[char_num] = character;
				char_num += 1;
			}
		}
	}

	let mut bug_grid: HashMap<i32, [char; 25]> = HashMap::new();
	bug_grid.insert(0, start_grid);

	for _ in 0..200 {
		let mut current_grid = bug_grid.clone();

		let outermost_level = *current_grid.keys().min().unwrap();
		let innermost_level = *current_grid.keys().max().unwrap();
		if current_grid[&outermost_level].iter().any(|x| *x == '#') {
			current_grid.insert(outermost_level - 1, DEFAULT_GRID);
		}
		if current_grid[&innermost_level].iter().any(|x| *x == '#') {
			current_grid.insert(innermost_level + 1, DEFAULT_GRID);
		}

		for (level_index, level) in current_grid.iter_mut() {
			let level_index = *level_index;
			for (space_index, space) in level.iter_mut().enumerate() {
				if space_index == 12 {
					continue;
				}
				let mut surrounding_space_indices: Vec<(i32, usize)> = Vec::new();
				if space_index % 5 == 0 {
					surrounding_space_indices.push((level_index - 1, 11));
				} else {
					surrounding_space_indices.push((level_index, space_index - 1));
				}
				if space_index % 5 == 4 {
					surrounding_space_indices.push((level_index - 1, 13));
				} else {
					surrounding_space_indices.push((level_index, space_index + 1));
				}
				if space_index < 5 {
					surrounding_space_indices.push((level_index - 1, 7));
				} else {
					surrounding_space_indices.push((level_index, space_index - 5));
				}
				if space_index >= 20 {
					surrounding_space_indices.push((level_index - 1, 17));
				} else {
					surrounding_space_indices.push((level_index, space_index + 5));
				}

				let recurse_in_space = surrounding_space_indices
					.iter()
					.enumerate()
					.filter(|(_, x)| x.1 == 12)
					.map(|(index, _)| index)
					.next();
				if let Some(surround_index) = recurse_in_space {
					surrounding_space_indices.remove(surround_index);
					match space_index {
						7 => {
							surrounding_space_indices.push((level_index + 1, 0));
							surrounding_space_indices.push((level_index + 1, 1));
							surrounding_space_indices.push((level_index + 1, 2));
							surrounding_space_indices.push((level_index + 1, 3));
							surrounding_space_indices.push((level_index + 1, 4));
						}
						11 => {
							surrounding_space_indices.push((level_index + 1, 0));
							surrounding_space_indices.push((level_index + 1, 5));
							surrounding_space_indices.push((level_index + 1, 10));
							surrounding_space_indices.push((level_index + 1, 15));
							surrounding_space_indices.push((level_index + 1, 20));
						}
						13 => {
							surrounding_space_indices.push((level_index + 1, 4));
							surrounding_space_indices.push((level_index + 1, 9));
							surrounding_space_indices.push((level_index + 1, 14));
							surrounding_space_indices.push((level_index + 1, 19));
							surrounding_space_indices.push((level_index + 1, 24));
						}
						17 => {
							surrounding_space_indices.push((level_index + 1, 20));
							surrounding_space_indices.push((level_index + 1, 21));
							surrounding_space_indices.push((level_index + 1, 22));
							surrounding_space_indices.push((level_index + 1, 23));
							surrounding_space_indices.push((level_index + 1, 24));
						}
						_ => unreachable!(),
					}
				}

				let surrounding_bug_count = surrounding_space_indices
					.iter()
					.map(|(level, space)| {
						if bug_grid.contains_key(level) {
							Some((level, space))
						} else {
							None
						}
					})
					.flatten()
					.map(|(level, space)| &bug_grid[level][*space])
					.filter(|value| **value == '#')
					.count();
				if *space == '#' {
					if surrounding_bug_count != 1 {
						*space = '.';
					}
				} else if surrounding_bug_count == 1 || surrounding_bug_count == 2 {
					*space = '#';
				}
			}
		}

		bug_grid = current_grid;
	}

	let bug_count: usize = bug_grid
		.iter()
		.map(|(_, level_grid)| level_grid.iter().filter(|c| **c == '#').count())
		.sum();
	println!("{}", bug_count);
}
