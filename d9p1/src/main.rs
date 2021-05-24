use std::collections::VecDeque;

fn initial_program() -> Vec<i64> {
	vec![
		1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 1, 3, 1000, 109, 988, 209, 12, 9,
		1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000, 2, 63, 1005, 63, 902, 1008, 1000, 0,
		63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 39, 1005, 1102, 1, 1,
		1021, 1101, 0, 212, 1025, 1101, 0, 24, 1014, 1102, 22, 1, 1019, 1101, 0, 35, 1003, 1101, 38, 0, 1002, 1101, 0,
		571, 1026, 1102, 32, 1, 1006, 1102, 31, 1, 1000, 1102, 25, 1, 1018, 1102, 1, 37, 1016, 1101, 0, 820, 1023,
		1102, 1, 29, 1004, 1101, 564, 0, 1027, 1101, 0, 375, 1028, 1101, 26, 0, 1013, 1102, 1, 370, 1029, 1101, 21, 0,
		1007, 1101, 0, 0, 1020, 1102, 1, 30, 1001, 1102, 36, 1, 1011, 1102, 1, 27, 1017, 1101, 0, 28, 1012, 1101, 0,
		217, 1024, 1101, 823, 0, 1022, 1102, 1, 20, 1009, 1101, 0, 23, 1010, 1101, 34, 0, 1015, 1101, 33, 0, 1008, 109,
		5, 1208, 0, 39, 63, 1005, 63, 199, 4, 187, 1106, 0, 203, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 2105, 1, 6,
		4, 209, 1105, 1, 221, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -4, 21108, 40, 39, -1, 1005, 1013, 241, 1001, 64,
		1, 64, 1105, 1, 243, 4, 227, 1002, 64, 2, 64, 109, 5, 21102, 41, 1, -1, 1008, 1018, 40, 63, 1005, 63, 267,
		1001, 64, 1, 64, 1106, 0, 269, 4, 249, 1002, 64, 2, 64, 109, -28, 1202, 10, 1, 63, 1008, 63, 30, 63, 1005, 63,
		291, 4, 275, 1106, 0, 295, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 24, 21107, 42, 43, -4, 1005, 1011, 313, 4,
		301, 1106, 0, 317, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -8, 21108, 43, 43, 3, 1005, 1010, 335, 4, 323, 1105,
		1, 339, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -8, 1207, 4, 34, 63, 1005, 63, 359, 1001, 64, 1, 64, 1106, 0,
		361, 4, 345, 1002, 64, 2, 64, 109, 26, 2106, 0, 3, 4, 367, 1106, 0, 379, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
		-21, 2102, 1, -2, 63, 1008, 63, 37, 63, 1005, 63, 399, 1105, 1, 405, 4, 385, 1001, 64, 1, 64, 1002, 64, 2, 64,
		109, 2, 1207, -2, 30, 63, 1005, 63, 427, 4, 411, 1001, 64, 1, 64, 1105, 1, 427, 1002, 64, 2, 64, 109, 4, 2108,
		36, -5, 63, 1005, 63, 447, 1001, 64, 1, 64, 1106, 0, 449, 4, 433, 1002, 64, 2, 64, 109, -13, 1201, 8, 0, 63,
		1008, 63, 41, 63, 1005, 63, 469, 1106, 0, 475, 4, 455, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 21107, 44,
		43, 3, 1005, 1014, 495, 1001, 64, 1, 64, 1106, 0, 497, 4, 481, 1002, 64, 2, 64, 109, 2, 1205, 8, 511, 4, 503,
		1106, 0, 515, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 1206, -6, 527, 1105, 1, 533, 4, 521, 1001, 64, 1, 64,
		1002, 64, 2, 64, 109, -29, 2107, 31, 8, 63, 1005, 63, 551, 4, 539, 1105, 1, 555, 1001, 64, 1, 64, 1002, 64, 2,
		64, 109, 28, 2106, 0, 1, 1001, 64, 1, 64, 1106, 0, 573, 4, 561, 1002, 64, 2, 64, 109, -3, 21101, 45, 0, -4,
		1008, 1019, 45, 63, 1005, 63, 595, 4, 579, 1105, 1, 599, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -23, 1208, 2,
		39, 63, 1005, 63, 615, 1105, 1, 621, 4, 605, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 15, 2108, 32, -9, 63, 1005,
		63, 643, 4, 627, 1001, 64, 1, 64, 1105, 1, 643, 1002, 64, 2, 64, 109, -9, 2107, 33, 0, 63, 1005, 63, 659, 1106,
		0, 665, 4, 649, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 7, 21101, 46, 0, 2, 1008, 1015, 49, 63, 1005, 63, 689,
		1001, 64, 1, 64, 1106, 0, 691, 4, 671, 1002, 64, 2, 64, 109, -8, 2101, 0, -3, 63, 1008, 63, 35, 63, 1005, 63,
		711, 1105, 1, 717, 4, 697, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 1202, -9, 1, 63, 1008, 63, 31, 63, 1005,
		63, 741, 1001, 64, 1, 64, 1105, 1, 743, 4, 723, 1002, 64, 2, 64, 109, -27, 2102, 1, 10, 63, 1008, 63, 31, 63,
		1005, 63, 769, 4, 749, 1001, 64, 1, 64, 1105, 1, 769, 1002, 64, 2, 64, 109, 9, 2101, 0, 1, 63, 1008, 63, 31,
		63, 1005, 63, 791, 4, 775, 1106, 0, 795, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 28, 1206, -7, 809, 4, 801,
		1105, 1, 813, 1001, 64, 1, 64, 1002, 64, 2, 64, 2105, 1, -4, 1106, 0, 829, 4, 817, 1001, 64, 1, 64, 1002, 64,
		2, 64, 109, -15, 21102, 47, 1, -2, 1008, 1010, 47, 63, 1005, 63, 851, 4, 835, 1106, 0, 855, 1001, 64, 1, 64,
		1002, 64, 2, 64, 109, 5, 1205, 3, 867, 1106, 0, 873, 4, 861, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -12, 1201,
		0, 0, 63, 1008, 63, 39, 63, 1005, 63, 895, 4, 879, 1105, 1, 899, 1001, 64, 1, 64, 4, 64, 99, 21101, 0, 27, 1,
		21102, 913, 1, 0, 1106, 0, 920, 21201, 1, 47951, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 962, 21201,
		-2, -1, 1, 21101, 0, 940, 0, 1105, 1, 920, 21201, 1, 0, -1, 21201, -2, -3, 1, 21101, 0, 955, 0, 1106, 0, 920,
		22201, 1, -1, -2, 1105, 1, 966, 21202, -2, 1, -2, 109, -3, 2105, 1, 0,
	]
}

