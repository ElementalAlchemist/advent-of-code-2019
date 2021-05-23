use std::fs;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_AREA: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn main() {
	let raw_image_data: Vec<char> = {
		let input = fs::read_to_string("input.txt").unwrap();
		input.chars().filter(|c| c.is_digit(10)).collect()
	};
	assert!(raw_image_data.len() % IMAGE_AREA == 0);
	let layers: Vec<&[char]> = {
		let mut layers = Vec::new();
		for layer_num in 0..(raw_image_data.len() / IMAGE_AREA) {
			let layer_start = layer_num * IMAGE_AREA;
			layers.push(&raw_image_data[layer_start..(layer_start + IMAGE_AREA)]);
		}
		layers
	};

	for row in 0..IMAGE_HEIGHT {
		for column in 0..IMAGE_WIDTH {
			let index = row * IMAGE_WIDTH + column;
			let mut is_visible = false;
			for layer in layers.iter() {
				if layer[index] == '0' {
					is_visible = true;
					break;
				} else if layer[index] == '1' {
					break;
				}
			}
			if is_visible {
				print!(" ");
			} else {
				print!("x");
			}
		}
		println!();
	}
}
