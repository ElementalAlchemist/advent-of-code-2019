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

fn passthrough_program() -> Vec<i32> {
	vec![3, 3, 104, 0, 99]
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

struct ProgramState {
	program: Vec<i32>,
	current_position: usize,
}

impl ProgramState {
	fn new() -> Self {
		let program = initial_program();
		Self {
			program,
			current_position: 0,
		}
	}

	fn new_passthrough() -> Self {
		let program = passthrough_program();
		Self {
			program,
			current_position: 0,
		}
	}

	fn get_current_position_value(&self) -> i32 {
		self.program[self.current_position]
	}

	fn get_advanced_position_value(&self, ahead_by: usize) -> i32 {
		self.program[self.current_position + ahead_by]
	}
}

fn add_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let left_addend = program_state.get_advanced_position_value(1);
	let right_addend = program_state.get_advanced_position_value(2);
	let output_position = value_as_position(program_state.get_advanced_position_value(3));
	program_state.current_position += 4;

	let left_addend = if immediate_mode.check_argument(0) {
		left_addend
	} else {
		program_state.program[value_as_position(left_addend)]
	};

	let right_addend = if immediate_mode.check_argument(1) {
		right_addend
	} else {
		program_state.program[value_as_position(right_addend)]
	};

	let output = left_addend + right_addend;

	program_state.program[output_position] = output;
}

fn multiply_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let left_factor = program_state.get_advanced_position_value(1);
	let right_factor = program_state.get_advanced_position_value(2);
	let output_position = value_as_position(program_state.get_advanced_position_value(3));
	program_state.current_position += 4;

	let left_factor = if immediate_mode.check_argument(0) {
		left_factor
	} else {
		program_state.program[value_as_position(left_factor)]
	};

	let right_factor = if immediate_mode.check_argument(1) {
		right_factor
	} else {
		program_state.program[value_as_position(right_factor)]
	};

	let output = left_factor * right_factor;

	program_state.program[output_position] = output;
}

fn input_instruction(program_state: &mut ProgramState, inputs: &mut VecDeque<i32>) -> bool {
	let input = inputs.pop_front();
	if let Some(input) = input {
		let set_position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.current_position += 2;
		program_state.program[set_position] = input;
		true
	} else {
		false
	}
}

fn output_instruction(program_state: &mut ProgramState, outputs: &mut Vec<i32>) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let output = if immediate_mode.check_argument(0) {
		program_state.get_advanced_position_value(1)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.program[position]
	};
	program_state.current_position += 2;
	outputs.push(output);
}

fn jump_if_true_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let condition = if immediate_mode.check_argument(0) {
		program_state.get_advanced_position_value(1)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.program[position]
	};
	if condition == 0 {
		program_state.current_position += 3;
	} else {
		let destination = if immediate_mode.check_argument(1) {
			value_as_position(program_state.get_advanced_position_value(2))
		} else {
			let position = value_as_position(program_state.get_advanced_position_value(2));
			value_as_position(program_state.program[position])
		};
		program_state.current_position = destination;
	}
}

fn jump_if_false_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let condition = if immediate_mode.check_argument(0) {
		program_state.get_advanced_position_value(1)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.program[position]
	};
	if condition == 0 {
		let destination = if immediate_mode.check_argument(1) {
			value_as_position(program_state.get_advanced_position_value(2))
		} else {
			let position = value_as_position(program_state.get_advanced_position_value(2));
			value_as_position(program_state.program[position])
		};
		program_state.current_position = destination;
	} else {
		program_state.current_position += 3;
	}
}

fn less_than_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let first_operand = if immediate_mode.check_argument(0) {
		program_state.get_advanced_position_value(1)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.program[position]
	};
	let second_operand = if immediate_mode.check_argument(1) {
		program_state.get_advanced_position_value(2)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(2));
		program_state.program[position]
	};
	let output_position = value_as_position(program_state.get_advanced_position_value(3));
	program_state.current_position += 4;

	if first_operand < second_operand {
		program_state.program[output_position] = 1;
	} else {
		program_state.program[output_position] = 0;
	}
}

fn equals_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let immediate_mode = ImmediateModeArguments::new(instruction);

	let first_operand = if immediate_mode.check_argument(0) {
		program_state.get_advanced_position_value(1)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(1));
		program_state.program[position]
	};
	let second_operand = if immediate_mode.check_argument(1) {
		program_state.get_advanced_position_value(2)
	} else {
		let position = value_as_position(program_state.get_advanced_position_value(2));
		program_state.program[position]
	};
	let output_position = value_as_position(program_state.get_advanced_position_value(3));
	program_state.current_position += 4;

	if first_operand == second_operand {
		program_state.program[output_position] = 1;
	} else {
		program_state.program[output_position] = 0;
	}
}

