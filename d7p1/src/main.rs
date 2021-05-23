use std::collections::VecDeque;

fn initial_program() -> Vec<i32> {
	vec![
		3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 55, 64, 89, 114, 195, 276, 357, 438, 99999, 3, 9, 101, 3, 9, 9,
		102, 3, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 1002, 9, 3, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 101, 3,
		9, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 1002, 9, 5, 9, 101, 5, 9, 9, 102, 3, 9, 9, 4, 9, 99, 3, 9,
		101, 3, 9, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3,
		9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
		1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002,
		9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102,
		2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9,
		9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
		4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
		3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99,
		3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
		9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
		1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
		101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001,
		9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99,
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

fn jump_if_true_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let condition = if immediate_mode.check_argument(0) {
		program[*instruction_pointer + 1]
	} else {
		let position = value_as_position(program[*instruction_pointer + 1]);
		program[position]
	};
	if condition == 0 {
		*instruction_pointer += 3;
	} else {
		let destination = if immediate_mode.check_argument(1) {
			value_as_position(program[*instruction_pointer + 2])
		} else {
			let position = value_as_position(program[*instruction_pointer + 2]);
			value_as_position(program[position])
		};
		*instruction_pointer = destination;
	}
}

fn jump_if_false_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let condition = if immediate_mode.check_argument(0) {
		program[*instruction_pointer + 1]
	} else {
		let position = value_as_position(program[*instruction_pointer + 1]);
		program[position]
	};
	if condition == 0 {
		let destination = if immediate_mode.check_argument(1) {
			value_as_position(program[*instruction_pointer + 2])
		} else {
			let position = value_as_position(program[*instruction_pointer + 2]);
			value_as_position(program[position])
		};
		*instruction_pointer = destination;
	} else {
		*instruction_pointer += 3;
	}
}

fn less_than_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let first_operand = if immediate_mode.check_argument(0) {
		program[*instruction_pointer + 1]
	} else {
		let position = value_as_position(program[*instruction_pointer + 1]);
		program[position]
	};
	let second_operand = if immediate_mode.check_argument(1) {
		program[*instruction_pointer + 2]
	} else {
		let position = value_as_position(program[*instruction_pointer + 2]);
		program[position]
	};
	let output_position = value_as_position(program[*instruction_pointer + 3]);
	*instruction_pointer += 4;

	if first_operand < second_operand {
		program[output_position] = 1;
	} else {
		program[output_position] = 0;
	}
}

fn equals_instruction(program: &mut [i32], instruction_pointer: &mut usize) {
	let instruction = program[*instruction_pointer];
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let first_operand = if immediate_mode.check_argument(0) {
		program[*instruction_pointer + 1]
	} else {
		let position = value_as_position(program[*instruction_pointer + 1]);
		program[position]
	};
	let second_operand = if immediate_mode.check_argument(1) {
		program[*instruction_pointer + 2]
	} else {
		let position = value_as_position(program[*instruction_pointer + 2]);
		program[position]
	};
	let output_position = value_as_position(program[*instruction_pointer + 3]);
	*instruction_pointer += 4;

	if first_operand == second_operand {
		program[output_position] = 1;
	} else {
		program[output_position] = 0;
	}
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
		5 => jump_if_true_instruction(program, instruction_pointer),
		6 => jump_if_false_instruction(program, instruction_pointer),
		7 => less_than_instruction(program, instruction_pointer),
		8 => equals_instruction(program, instruction_pointer),
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

fn any_are_equal(first: i32, second: i32, third: i32, fourth: i32, fifth: i32) -> bool {
	if first == second || first == third || first == fourth || first == fifth {
		return true;
	}
	if second == third || second == fourth || second == fifth {
		return true;
	}
	if third == fourth || third == fifth {
		return true;
	}
	fourth == fifth
}

fn main() {
	let mut final_values: Vec<i32> = Vec::new();
	for first in 0..5 {
		for second in 0..5 {
			for third in 0..5 {
				for fourth in 0..5 {
					for fifth in 0..5 {
						if any_are_equal(first, second, third, fourth, fifth) {
							continue;
						}
						let mut inputs: VecDeque<i32> = VecDeque::new();
						inputs.push_back(first);
						inputs.push_back(0);
						let mut outputs: Vec<i32> = execute_program(&mut inputs);

						inputs.clear();
						inputs.push_back(second);
						inputs.push_back(outputs[0]);
						outputs = execute_program(&mut inputs);

						inputs.clear();
						inputs.push_back(third);
						inputs.push_back(outputs[0]);
						outputs = execute_program(&mut inputs);

						inputs.clear();
						inputs.push_back(fourth);
						inputs.push_back(outputs[0]);
						outputs = execute_program(&mut inputs);

						inputs.clear();
						inputs.push_back(fifth);
						inputs.push_back(outputs[0]);
						outputs = execute_program(&mut inputs);
						final_values.push(outputs[0]);
					}
				}
			}
		}
	}

	let max = final_values.iter().max().unwrap();
	println!("{}", *max);
}
