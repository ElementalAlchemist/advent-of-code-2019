use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

fn initial_program() -> Vec<i64> {
	vec![
		109, 2050, 21102, 1, 966, 1, 21102, 1, 13, 0, 1106, 0, 1378, 21101, 0, 20, 0, 1106, 0, 1337, 21101, 0, 27, 0,
		1106, 0, 1279, 1208, 1, 65, 748, 1005, 748, 73, 1208, 1, 79, 748, 1005, 748, 110, 1208, 1, 78, 748, 1005, 748,
		132, 1208, 1, 87, 748, 1005, 748, 169, 1208, 1, 82, 748, 1005, 748, 239, 21101, 0, 1041, 1, 21102, 1, 73, 0,
		1106, 0, 1421, 21102, 78, 1, 1, 21101, 0, 1041, 2, 21102, 1, 88, 0, 1106, 0, 1301, 21101, 0, 68, 1, 21101,
		1041, 0, 2, 21101, 103, 0, 0, 1106, 0, 1301, 1102, 1, 1, 750, 1105, 1, 298, 21102, 1, 82, 1, 21102, 1041, 1, 2,
		21101, 0, 125, 0, 1105, 1, 1301, 1102, 1, 2, 750, 1105, 1, 298, 21102, 79, 1, 1, 21102, 1041, 1, 2, 21101, 147,
		0, 0, 1106, 0, 1301, 21101, 84, 0, 1, 21101, 0, 1041, 2, 21102, 162, 1, 0, 1106, 0, 1301, 1102, 1, 3, 750,
		1105, 1, 298, 21102, 65, 1, 1, 21102, 1041, 1, 2, 21101, 184, 0, 0, 1105, 1, 1301, 21102, 1, 76, 1, 21101,
		1041, 0, 2, 21102, 199, 1, 0, 1106, 0, 1301, 21102, 1, 75, 1, 21101, 0, 1041, 2, 21101, 0, 214, 0, 1105, 1,
		1301, 21102, 1, 221, 0, 1105, 1, 1337, 21101, 10, 0, 1, 21101, 0, 1041, 2, 21101, 0, 236, 0, 1106, 0, 1301,
		1106, 0, 553, 21101, 0, 85, 1, 21101, 1041, 0, 2, 21102, 1, 254, 0, 1106, 0, 1301, 21102, 78, 1, 1, 21102, 1,
		1041, 2, 21101, 0, 269, 0, 1105, 1, 1301, 21102, 276, 1, 0, 1106, 0, 1337, 21102, 10, 1, 1, 21102, 1, 1041, 2,
		21102, 291, 1, 0, 1106, 0, 1301, 1101, 1, 0, 755, 1106, 0, 553, 21101, 32, 0, 1, 21101, 0, 1041, 2, 21102, 1,
		313, 0, 1105, 1, 1301, 21101, 0, 320, 0, 1105, 1, 1337, 21101, 327, 0, 0, 1106, 0, 1279, 1202, 1, 1, 749,
		21102, 65, 1, 2, 21102, 73, 1, 3, 21102, 346, 1, 0, 1105, 1, 1889, 1206, 1, 367, 1007, 749, 69, 748, 1005, 748,
		360, 1101, 0, 1, 756, 1001, 749, -64, 751, 1105, 1, 406, 1008, 749, 74, 748, 1006, 748, 381, 1101, -1, 0, 751,
		1106, 0, 406, 1008, 749, 84, 748, 1006, 748, 395, 1102, 1, -2, 751, 1105, 1, 406, 21101, 0, 1100, 1, 21102, 1,
		406, 0, 1105, 1, 1421, 21102, 1, 32, 1, 21102, 1100, 1, 2, 21101, 421, 0, 0, 1106, 0, 1301, 21101, 428, 0, 0,
		1106, 0, 1337, 21102, 435, 1, 0, 1106, 0, 1279, 1202, 1, 1, 749, 1008, 749, 74, 748, 1006, 748, 453, 1101, 0,
		-1, 752, 1105, 1, 478, 1008, 749, 84, 748, 1006, 748, 467, 1102, 1, -2, 752, 1106, 0, 478, 21101, 0, 1168, 1,
		21101, 478, 0, 0, 1106, 0, 1421, 21101, 0, 485, 0, 1106, 0, 1337, 21102, 10, 1, 1, 21102, 1, 1168, 2, 21102,
		500, 1, 0, 1105, 1, 1301, 1007, 920, 15, 748, 1005, 748, 518, 21102, 1, 1209, 1, 21101, 518, 0, 0, 1105, 1,
		1421, 1002, 920, 3, 529, 1001, 529, 921, 529, 101, 0, 750, 0, 1001, 529, 1, 537, 102, 1, 751, 0, 1001, 537, 1,
		545, 1002, 752, 1, 0, 1001, 920, 1, 920, 1105, 1, 13, 1005, 755, 577, 1006, 756, 570, 21102, 1100, 1, 1, 21102,
		1, 570, 0, 1106, 0, 1421, 21101, 0, 987, 1, 1105, 1, 581, 21101, 0, 1001, 1, 21101, 588, 0, 0, 1106, 0, 1378,
		1101, 758, 0, 593, 1001, 0, 0, 753, 1006, 753, 654, 20101, 0, 753, 1, 21101, 610, 0, 0, 1106, 0, 667, 21102, 1,
		0, 1, 21101, 621, 0, 0, 1105, 1, 1463, 1205, 1, 647, 21102, 1015, 1, 1, 21101, 0, 635, 0, 1106, 0, 1378, 21102,
		1, 1, 1, 21102, 1, 646, 0, 1105, 1, 1463, 99, 1001, 593, 1, 593, 1106, 0, 592, 1006, 755, 664, 1102, 0, 1, 755,
		1106, 0, 647, 4, 754, 99, 109, 2, 1101, 0, 726, 757, 22101, 0, -1, 1, 21102, 9, 1, 2, 21101, 697, 0, 3, 21101,
		0, 692, 0, 1106, 0, 1913, 109, -2, 2106, 0, 0, 109, 2, 101, 0, 757, 706, 1201, -1, 0, 0, 1001, 757, 1, 757,
		109, -2, 2106, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 63, 191, 223, 127, 159, 95, 0, 170, 122, 102, 166, 184, 142, 77, 113,
		215, 202, 162, 219, 154, 231, 92, 123, 197, 247, 85, 87, 138, 181, 174, 55, 185, 153, 234, 107, 254, 124, 249,
		84, 139, 233, 213, 226, 115, 246, 228, 99, 34, 121, 118, 163, 137, 54, 169, 57, 106, 46, 38, 125, 189, 119, 51,
		103, 175, 79, 157, 61, 207, 221, 71, 251, 238, 178, 86, 78, 232, 47, 187, 203, 253, 248, 136, 94, 69, 201, 190,
		205, 117, 227, 156, 98, 177, 42, 100, 204, 114, 200, 188, 76, 35, 49, 252, 236, 196, 206, 70, 143, 182, 62,
		198, 168, 229, 68, 93, 230, 60, 183, 243, 199, 237, 43, 140, 244, 155, 222, 126, 241, 216, 214, 58, 218, 217,
		50, 239, 141, 56, 158, 171, 109, 235, 245, 179, 152, 101, 59, 116, 108, 212, 250, 120, 111, 173, 172, 39, 242,
		220, 167, 186, 53, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 73, 110, 112, 117, 116, 32, 105, 110, 115, 116, 114,
		117, 99, 116, 105, 111, 110, 115, 58, 10, 13, 10, 87, 97, 108, 107, 105, 110, 103, 46, 46, 46, 10, 10, 13, 10,
		82, 117, 110, 110, 105, 110, 103, 46, 46, 46, 10, 10, 25, 10, 68, 105, 100, 110, 39, 116, 32, 109, 97, 107,
		101, 32, 105, 116, 32, 97, 99, 114, 111, 115, 115, 58, 10, 10, 58, 73, 110, 118, 97, 108, 105, 100, 32, 111,
		112, 101, 114, 97, 116, 105, 111, 110, 59, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 115, 111, 109, 101,
		116, 104, 105, 110, 103, 32, 108, 105, 107, 101, 32, 65, 78, 68, 44, 32, 79, 82, 44, 32, 111, 114, 32, 78, 79,
		84, 67, 73, 110, 118, 97, 108, 105, 100, 32, 102, 105, 114, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110,
		116, 59, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 108,
		105, 107, 101, 32, 65, 44, 32, 66, 44, 32, 67, 44, 32, 68, 44, 32, 74, 44, 32, 111, 114, 32, 84, 40, 73, 110,
		118, 97, 108, 105, 100, 32, 115, 101, 99, 111, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 59, 32,
		101, 120, 112, 101, 99, 116, 101, 100, 32, 74, 32, 111, 114, 32, 84, 52, 79, 117, 116, 32, 111, 102, 32, 109,
		101, 109, 111, 114, 121, 59, 32, 97, 116, 32, 109, 111, 115, 116, 32, 49, 53, 32, 105, 110, 115, 116, 114, 117,
		99, 116, 105, 111, 110, 115, 32, 99, 97, 110, 32, 98, 101, 32, 115, 116, 111, 114, 101, 100, 0, 109, 1, 1005,
		1262, 1270, 3, 1262, 20102, 1, 1262, 0, 109, -1, 2105, 1, 0, 109, 1, 21102, 1288, 1, 0, 1106, 0, 1263, 20102,
		1, 1262, 0, 1101, 0, 0, 1262, 109, -1, 2106, 0, 0, 109, 5, 21101, 0, 1310, 0, 1106, 0, 1279, 22102, 1, 1, -2,
		22208, -2, -4, -1, 1205, -1, 1332, 21202, -3, 1, 1, 21101, 0, 1332, 0, 1106, 0, 1421, 109, -5, 2105, 1, 0, 109,
		2, 21101, 1346, 0, 0, 1105, 1, 1263, 21208, 1, 32, -1, 1205, -1, 1363, 21208, 1, 9, -1, 1205, -1, 1363, 1105,
		1, 1373, 21102, 1370, 1, 0, 1105, 1, 1279, 1105, 1, 1339, 109, -2, 2105, 1, 0, 109, 5, 1202, -4, 1, 1385,
		21002, 0, 1, -2, 22101, 1, -4, -4, 21102, 0, 1, -3, 22208, -3, -2, -1, 1205, -1, 1416, 2201, -4, -3, 1408, 4,
		0, 21201, -3, 1, -3, 1105, 1, 1396, 109, -5, 2105, 1, 0, 109, 2, 104, 10, 21202, -1, 1, 1, 21102, 1436, 1, 0,
		1106, 0, 1378, 104, 10, 99, 109, -2, 2105, 1, 0, 109, 3, 20002, 593, 753, -1, 22202, -1, -2, -1, 201, -1, 754,
		754, 109, -3, 2106, 0, 0, 109, 10, 21101, 0, 5, -5, 21102, 1, 1, -4, 21101, 0, 0, -3, 1206, -9, 1555, 21101, 3,
		0, -6, 21102, 1, 5, -7, 22208, -7, -5, -8, 1206, -8, 1507, 22208, -6, -4, -8, 1206, -8, 1507, 104, 64, 1106, 0,
		1529, 1205, -6, 1527, 1201, -7, 716, 1515, 21002, 0, -11, -8, 21201, -8, 46, -8, 204, -8, 1105, 1, 1529, 104,
		46, 21201, -7, 1, -7, 21207, -7, 22, -8, 1205, -8, 1488, 104, 10, 21201, -6, -1, -6, 21207, -6, 0, -8, 1206,
		-8, 1484, 104, 10, 21207, -4, 1, -8, 1206, -8, 1569, 21101, 0, 0, -9, 1105, 1, 1689, 21208, -5, 21, -8, 1206,
		-8, 1583, 21101, 1, 0, -9, 1105, 1, 1689, 1201, -5, 716, 1588, 21001, 0, 0, -2, 21208, -4, 1, -1, 22202, -2,
		-1, -1, 1205, -2, 1613, 22102, 1, -5, 1, 21101, 0, 1613, 0, 1105, 1, 1444, 1206, -1, 1634, 21202, -5, 1, 1,
		21102, 1, 1627, 0, 1106, 0, 1694, 1206, 1, 1634, 21102, 2, 1, -3, 22107, 1, -4, -8, 22201, -1, -8, -8, 1206,
		-8, 1649, 21201, -5, 1, -5, 1206, -3, 1663, 21201, -3, -1, -3, 21201, -4, 1, -4, 1105, 1, 1667, 21201, -4, -1,
		-4, 21208, -4, 0, -1, 1201, -5, 716, 1676, 22002, 0, -1, -1, 1206, -1, 1686, 21101, 0, 1, -4, 1105, 1, 1477,
		109, -10, 2106, 0, 0, 109, 11, 21101, 0, 0, -6, 21101, 0, 0, -8, 21102, 0, 1, -7, 20208, -6, 920, -9, 1205, -9,
		1880, 21202, -6, 3, -9, 1201, -9, 921, 1724, 21002, 0, 1, -5, 1001, 1724, 1, 1733, 20101, 0, 0, -4, 22101, 0,
		-4, 1, 21101, 0, 1, 2, 21102, 1, 9, 3, 21102, 1754, 1, 0, 1105, 1, 1889, 1206, 1, 1772, 2201, -10, -4, 1766,
		1001, 1766, 716, 1766, 21001, 0, 0, -3, 1106, 0, 1790, 21208, -4, -1, -9, 1206, -9, 1786, 22101, 0, -8, -3,
		1105, 1, 1790, 21202, -7, 1, -3, 1001, 1733, 1, 1796, 20102, 1, 0, -2, 21208, -2, -1, -9, 1206, -9, 1812,
		22102, 1, -8, -1, 1106, 0, 1816, 22102, 1, -7, -1, 21208, -5, 1, -9, 1205, -9, 1837, 21208, -5, 2, -9, 1205,
		-9, 1844, 21208, -3, 0, -1, 1106, 0, 1855, 22202, -3, -1, -1, 1106, 0, 1855, 22201, -3, -1, -1, 22107, 0, -1,
		-1, 1105, 1, 1855, 21208, -2, -1, -9, 1206, -9, 1869, 22102, 1, -1, -8, 1105, 1, 1873, 21201, -1, 0, -7, 21201,
		-6, 1, -6, 1105, 1, 1708, 22101, 0, -8, -10, 109, -11, 2106, 0, 0, 109, 7, 22207, -6, -5, -3, 22207, -4, -6,
		-2, 22201, -3, -2, -1, 21208, -1, 0, -6, 109, -7, 2106, 0, 0, 0, 109, 5, 1202, -2, 1, 1912, 21207, -4, 0, -1,
		1206, -1, 1930, 21101, 0, 0, -4, 21201, -4, 0, 1, 22102, 1, -3, 2, 21102, 1, 1, 3, 21102, 1, 1949, 0, 1106, 0,
		1954, 109, -5, 2106, 0, 0, 109, 6, 21207, -4, 1, -1, 1206, -1, 1977, 22207, -5, -3, -1, 1206, -1, 1977, 22102,
		1, -5, -5, 1106, 0, 2045, 21201, -5, 0, 1, 21201, -4, -1, 2, 21202, -3, 2, 3, 21102, 1, 1996, 0, 1106, 0, 1954,
		21201, 1, 0, -5, 21101, 0, 1, -2, 22207, -5, -3, -1, 1206, -1, 2015, 21102, 0, 1, -2, 22202, -3, -2, -3, 22107,
		0, -4, -1, 1206, -1, 2037, 21202, -2, 1, 1, 21101, 0, 2037, 0, 105, 1, 1912, 21202, -3, -1, -3, 22201, -5, -3,
		-5, 109, -6, 2105, 1, 0,
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

#[derive(Debug, PartialEq)]
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
	let mut program_state = ProgramState::new();
	let mut inputs: VecDeque<i64> = VecDeque::new();
	let mut outputs: Vec<i64> = Vec::new();

	loop {
		let program_status = execute_program(&mut program_state, &mut inputs, &mut outputs);

		if program_status == RunStatus::Complete {
			let mut big_output: Option<i64> = None;
			for output_val in outputs.iter() {
				if *output_val >= 128 {
					big_output = Some(*output_val);
				} else {
					print!("{}", *output_val as u8 as char);
				}
			}
			if let Some(val) = big_output {
				println!("Damage: {}", val);
			}
			break;
		}

		if !outputs.is_empty() {
			for output_val in outputs.iter() {
				print!("{}", *output_val as u8 as char);
			}
			outputs.clear();
			stdout().flush().expect("Something exploded.");
		}
		let mut text_input = String::new();
		stdin().read_line(&mut text_input).expect("Something exploded.");
		for input_char in text_input.chars() {
			inputs.push_back(input_char as u8 as i64);
		}
	}
	/*
	Instructions applied successfully at user input:
	OR A T
	AND B T
	AND C T
	NOT T J
	AND D J
	WALK
	These instructions instruct the drone to jump any time there's a hole
	at A, B, or C, but not a hole at D. It always jumps 4 spaces (to D), so
	this ensures it should make it across.
	*/
}