enum InstructionStatus {
	WaitingForInput,
	InstructionComplete,
	ProgramComplete,
}

/// Executes the program instruction at the provided location.
/// Returns true when execution of the program is complete.
fn execute_instruction(
	program_state: &mut ProgramState,
	inputs: &mut VecDeque<i32>,
	outputs: &mut Vec<i32>,
) -> InstructionStatus {
	let instruction = program_state.get_current_position_value() % 100;
	match instruction {
		1 => add_instruction(program_state),
		2 => multiply_instruction(program_state),
		3 => {
			if !input_instruction(program_state, inputs) {
				return InstructionStatus::WaitingForInput;
			}
		}
		4 => output_instruction(program_state, outputs),
		5 => jump_if_true_instruction(program_state),
		6 => jump_if_false_instruction(program_state),
		7 => less_than_instruction(program_state),
		8 => equals_instruction(program_state),
		99 => return InstructionStatus::ProgramComplete,
		_ => panic!(
			"The Intcode program encountered an invalid instruction: {} (at position {})",
			instruction, program_state.current_position
		),
	}
	InstructionStatus::InstructionComplete
}

enum RunStatus {
	WaitingForInput,
	Complete,
}

fn execute_program(program_state: &mut ProgramState, inputs: &mut VecDeque<i32>, outputs: &mut Vec<i32>) -> RunStatus {
	loop {
		match execute_instruction(program_state, inputs, outputs) {
			InstructionStatus::WaitingForInput => break RunStatus::WaitingForInput,
			InstructionStatus::InstructionComplete => (),
			InstructionStatus::ProgramComplete => break RunStatus::Complete,
		}
		if program_state.current_position > program_state.program.len() {
			panic!("Execution went past the end of the Intcode program.");
		}
	}
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
	for first in 5..10 {
		for second in 5..10 {
			for third in 5..10 {
				for fourth in 5..10 {
					for fifth in 5..10 {
						if any_are_equal(first, second, third, fourth, fifth) {
							continue;
						}
						let mut first_program = ProgramState::new();
						let mut second_program = ProgramState::new();
						let mut third_program = ProgramState::new();
						let mut fourth_program = ProgramState::new();
						let mut fifth_program = ProgramState::new();

						let mut complete_amplifiers: usize = 0;

						let mut input: VecDeque<i32> = VecDeque::new();
						let mut output: Vec<i32> = Vec::new();

						input.push_back(first);
						execute_program(&mut first_program, &mut input, &mut output);

						input.push_back(second);
						execute_program(&mut second_program, &mut input, &mut output);

						input.push_back(third);
						execute_program(&mut third_program, &mut input, &mut output);

						input.push_back(fourth);
						execute_program(&mut fourth_program, &mut input, &mut output);

						input.push_back(fifth);
						execute_program(&mut fifth_program, &mut input, &mut output);

						assert!(output.is_empty());

						input.push_back(0);
						let final_value = loop {
							match execute_program(&mut first_program, &mut input, &mut output) {
								RunStatus::WaitingForInput => (),
								RunStatus::Complete => {
									complete_amplifiers += 1;
									if complete_amplifiers == 5 {
										break output.pop().unwrap();
									}
								}
							}
							input.push_back(output.pop().unwrap());

							match execute_program(&mut second_program, &mut input, &mut output) {
								RunStatus::WaitingForInput => (),
								RunStatus::Complete => {
									complete_amplifiers += 1;
									if complete_amplifiers == 5 {
										break output.pop().unwrap();
									}
								}
							}
							input.push_back(output.pop().unwrap());

							match execute_program(&mut third_program, &mut input, &mut output) {
								RunStatus::WaitingForInput => (),
								RunStatus::Complete => {
									complete_amplifiers += 1;
									if complete_amplifiers == 5 {
										break output.pop().unwrap();
									}
								}
							}
							input.push_back(output.pop().unwrap());

							match execute_program(&mut fourth_program, &mut input, &mut output) {
								RunStatus::WaitingForInput => (),
								RunStatus::Complete => {
									complete_amplifiers += 1;
									if complete_amplifiers == 5 {
										break output.pop().unwrap();
									}
								}
							}
							input.push_back(output.pop().unwrap());

							match execute_program(&mut fifth_program, &mut input, &mut output) {
								RunStatus::WaitingForInput => (),
								RunStatus::Complete => {
									complete_amplifiers += 1;
									if complete_amplifiers == 5 {
										break output.pop().unwrap();
									}
								}
							}
							input.push_back(output.pop().unwrap());
						};
						final_values.push(final_value);
					}
				}
			}
		}
	}

	let max = final_values.iter().max().unwrap();
	println!("{}", *max);
}
