fn initial_program() -> Vec<usize> {
	vec![
		1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 2, 23, 10, 27, 1, 6, 27, 31, 1, 31,
		6, 35, 2, 35, 10, 39, 1, 39, 5, 43, 2, 6, 43, 47, 2, 47, 10, 51, 1, 51, 6, 55, 1, 55, 6, 59, 1, 9, 59, 63, 1,
		63, 9, 67, 1, 67, 6, 71, 2, 71, 13, 75, 1, 75, 5, 79, 1, 79, 9, 83, 2, 6, 83, 87, 1, 87, 5, 91, 2, 6, 91, 95,
		1, 95, 9, 99, 2, 6, 99, 103, 1, 5, 103, 107, 1, 6, 107, 111, 1, 111, 10, 115, 2, 115, 13, 119, 1, 119, 6, 123,
		1, 123, 2, 127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
	]
}

/// Executes the program instruction at the provided location.
/// Returns the next instruction location or None to stop execution.
fn execute_instruction(program: &mut [usize], instruction_pointer: usize) -> Option<usize> {
	let instruction = program[instruction_pointer];
	match instruction {
		1 => {
			let left_position = program[instruction_pointer + 1];
			let right_position = program[instruction_pointer + 2];
			let destination_position = program[instruction_pointer + 3];

			let left = program[left_position];
			let right = program[right_position];

			program[destination_position] = left + right;
			Some(instruction_pointer + 4)
		}
		2 => {
			let left_position = program[instruction_pointer + 1];
			let right_position = program[instruction_pointer + 2];
			let destination_position = program[instruction_pointer + 3];

			let left = program[left_position];
			let right = program[right_position];

			program[destination_position] = left * right;
			Some(instruction_pointer + 4)
		}
		99 => None,
		_ => panic!("The Intcode program encountered an error."),
	}
}

fn execute_program(noun: usize, verb: usize) -> usize {
	let mut program: Vec<usize> = initial_program();
	program[1] = noun;
	program[2] = verb;
	let mut current_position: usize = 0;
	loop {
		let next_instruction = execute_instruction(&mut program, current_position);
		current_position = match next_instruction {
			Some(pos) => pos,
			None => break,
		};
		if current_position > program.len() {
			panic!("Execution went past the end of the Intcode program.");
		}
	}

	program[0]
}

fn main() {
	for noun in 0..99usize {
		for verb in 0..99usize {
			let output = execute_program(noun, verb);
			if output == 19690720 {
				let input_combined = noun * 100 + verb;
				println!("{}, {}, {}", noun, verb, input_combined);
			}
		}
	}
}