fn value_as_position(value: i64) -> usize {
	if value < 0 {
		panic!("Invalid position");
	}
	value as usize
}

#[derive(Clone)]
enum ArgumentModeType {
	Position,
	Immediate,
	RelativePosition,
}

impl From<char> for ArgumentModeType {
	fn from(ch: char) -> Self {
		match ch {
			'1' => Self::Immediate,
			'2' => Self::RelativePosition,
			_ => Self::Position,
		}
	}
}

impl Default for ArgumentModeType {
	fn default() -> Self {
		Self::Position
	}
}

struct ArgumentMode {
	mode_type: ArgumentModeType,
	arg_number: usize,
}

impl ArgumentMode {
	fn new(mode_type: ArgumentModeType, arg_number: usize) -> Self {
		Self { mode_type, arg_number }
	}

	/// Gets the value of an argument, taking into account the mode and what data it needs to read from the program.
	fn get_argument_value(&self, program_state: &ProgramState) -> i64 {
		let arg_offset = self.arg_number + 1;
		let arg_value = program_state.get_advanced_position_value(arg_offset);
		match self.mode_type {
			ArgumentModeType::Position => program_state.get_program_value(value_as_position(arg_value)),
			ArgumentModeType::Immediate => arg_value,
			ArgumentModeType::RelativePosition => {
				program_state.get_program_value(value_as_position(program_state.get_relative_base() + arg_value))
			}
		}
	}

	/// Gets the value of an argument representing a position in the program to which data needs to be written.
	fn get_argument_position(&self, program_state: &ProgramState) -> usize {
		let arg_offset = self.arg_number + 1;
		let arg_value = program_state.get_advanced_position_value(arg_offset);
		let position = match self.mode_type {
			ArgumentModeType::Position => arg_value,
			ArgumentModeType::Immediate => {
				panic!("Used immediate mode to get a position value (position {})", arg_value)
			}
			ArgumentModeType::RelativePosition => program_state.get_relative_base() + arg_value,
		};
		value_as_position(position)
	}
}

struct ArgumentModes {
	parameters: Vec<ArgumentModeType>,
}

impl ArgumentModes {
	fn new(instruction: i64) -> Self {
		let instruction = instruction.to_string();
		let mut instruction_chars = instruction.chars().rev();
		instruction_chars.next();
		instruction_chars.next();
		let mut parameters: Vec<ArgumentModeType> = Vec::new();
		for instruction_char in instruction_chars {
			parameters.push(instruction_char.into());
		}
		Self { parameters }
	}

	fn check_argument(&self, argument_position: usize) -> ArgumentMode {
		if let Some(val) = self.parameters.get(argument_position) {
			ArgumentMode::new(val.clone(), argument_position)
		} else {
			ArgumentMode::new(ArgumentModeType::default(), argument_position)
		}
	}
}

