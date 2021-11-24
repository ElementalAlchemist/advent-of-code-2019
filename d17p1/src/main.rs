use std::collections::{HashSet, VecDeque};

fn initial_program() -> Vec<i64> {
	vec![
		1, 330, 331, 332, 109, 3564, 1102, 1182, 1, 15, 1101, 0, 1449, 24, 1002, 0, 1, 570, 1006, 570, 36, 101, 0, 571,
		0, 1001, 570, -1, 570, 1001, 24, 1, 24, 1106, 0, 18, 1008, 571, 0, 571, 1001, 15, 1, 15, 1008, 15, 1449, 570,
		1006, 570, 14, 21102, 58, 1, 0, 1105, 1, 786, 1006, 332, 62, 99, 21101, 333, 0, 1, 21102, 1, 73, 0, 1106, 0,
		579, 1101, 0, 0, 572, 1102, 1, 0, 573, 3, 574, 101, 1, 573, 573, 1007, 574, 65, 570, 1005, 570, 151, 107, 67,
		574, 570, 1005, 570, 151, 1001, 574, -64, 574, 1002, 574, -1, 574, 1001, 572, 1, 572, 1007, 572, 11, 570, 1006,
		570, 165, 101, 1182, 572, 127, 1001, 574, 0, 0, 3, 574, 101, 1, 573, 573, 1008, 574, 10, 570, 1005, 570, 189,
		1008, 574, 44, 570, 1006, 570, 158, 1105, 1, 81, 21102, 340, 1, 1, 1106, 0, 177, 21102, 477, 1, 1, 1105, 1,
		177, 21101, 0, 514, 1, 21101, 176, 0, 0, 1106, 0, 579, 99, 21102, 1, 184, 0, 1105, 1, 579, 4, 574, 104, 10, 99,
		1007, 573, 22, 570, 1006, 570, 165, 102, 1, 572, 1182, 21102, 375, 1, 1, 21102, 1, 211, 0, 1106, 0, 579, 21101,
		1182, 11, 1, 21102, 222, 1, 0, 1105, 1, 979, 21101, 388, 0, 1, 21101, 233, 0, 0, 1105, 1, 579, 21101, 1182, 22,
		1, 21102, 1, 244, 0, 1105, 1, 979, 21102, 1, 401, 1, 21102, 255, 1, 0, 1105, 1, 579, 21101, 1182, 33, 1, 21102,
		1, 266, 0, 1106, 0, 979, 21102, 1, 414, 1, 21102, 1, 277, 0, 1105, 1, 579, 3, 575, 1008, 575, 89, 570, 1008,
		575, 121, 575, 1, 575, 570, 575, 3, 574, 1008, 574, 10, 570, 1006, 570, 291, 104, 10, 21102, 1, 1182, 1, 21101,
		313, 0, 0, 1105, 1, 622, 1005, 575, 327, 1102, 1, 1, 575, 21102, 1, 327, 0, 1105, 1, 786, 4, 438, 99, 0, 1, 1,
		6, 77, 97, 105, 110, 58, 10, 33, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 102, 117, 110, 99, 116, 105,
		111, 110, 32, 110, 97, 109, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 0, 12, 70, 117, 110, 99, 116,
		105, 111, 110, 32, 65, 58, 10, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 66, 58, 10, 12, 70, 117, 110, 99,
		116, 105, 111, 110, 32, 67, 58, 10, 23, 67, 111, 110, 116, 105, 110, 117, 111, 117, 115, 32, 118, 105, 100,
		101, 111, 32, 102, 101, 101, 100, 63, 10, 0, 37, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 82, 44, 32, 76,
		44, 32, 111, 114, 32, 100, 105, 115, 116, 97, 110, 99, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 36,
		10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 99, 111, 109, 109, 97, 32, 111, 114, 32, 110, 101, 119, 108, 105,
		110, 101, 32, 98, 117, 116, 32, 103, 111, 116, 58, 32, 43, 10, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110,
		115, 32, 109, 97, 121, 32, 98, 101, 32, 97, 116, 32, 109, 111, 115, 116, 32, 50, 48, 32, 99, 104, 97, 114, 97,
		99, 116, 101, 114, 115, 33, 10, 94, 62, 118, 60, 0, 1, 0, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 36, 16, 0, 109, 4,
		1202, -3, 1, 586, 21001, 0, 0, -1, 22101, 1, -3, -3, 21102, 0, 1, -2, 2208, -2, -1, 570, 1005, 570, 617, 2201,
		-3, -2, 609, 4, 0, 21201, -2, 1, -2, 1105, 1, 597, 109, -4, 2105, 1, 0, 109, 5, 2102, 1, -4, 629, 21002, 0, 1,
		-2, 22101, 1, -4, -4, 21101, 0, 0, -3, 2208, -3, -2, 570, 1005, 570, 781, 2201, -4, -3, 652, 21001, 0, 0, -1,
		1208, -1, -4, 570, 1005, 570, 709, 1208, -1, -5, 570, 1005, 570, 734, 1207, -1, 0, 570, 1005, 570, 759, 1206,
		-1, 774, 1001, 578, 562, 684, 1, 0, 576, 576, 1001, 578, 566, 692, 1, 0, 577, 577, 21102, 1, 702, 0, 1106, 0,
		786, 21201, -1, -1, -1, 1106, 0, 676, 1001, 578, 1, 578, 1008, 578, 4, 570, 1006, 570, 724, 1001, 578, -4, 578,
		21102, 731, 1, 0, 1106, 0, 786, 1106, 0, 774, 1001, 578, -1, 578, 1008, 578, -1, 570, 1006, 570, 749, 1001,
		578, 4, 578, 21102, 756, 1, 0, 1106, 0, 786, 1105, 1, 774, 21202, -1, -11, 1, 22101, 1182, 1, 1, 21101, 0, 774,
		0, 1105, 1, 622, 21201, -3, 1, -3, 1106, 0, 640, 109, -5, 2106, 0, 0, 109, 7, 1005, 575, 802, 20102, 1, 576,
		-6, 20102, 1, 577, -5, 1105, 1, 814, 21101, 0, 0, -1, 21101, 0, 0, -5, 21101, 0, 0, -6, 20208, -6, 576, -2,
		208, -5, 577, 570, 22002, 570, -2, -2, 21202, -5, 45, -3, 22201, -6, -3, -3, 22101, 1449, -3, -3, 1201, -3, 0,
		843, 1005, 0, 863, 21202, -2, 42, -4, 22101, 46, -4, -4, 1206, -2, 924, 21102, 1, 1, -1, 1106, 0, 924, 1205,
		-2, 873, 21101, 0, 35, -4, 1106, 0, 924, 1201, -3, 0, 878, 1008, 0, 1, 570, 1006, 570, 916, 1001, 374, 1, 374,
		2102, 1, -3, 895, 1102, 2, 1, 0, 2102, 1, -3, 902, 1001, 438, 0, 438, 2202, -6, -5, 570, 1, 570, 374, 570, 1,
		570, 438, 438, 1001, 578, 558, 921, 21002, 0, 1, -4, 1006, 575, 959, 204, -4, 22101, 1, -6, -6, 1208, -6, 45,
		570, 1006, 570, 814, 104, 10, 22101, 1, -5, -5, 1208, -5, 47, 570, 1006, 570, 810, 104, 10, 1206, -1, 974, 99,
		1206, -1, 974, 1101, 0, 1, 575, 21101, 0, 973, 0, 1105, 1, 786, 99, 109, -7, 2106, 0, 0, 109, 6, 21102, 1, 0,
		-4, 21101, 0, 0, -3, 203, -2, 22101, 1, -3, -3, 21208, -2, 82, -1, 1205, -1, 1030, 21208, -2, 76, -1, 1205, -1,
		1037, 21207, -2, 48, -1, 1205, -1, 1124, 22107, 57, -2, -1, 1205, -1, 1124, 21201, -2, -48, -2, 1106, 0, 1041,
		21101, 0, -4, -2, 1105, 1, 1041, 21102, -5, 1, -2, 21201, -4, 1, -4, 21207, -4, 11, -1, 1206, -1, 1138, 2201,
		-5, -4, 1059, 1201, -2, 0, 0, 203, -2, 22101, 1, -3, -3, 21207, -2, 48, -1, 1205, -1, 1107, 22107, 57, -2, -1,
		1205, -1, 1107, 21201, -2, -48, -2, 2201, -5, -4, 1090, 20102, 10, 0, -1, 22201, -2, -1, -2, 2201, -5, -4,
		1103, 2101, 0, -2, 0, 1105, 1, 1060, 21208, -2, 10, -1, 1205, -1, 1162, 21208, -2, 44, -1, 1206, -1, 1131,
		1106, 0, 989, 21102, 1, 439, 1, 1105, 1, 1150, 21101, 0, 477, 1, 1105, 1, 1150, 21102, 1, 514, 1, 21101, 1149,
		0, 0, 1105, 1, 579, 99, 21102, 1157, 1, 0, 1106, 0, 579, 204, -2, 104, 10, 99, 21207, -3, 22, -1, 1206, -1,
		1138, 2101, 0, -5, 1176, 1201, -4, 0, 0, 109, -6, 2105, 1, 0, 36, 9, 36, 1, 7, 1, 36, 1, 7, 1, 36, 1, 7, 1, 24,
		7, 5, 1, 7, 1, 24, 1, 5, 1, 5, 1, 7, 1, 24, 1, 5, 1, 1, 13, 24, 1, 5, 1, 1, 1, 3, 1, 32, 1, 5, 1, 1, 1, 3, 7,
		26, 1, 5, 1, 1, 1, 9, 1, 26, 1, 5, 1, 1, 1, 9, 1, 26, 1, 5, 1, 1, 1, 9, 1, 26, 9, 9, 1, 32, 1, 11, 1, 32, 1,
		11, 1, 32, 1, 11, 1, 32, 7, 5, 1, 44, 1, 44, 1, 44, 1, 36, 9, 36, 1, 22, 9, 13, 1, 22, 1, 7, 1, 13, 1, 20, 13,
		9, 7, 16, 1, 1, 1, 7, 1, 1, 1, 9, 1, 1, 1, 3, 1, 10, 5, 1, 1, 1, 1, 7, 1, 1, 1, 9, 1, 1, 1, 3, 1, 10, 1, 3, 1,
		1, 1, 1, 1, 7, 1, 1, 1, 9, 1, 1, 1, 3, 1, 6, 13, 7, 1, 1, 1, 3, 5, 1, 1, 1, 5, 6, 1, 3, 1, 3, 1, 1, 1, 9, 1, 1,
		1, 3, 1, 3, 1, 1, 1, 12, 1, 3, 7, 9, 13, 12, 1, 7, 1, 13, 1, 3, 1, 3, 1, 14, 1, 7, 1, 13, 9, 14, 1, 7, 1, 17,
		1, 18, 9, 17, 7, 44, 1, 44, 1, 44, 1, 42, 9, 38, 1, 5, 1, 38, 1, 5, 1, 38, 1, 5, 1, 38, 1, 5, 1, 38, 1, 5, 1,
		38, 1, 5, 1, 38, 1, 5, 1, 38, 7, 6,
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

#[derive(Clone)]
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

#[derive(PartialEq)]
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

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

impl Coordinate {
	fn next_north(&self) -> Coordinate {
		Coordinate {
			x: self.x,
			y: self.y - 1,
		}
	}

	fn next_south(&self) -> Coordinate {
		Coordinate {
			x: self.x,
			y: self.y + 1,
		}
	}

	fn next_west(&self) -> Coordinate {
		Coordinate {
			x: self.x - 1,
			y: self.y,
		}
	}

	fn next_east(&self) -> Coordinate {
		Coordinate {
			x: self.x + 1,
			y: self.y,
		}
	}

	fn alignment_parameter(&self) -> u32 {
		self.x * self.y
	}
}

impl Default for Coordinate {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

fn main() {
	let mut program_state = ProgramState::new();
	let mut inputs: VecDeque<i64> = VecDeque::new();
	let mut outputs: Vec<i64> = Vec::new();
	execute_program(&mut program_state, &mut inputs, &mut outputs);

	let mut scaffold_coordinates: HashSet<Coordinate> = HashSet::new();
	let mut line: u32 = 0;
	let mut pos: u32 = 0;
	for place in outputs.iter() {
		let object = *place as u8 as char;
		if object == '\n' {
			line += 1;
			pos = 0;
			continue;
		}
		if object == '#' {
			scaffold_coordinates.insert(Coordinate { x: pos, y: line });
		}
		pos += 1;
	}

	let mut total_alignment_parameters: u32 = 0;
	for scaffold in scaffold_coordinates.iter() {
		if scaffold.x != 0
			&& scaffold.y != 0
			&& scaffold_coordinates.contains(&scaffold.next_north())
			&& scaffold_coordinates.contains(&scaffold.next_east())
			&& scaffold_coordinates.contains(&scaffold.next_south())
			&& scaffold_coordinates.contains(&scaffold.next_west())
		{
			total_alignment_parameters += scaffold.alignment_parameter();
		}
	}

	println!("{}", total_alignment_parameters);
}
