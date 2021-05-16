use std::collections::VecDeque;

fn initial_program() -> Vec<i32> {
	vec![
		3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1001, 210, 88, 224, 101, -143, 224, 224, 4, 224, 1002, 223, 8,
		223, 101, 3, 224, 224, 1, 223, 224, 223, 101, 42, 92, 224, 101, -78, 224, 224, 4, 224, 1002, 223, 8, 223, 1001,
		224, 3, 224, 1, 223, 224, 223, 1101, 73, 10, 225, 1102, 38, 21, 225, 1102, 62, 32, 225, 1, 218, 61, 224, 1001,
		224, -132, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 5, 224, 1, 224, 223, 223, 1102, 19, 36, 225, 102, 79, 65,
		224, 101, -4898, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 224, 223, 223, 1101, 66, 56, 224,
		1001, 224, -122, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1002, 58, 82, 224, 101,
		-820, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 223, 224, 223, 2, 206, 214, 224, 1001, 224,
		-648, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 223, 224, 223, 1102, 76, 56, 224, 1001, 224, -4256,
		224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 1102, 37, 8, 225, 1101, 82, 55, 225, 1102,
		76, 81, 225, 1101, 10, 94, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999,
		1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
		1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225,
		1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0,
		106, 0, 0, 1105, 1, 99999, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 1008, 677,
		677, 224, 1002, 223, 2, 223, 1006, 224, 344, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005,
		224, 359, 1001, 223, 1, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 374, 101, 1, 223, 223, 1107,
		677, 677, 224, 1002, 223, 2, 223, 1006, 224, 389, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1006,
		224, 404, 101, 1, 223, 223, 7, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 108, 677,
		677, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
		449, 1001, 223, 1, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 464, 101, 1, 223, 223, 8, 226, 226,
		224, 1002, 223, 2, 223, 1006, 224, 479, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224,
		494, 1001, 223, 1, 223, 1008, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 509, 101, 1, 223, 223, 7, 677, 226,
		224, 102, 2, 223, 223, 1006, 224, 524, 101, 1, 223, 223, 1007, 677, 226, 224, 1002, 223, 2, 223, 1006, 224,
		539, 1001, 223, 1, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 554, 1001, 223, 1, 223, 8, 677, 226,
		224, 1002, 223, 2, 223, 1005, 224, 569, 101, 1, 223, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224,
		584, 101, 1, 223, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 599, 101, 1, 223, 223, 107, 226, 226,
		224, 102, 2, 223, 223, 1006, 224, 614, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
		629, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 1007, 677,
		677, 224, 102, 2, 223, 223, 1006, 224, 659, 1001, 223, 1, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1006,
		224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
	]
}

fn value_as_position(value: i32) -> usize {
	if value < 0 {
		panic!("Invalid position");
	}
	value as usize
}

struct ImmediateModeArguments {
	parameters: Vec<bool>,
}

impl ImmediateModeArguments {
	fn new(instruction: i32) -> Self {
		let instruction = instruction.to_string();
		let mut instruction_chars = instruction.chars().rev();
		instruction_chars.next();
		instruction_chars.next();
		let mut parameters: Vec<bool> = Vec::new();
		for instruction_char in instruction_chars {
			parameters.push(instruction_char == '1');
		}
		Self { parameters }
	}

	fn check_argument(&self, argument_position: usize) -> bool {
		if let Some(val) = self.parameters.get(argument_position) {
			*val
		} else {
			false
		}
	}
}

fn add_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let left_addend = program[*instruction_pointer + 1];
	let right_addend = program[*instruction_pointer + 2];
	let output_position = value_as_position(program[*instruction_pointer + 3]);
	*instruction_pointer += 4;

	let left_addend = if immediate_mode.check_argument(0) {
		left_addend
	} else {
		program[value_as_position(left_addend)]
	};

	let right_addend = if immediate_mode.check_argument(1) {
		right_addend
	} else {
		program[value_as_position(right_addend)]
	};

	let output = left_addend + right_addend;

	program[output_position] = output;
}

fn multiply_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let left_factor = program[*instruction_pointer + 1];
	let right_factor = program[*instruction_pointer + 2];
	let output_position = value_as_position(program[*instruction_pointer + 3]);
	*instruction_pointer += 4;

	let left_factor = if immediate_mode.check_argument(0) {
		left_factor
	} else {
		program[value_as_position(left_factor)]
	};

	let right_factor = if immediate_mode.check_argument(1) {
		right_factor
	} else {
		program[value_as_position(right_factor)]
	};

	let output = left_factor * right_factor;

	program[output_position] = output;
}

fn input_instruction(program: &mut [i32], instruction_pointer: &mut usize, inputs: &mut VecDeque<i32>) {
	let input = inputs.pop_front().expect("Ran out of inputs");
	let set_position = value_as_position(program[*instruction_pointer + 1]);
	*instruction_pointer += 2;
	program[set_position] = input;
}

fn output_instruction(program: &mut [i32], instruction_pointer: &mut usize, outputs: &mut Vec<i32>) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let output = if immediate_mode.check_argument(0) {
		program[*instruction_pointer + 1]
	} else {
		let position = value_as_position(program[*instruction_pointer + 1]);
		program[position]
	};
	*instruction_pointer += 2;
	outputs.push(output);
}

/// Executes the program instruction at the provided location.
/// Returns true when execution of the program is complete.
fn execute_instruction(
	program: &mut [i32],
	instruction_pointer: &mut usize,
	inputs: &mut VecDeque<i32>,
	outputs: &mut Vec<i32>,
) -> bool {
	let instruction = program[*instruction_pointer] % 100;
	match instruction {
		1 => add_instruction(program, instruction_pointer),
		2 => multiply_instruction(program, instruction_pointer),
		3 => input_instruction(program, instruction_pointer, inputs),
		4 => output_instruction(program, instruction_pointer, outputs),
		99 => return true,
		_ => panic!(
			"The Intcode program encountered an invalid instruction: {} (at position {})",
			instruction, instruction_pointer
		),
	}
	false
}

fn execute_program(inputs: &mut VecDeque<i32>) -> Vec<i32> {
	let mut program: Vec<i32> = initial_program();
	let mut current_position: usize = 0;
	let mut outputs: Vec<i32> = Vec::new();
	loop {
		if execute_instruction(&mut program, &mut current_position, inputs, &mut outputs) {
			break;
		}
		if current_position > program.len() {
			panic!("Execution went past the end of the Intcode program.");
		}
	}

	outputs
}

fn main() {
	let mut inputs: VecDeque<i32> = VecDeque::new();
	inputs.push_back(1);
	let output = execute_program(&mut inputs);
	println!("{:?}", output);
}
