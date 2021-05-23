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

	let mut layer_iter = layers.iter().enumerate();
	let (_, first_layer) = layer_iter.next().unwrap();
	let mut fewest_zeroes = first_layer.iter().filter(|c| **c == '0').count();
	let mut fewest_zeroes_layer: usize = 0;
	for (layer_index, layer) in layer_iter {
		let zero_count = layer.iter().filter(|c| **c == '0').count();
		if zero_count < fewest_zeroes {
			fewest_zeroes = zero_count;
			fewest_zeroes_layer = layer_index;
		}
	}

	let mut one_count: usize = 0;
	let mut two_count: usize = 0;
	for digit in layers[fewest_zeroes_layer].iter() {
		if *digit == '1' {
			one_count += 1;
		} else if *digit == '2' {
			two_count += 1;
		}
	}
	let answer = one_count * two_count;
	println!("{}", answer);
}
