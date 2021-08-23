use std::collections::{HashMap, VecDeque};

fn initial_program() -> Vec<i64> {
	vec![
		1, 380, 379, 385, 1008, 2267, 709926, 381, 1005, 381, 12, 99, 109, 2268, 1101, 0, 0, 383, 1101, 0, 0, 382,
		20101, 0, 382, 1, 21002, 383, 1, 2, 21102, 37, 1, 0, 1105, 1, 578, 4, 382, 4, 383, 204, 1, 1001, 382, 1, 382,
		1007, 382, 37, 381, 1005, 381, 22, 1001, 383, 1, 383, 1007, 383, 22, 381, 1005, 381, 18, 1006, 385, 69, 99,
		104, -1, 104, 0, 4, 386, 3, 384, 1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108, 1105, 1,
		161, 107, 1, 392, 381, 1006, 381, 161, 1101, -1, 0, 384, 1106, 0, 119, 1007, 392, 35, 381, 1006, 381, 161,
		1101, 0, 1, 384, 21002, 392, 1, 1, 21102, 1, 20, 2, 21102, 1, 0, 3, 21101, 138, 0, 0, 1106, 0, 549, 1, 392,
		384, 392, 21001, 392, 0, 1, 21102, 20, 1, 2, 21101, 3, 0, 3, 21101, 0, 161, 0, 1106, 0, 549, 1101, 0, 0, 384,
		20001, 388, 390, 1, 21001, 389, 0, 2, 21102, 180, 1, 0, 1105, 1, 578, 1206, 1, 213, 1208, 1, 2, 381, 1006, 381,
		205, 20001, 388, 390, 1, 20101, 0, 389, 2, 21102, 1, 205, 0, 1106, 0, 393, 1002, 390, -1, 390, 1102, 1, 1, 384,
		20102, 1, 388, 1, 20001, 389, 391, 2, 21101, 0, 228, 0, 1105, 1, 578, 1206, 1, 261, 1208, 1, 2, 381, 1006, 381,
		253, 20101, 0, 388, 1, 20001, 389, 391, 2, 21102, 253, 1, 0, 1106, 0, 393, 1002, 391, -1, 391, 1101, 0, 1, 384,
		1005, 384, 161, 20001, 388, 390, 1, 20001, 389, 391, 2, 21102, 279, 1, 0, 1106, 0, 578, 1206, 1, 316, 1208, 1,
		2, 381, 1006, 381, 304, 20001, 388, 390, 1, 20001, 389, 391, 2, 21102, 1, 304, 0, 1105, 1, 393, 1002, 390, -1,
		390, 1002, 391, -1, 391, 1102, 1, 1, 384, 1005, 384, 161, 20102, 1, 388, 1, 20101, 0, 389, 2, 21101, 0, 0, 3,
		21102, 338, 1, 0, 1105, 1, 549, 1, 388, 390, 388, 1, 389, 391, 389, 20101, 0, 388, 1, 20101, 0, 389, 2, 21101,
		4, 0, 3, 21101, 0, 365, 0, 1106, 0, 549, 1007, 389, 21, 381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1,
		0, 0, 0, 0, 0, 0, 247, 16, 17, 1, 1, 18, 109, 3, 22102, 1, -2, 1, 22102, 1, -1, 2, 21102, 0, 1, 3, 21101, 414,
		0, 0, 1105, 1, 549, 22101, 0, -2, 1, 21202, -1, 1, 2, 21101, 429, 0, 0, 1105, 1, 601, 1202, 1, 1, 435, 1, 386,
		0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005, 387, 451, 99, 109, -3, 2106, 0, 0, 109, 8, 22202,
		-7, -6, -3, 22201, -3, -5, -3, 21202, -4, 64, -2, 2207, -3, -2, 381, 1005, 381, 492, 21202, -2, -1, -1, 22201,
		-3, -1, -3, 2207, -3, -2, 381, 1006, 381, 481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518, 21202, -2,
		-1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4, 381, 1005, 381, 540, 21202, -4, -1,
		-1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006, 381, 529, 22101, 0, -3, -7, 109, -8, 2106, 0, 0, 109, 4, 1202,
		-2, 37, 566, 201, -3, 566, 566, 101, 639, 566, 566, 1202, -1, 1, 0, 204, -3, 204, -2, 204, -1, 109, -4, 2106,
		0, 0, 109, 3, 1202, -1, 37, 593, 201, -2, 593, 593, 101, 639, 593, 593, 21001, 0, 0, -2, 109, -3, 2106, 0, 0,
		109, 3, 22102, 22, -2, 1, 22201, 1, -1, 1, 21102, 1, 409, 2, 21101, 34, 0, 3, 21102, 814, 1, 4, 21102, 1, 630,
		0, 1106, 0, 456, 21201, 1, 1453, -2, 109, -3, 2106, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 2, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 2,
		2, 2, 2, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 0, 2, 0, 2,
		2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 2, 0, 0, 2, 0, 0, 1, 1, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0,
		2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 1, 1, 0, 2, 2, 0, 0, 0, 2, 2, 0, 0, 2, 2, 2, 0, 2, 2, 0,
		0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 2, 2, 0, 2, 2, 0, 1, 1, 0, 2, 0, 2, 2, 0, 2, 0, 2, 2, 2, 0, 0, 2, 0, 0, 0,
		0, 2, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 1, 1, 0, 2, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 0, 0, 1, 1, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2,
		2, 2, 2, 2, 0, 2, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 2, 0, 1, 1, 0, 2, 0, 2, 0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 2, 2, 2,
		2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 0, 1, 1, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 0, 2,
		2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 2, 2, 0, 1, 1, 0, 2, 0, 2, 2, 2, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 0,
		2, 2, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 0, 1, 1, 0, 2, 2, 2, 0, 0, 2, 0, 2, 0, 2, 2, 0, 0, 2, 0, 0,
		0, 2, 2, 2, 0, 2, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2,
		0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 0, 1, 1, 0, 0, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 0, 0,
		2, 0, 0, 0, 2, 2, 2, 0, 0, 2, 2, 0, 2, 2, 0, 0, 2, 0, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2,
		2, 2, 2, 2, 2, 2, 0, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 98, 79, 75, 78, 5, 74, 79, 17, 41, 95, 84, 78, 77, 91,
		76, 42, 52, 12, 91, 69, 32, 17, 81, 63, 98, 80, 66, 62, 50, 63, 55, 28, 34, 41, 10, 81, 2, 15, 67, 29, 97, 23,
		64, 56, 30, 22, 44, 49, 59, 26, 2, 31, 20, 74, 70, 46, 20, 82, 27, 48, 34, 90, 23, 84, 90, 26, 89, 43, 23, 31,
		1, 63, 76, 26, 95, 91, 18, 31, 81, 40, 80, 50, 69, 70, 38, 26, 5, 31, 58, 52, 19, 7, 62, 16, 10, 52, 74, 64,
		85, 47, 55, 98, 31, 27, 9, 74, 87, 65, 50, 93, 58, 12, 47, 8, 84, 34, 88, 41, 72, 81, 74, 76, 17, 41, 86, 60,
		70, 52, 95, 38, 40, 27, 49, 60, 14, 31, 81, 95, 89, 12, 89, 67, 32, 43, 78, 90, 2, 19, 19, 25, 96, 62, 57, 17,
		43, 52, 22, 25, 85, 92, 82, 54, 87, 22, 20, 31, 59, 39, 59, 50, 80, 91, 76, 18, 94, 90, 5, 7, 65, 3, 8, 87, 65,
		67, 86, 71, 97, 26, 53, 90, 14, 57, 51, 54, 25, 78, 29, 51, 88, 54, 24, 31, 51, 61, 92, 92, 4, 59, 93, 43, 76,
		94, 93, 67, 20, 36, 77, 85, 18, 72, 40, 98, 55, 14, 60, 34, 37, 6, 88, 67, 37, 12, 3, 63, 56, 30, 7, 97, 77,
		72, 14, 82, 42, 30, 61, 5, 34, 87, 87, 62, 59, 82, 11, 64, 49, 1, 71, 66, 94, 15, 81, 25, 72, 50, 45, 5, 54,
		79, 45, 66, 88, 71, 59, 53, 81, 44, 23, 36, 34, 15, 26, 36, 38, 14, 82, 69, 68, 7, 77, 62, 28, 18, 13, 85, 49,
		78, 24, 89, 11, 87, 32, 65, 45, 6, 23, 55, 94, 41, 91, 95, 7, 87, 6, 27, 74, 31, 68, 36, 26, 38, 86, 17, 41,
		63, 21, 26, 98, 37, 89, 65, 1, 75, 52, 65, 59, 91, 17, 43, 63, 7, 33, 6, 95, 81, 29, 52, 77, 82, 35, 66, 94,
		60, 56, 9, 34, 32, 22, 61, 39, 43, 11, 2, 80, 68, 34, 59, 73, 67, 36, 26, 60, 33, 8, 96, 7, 26, 5, 90, 47, 51,
		34, 15, 6, 29, 28, 70, 30, 14, 53, 5, 45, 75, 62, 86, 96, 51, 87, 25, 30, 48, 51, 27, 11, 80, 84, 20, 33, 16,
		7, 20, 55, 29, 27, 10, 67, 80, 13, 32, 32, 52, 56, 64, 21, 5, 62, 43, 32, 45, 4, 39, 57, 82, 50, 11, 81, 64,
		64, 10, 16, 79, 57, 59, 71, 27, 61, 68, 39, 79, 1, 9, 65, 54, 96, 30, 66, 23, 56, 98, 25, 45, 23, 25, 96, 36,
		81, 68, 36, 3, 29, 16, 67, 52, 83, 11, 28, 33, 78, 45, 43, 32, 27, 97, 71, 54, 34, 12, 11, 10, 1, 29, 35, 48,
		34, 57, 70, 2, 61, 30, 70, 22, 47, 27, 26, 17, 65, 61, 31, 63, 54, 16, 88, 32, 2, 88, 47, 94, 66, 4, 79, 39, 7,
		44, 94, 75, 48, 69, 93, 76, 49, 23, 92, 79, 74, 43, 43, 53, 92, 2, 71, 64, 8, 96, 58, 5, 76, 4, 69, 12, 21, 36,
		10, 96, 28, 77, 92, 77, 52, 54, 69, 10, 3, 16, 53, 21, 16, 69, 97, 71, 92, 64, 6, 47, 37, 43, 66, 63, 32, 34,
		45, 23, 79, 92, 19, 88, 18, 16, 90, 28, 63, 13, 14, 81, 67, 10, 97, 38, 66, 41, 1, 25, 2, 92, 76, 56, 46, 70,
		4, 92, 84, 38, 50, 91, 37, 22, 7, 78, 85, 60, 45, 53, 17, 30, 1, 51, 64, 64, 29, 48, 49, 5, 14, 29, 66, 2, 31,
		67, 85, 10, 14, 72, 52, 93, 74, 38, 43, 73, 38, 81, 18, 24, 11, 52, 58, 71, 45, 49, 74, 72, 89, 68, 77, 9, 41,
		54, 41, 62, 29, 58, 29, 68, 78, 16, 40, 52, 92, 18, 73, 42, 38, 6, 62, 96, 18, 10, 44, 18, 35, 28, 43, 17, 95,
		43, 59, 87, 27, 94, 80, 65, 58, 78, 35, 78, 96, 58, 22, 22, 89, 38, 44, 89, 95, 48, 78, 36, 79, 89, 46, 91, 86,
		88, 41, 77, 94, 75, 91, 33, 60, 34, 59, 8, 13, 17, 93, 51, 86, 21, 32, 67, 49, 64, 38, 73, 60, 12, 97, 60, 59,
		72, 20, 46, 10, 41, 70, 80, 12, 78, 10, 86, 17, 93, 69, 21, 67, 30, 33, 39, 55, 98, 92, 31, 69, 44, 27, 20, 76,
		88, 66, 54, 83, 62, 3, 6, 66, 65, 85, 87, 30, 26, 17, 13, 54, 15, 66, 40, 93, 94, 47, 22, 2, 3, 53, 81, 57, 68,
		10, 10, 85, 30, 39, 29, 24, 86, 24, 4, 709926,
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
	x: i64,
	y: i64,
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Self {
		Coordinate { x, y }
	}
}

impl Default for Coordinate {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

#[derive(Eq, PartialEq)]
enum TileType {
	Empty,
	Wall,
	Block,
	Paddle,
	Ball,
}

impl Default for TileType {
	fn default() -> Self {
		Self::Empty
	}
}

fn main() {
	let mut input: VecDeque<i64> = VecDeque::new();
	let mut output: Vec<i64> = Vec::new();
	let mut program_state = ProgramState::new();
	let mut tiles: HashMap<Coordinate, TileType> = HashMap::new();

	execute_program(&mut program_state, &mut input, &mut output);

	if output.len() % 3 != 0 {
		panic!("Expected multiple of 3 outputs");
	}

	let mut output_iter = output.iter();
	loop {
		let x = output_iter.next();
		let x = if let Some(x) = x {
			x
		} else {
			break;
		};
		let y = output_iter.next().unwrap();
		let type_code = output_iter.next().unwrap();

		let coord = Coordinate::new(*x, *y);
		let tile_type = match type_code {
			0 => TileType::Empty,
			1 => TileType::Wall,
			2 => TileType::Block,
			3 => TileType::Paddle,
			4 => TileType::Ball,
			_ => panic!("Invalid tile type code {}", type_code),
		};

		tiles.insert(coord, tile_type);
	}

	let block_tiles = tiles.values().filter(|t| **t == TileType::Block).count();
	println!("{}", block_tiles);
}