struct ProgramState {
	program: Vec<i64>,
	current_position: usize,
	relative_base: i64,
}

impl ProgramState {
	fn new() -> Self {
		let program = initial_program();
		Self {
			program,
			current_position: 0,
			relative_base: 0,
		}
	}

	fn get_program_value(&self, at_position: usize) -> i64 {
		match self.program.get(at_position) {
			Some(val) => *val,
			None => 0,
		}
	}

	fn get_current_position_value(&self) -> i64 {
		self.get_program_value(self.current_position)
	}

	fn get_advanced_position_value(&self, ahead_by: usize) -> i64 {
		self.get_program_value(self.current_position + ahead_by)
	}

	fn set_program_value(&mut self, at_position: usize, value: i64) {
		while at_position >= self.program.len() {
			self.program.push(0);
		}
		self.program[at_position] = value;
	}

	fn get_relative_base(&self) -> i64 {
		self.relative_base
	}

	fn change_relative_base(&mut self, offset: i64) {
		self.relative_base += offset;
	}
}

fn add_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let left_addend = argument_modes.check_argument(0).get_argument_value(program_state);
	let right_addend = argument_modes.check_argument(1).get_argument_value(program_state);
	let output_position = argument_modes.check_argument(2).get_argument_position(program_state);
	program_state.current_position += 4;

	let output = left_addend + right_addend;

	program_state.set_program_value(output_position, output);
}

fn multiply_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let left_factor = argument_modes.check_argument(0).get_argument_value(program_state);
	let right_factor = argument_modes.check_argument(1).get_argument_value(program_state);
	let output_position = argument_modes.check_argument(2).get_argument_position(program_state);
	program_state.current_position += 4;

	let output = left_factor * right_factor;

	program_state.set_program_value(output_position, output);
}

fn input_instruction(program_state: &mut ProgramState, inputs: &mut VecDeque<i64>) -> bool {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let input = inputs.pop_front();
	if let Some(input) = input {
		let set_position = argument_modes.check_argument(0).get_argument_position(program_state);
		program_state.current_position += 2;
		program_state.set_program_value(set_position, input);
		true
	} else {
		false
	}
}

fn output_instruction(program_state: &mut ProgramState, outputs: &mut Vec<i64>) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let output = argument_modes.check_argument(0).get_argument_value(program_state);
	program_state.current_position += 2;
	outputs.push(output);
}

fn jump_if_true_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let condition = argument_modes.check_argument(0).get_argument_value(program_state);
	if condition == 0 {
		program_state.current_position += 3;
	} else {
		let destination = argument_modes.check_argument(1).get_argument_value(program_state);
		program_state.current_position = value_as_position(destination);
	}
}

fn jump_if_false_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let condition = argument_modes.check_argument(0).get_argument_value(program_state);
	if condition == 0 {
		let destination = argument_modes.check_argument(1).get_argument_value(program_state);
		program_state.current_position = value_as_position(destination);
	} else {
		program_state.current_position += 3;
	}
}

fn less_than_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let first_operand = argument_modes.check_argument(0).get_argument_value(program_state);
	let second_operand = argument_modes.check_argument(1).get_argument_value(program_state);
	let output_position = argument_modes.check_argument(2).get_argument_position(program_state);
	program_state.current_position += 4;

	if first_operand < second_operand {
		program_state.set_program_value(output_position, 1);
	} else {
		program_state.set_program_value(output_position, 0);
	}
}

fn equals_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let first_operand = argument_modes.check_argument(0).get_argument_value(program_state);
	let second_operand = argument_modes.check_argument(1).get_argument_value(program_state);
	let output_position = argument_modes.check_argument(2).get_argument_position(program_state);
	program_state.current_position += 4;

	if first_operand == second_operand {
		program_state.set_program_value(output_position, 1);
	} else {
		program_state.set_program_value(output_position, 0);
	}
}

fn set_relative_base_instruction(program_state: &mut ProgramState) {
	let instruction = program_state.get_current_position_value();
	let argument_modes = ArgumentModes::new(instruction);

	let base_offset = argument_modes.check_argument(0).get_argument_value(program_state);
	program_state.change_relative_base(base_offset);
	program_state.current_position += 2;
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
	inputs: &mut VecDeque<i64>,
	outputs: &mut Vec<i64>,
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
		9 => set_relative_base_instruction(program_state),
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

fn execute_program(program_state: &mut ProgramState, inputs: &mut VecDeque<i64>, outputs: &mut Vec<i64>) -> RunStatus {
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

fn main() {
	let mut input: VecDeque<i64> = VecDeque::new();
	input.push_back(1);
	let mut output: Vec<i64> = Vec::new();
	let mut program_state = ProgramState::new();
	execute_program(&mut program_state, &mut input, &mut output);
	println!("{:?}", output);
}
