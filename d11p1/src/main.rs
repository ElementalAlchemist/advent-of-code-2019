use std::collections::{HashSet, VecDeque};

fn initial_program() -> Vec<i64> {
	vec![
		3,
		8,
		1005,
		8,
		335,
		1106,
		0,
		11,
		0,
		0,
		0,
		104,
		1,
		104,
		0,
		3,
		8,
		1002,
		8,
		-1,
		10,
		1001,
		10,
		1,
		10,
		4,
		10,
		108,
		0,
		8,
		10,
		4,
		10,
		102,
		1,
		8,
		28,
		3,
		8,
		1002,
		8,
		-1,
		10,
		1001,
		10,
		1,
		10,
		4,
		10,
		1008,
		8,
		1,
		10,
		4,
		10,
		101,
		0,
		8,
		51,
		1006,
		0,
		82,
		1006,
		0,
		56,
		1,
		1107,
		0,
		10,
		3,
		8,
		102,
		-1,
		8,
		10,
		101,
		1,
		10,
		10,
		4,
		10,
		1008,
		8,
		0,
		10,
		4,
		10,
		1001,
		8,
		0,
		83,
		3,
		8,
		1002,
		8,
		-1,
		10,
		101,
		1,
		10,
		10,
		4,
		10,
		108,
		1,
		8,
		10,
		4,
		10,
		101,
		0,
		8,
		104,
		1006,
		0,
		58,
		3,
		8,
		1002,
		8,
		-1,
		10,
		1001,
		10,
		1,
		10,
		4,
		10,
		108,
		0,
		8,
		10,
		4,
		10,
		1001,
		8,
		0,
		129,
		1006,
		0,
		54,
		1006,
		0,
		50,
		1006,
		0,
		31,
		3,
		8,
		1002,
		8,
		-1,
		10,
		1001,
		10,
		1,
		10,
		4,
		10,
		1008,
		8,
		1,
		10,
		4,
		10,
		102,
		1,
		8,
		161,
		2,
		101,
		14,
		10,
		1006,
		0,
		43,
		1006,
		0,
		77,
		3,
		8,
		102,
		-1,
		8,
		10,
		1001,
		10,
		1,
		10,
		4,
		10,
		1008,
		8,
		0,
		10,
		4,
		10,
		102,
		1,
		8,
		193,
		2,
		101,
		12,
		10,
		2,
		109,
		18,
		10,
		1,
		1009,
		13,
		10,
		3,
		8,
		102,
		-1,
		8,
		10,
		101,
		1,
		10,
		10,
		4,
		10,
		108,
		1,
		8,
		10,
		4,
		10,
		102,
		1,
		8,
		226,
		1,
		1103,
		1,
		10,
		1,
		1007,
		16,
		10,
		1,
		3,
		4,
		10,
		1006,
		0,
		88,
		3,
		8,
		102,
		-1,
		8,
		10,
		101,
		1,
		10,
		10,
		4,
		10,
		108,
		1,
		8,
		10,
		4,
		10,
		1001,
		8,
		0,
		263,
		1006,
		0,
		50,
		2,
		1108,
		17,
		10,
		1006,
		0,
		36,
		1,
		9,
		8,
		10,
		3,
		8,
		1002,
		8,
		-1,
		10,
		101,
		1,
		10,
		10,
		4,
		10,
		1008,
		8,
		0,
		10,
		4,
		10,
		1002,
		8,
		1,
		300,
		1006,
		0,
		22,
		2,
		106,
		2,
		10,
		2,
		1001,
		19,
		10,
		1,
		3,
		1,
		10,
		101,
		1,
		9,
		9,
		1007,
		9,
		925,
		10,
		1005,
		10,
		15,
		99,
		109,
		657,
		104,
		0,
		104,
		1,
		21101,
		0,
		937268454156,
		1,
		21102,
		1,
		352,
		0,
		1106,
		0,
		456,
		21101,
		0,
		666538713748,
		1,
		21102,
		363,
		1,
		0,
		1105,
		1,
		456,
		3,
		10,
		104,
		0,
		104,
		1,
		3,
		10,
		104,
		0,
		104,
		0,
		3,
		10,
		104,
		0,
		104,
		1,
		3,
		10,
		104,
		0,
		104,
		1,
		3,
		10,
		104,
		0,
		104,
		0,
		3,
		10,
		104,
		0,
		104,
		1,
		21101,
		3316845608,
		0,
		1,
		21102,
		1,
		410,
		0,
		1105,
		1,
		456,
		21101,
		0,
		209475103911,
		1,
		21101,
		421,
		0,
		0,
		1106,
		0,
		456,
		3,
		10,
		104,
		0,
		104,
		0,
		3,
		10,
		104,
		0,
		104,
		0,
		21101,
		0,
		984353603944,
		1,
		21101,
		444,
		0,
		0,
		1105,
		1,
		456,
		21102,
		1,
		988220752232,
		1,
		21102,
		1,
		455,
		0,
		1106,
		0,
		456,
		99,
		109,
		2,
		22101,
		0,
		-1,
		1,
		21102,
		40,
		1,
		2,
		21101,
		487,
		0,
		3,
		21101,
		0,
		477,
		0,
		1106,
		0,
		520,
		109,
		-2,
		2105,
		1,
		0,
		0,
		1,
		0,
		0,
		1,
		109,
		2,
		3,
		10,
		204,
		-1,
		1001,
		482,
		483,
		498,
		4,
		0,
		1001,
		482,
		1,
		482,
		108,
		4,
		482,
		10,
		1006,
		10,
		514,
		1102,
		0,
		1,
		482,
		109,
		-2,
		2105,
		1,
		0,
		0,
		109,
		4,
		2101,
		0,
		-1,
		519,
		1207,
		-3,
		0,
		10,
		1006,
		10,
		537,
		21101,
		0,
		0,
		-3,
		22101,
		0,
		-3,
		1,
		22101,
		0,
		-2,
		2,
		21102,
		1,
		1,
		3,
		21101,
		556,
		0,
		0,
		1106,
		0,
		561,
		109,
		-4,
		2106,
		0,
		0,
		109,
		5,
		1207,
		-3,
		1,
		10,
		1006,
		10,
		584,
		2207,
		-4,
		-2,
		10,
		1006,
		10,
		584,
		21201,
		-4,
		0,
		-4,
		1106,
		0,
		652,
		22101,
		0,
		-4,
		1,
		21201,
		-3,
		-1,
		2,
		21202,
		-2,
		2,
		3,
		21101,
		0,
		603,
		0,
		1105,
		1,
		561,
		22101,
		0,
		1,
		-4,
		21102,
		1,
		1,
		-1,
		2207,
		-4,
		-2,
		10,
		1006,
		10,
		622,
		21102,
		1,
		0,
		-1,
		22202,
		-2,
		-1,
		-2,
		2107,
		0,
		-3,
		10,
		1006,
		10,
		644,
		21201,
		-1,
		0,
		1,
		21101,
		644,
		0,
		0,
		105,
		1,
		519,
		21202,
		-2,
		-1,
		-2,
		22201,
		-4,
		-2,
		-4,
		109,
		-5,
		2106,
		0,
		0,
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

enum RobotDirection {
	Up,
	Right,
	Down,
	Left,
}

impl RobotDirection {
	fn turn_right(self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}

	fn turn_left(self) -> Self {
		match self {
			Self::Up => Self::Left,
			Self::Left => Self::Down,
			Self::Down => Self::Right,
			Self::Right => Self::Up,
		}
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn move_coord(&mut self, direction: &RobotDirection) {
		match direction {
			RobotDirection::Up => self.y += 1,
			RobotDirection::Down => self.y -= 1,
			RobotDirection::Right => self.x += 1,
			RobotDirection::Left => self.x -= 1,
		}
	}
}

impl Default for Coordinate {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

fn main() {
	let mut input: VecDeque<i64> = VecDeque::new();
	input.push_back(0);
	let mut painted_tiles: HashSet<Coordinate> = HashSet::new();
	let mut white_tiles: HashSet<Coordinate> = HashSet::new();
	let mut current_coord: Coordinate = Coordinate::default();
	let mut current_direction: RobotDirection = RobotDirection::Up;
	let mut output: Vec<i64> = Vec::new();
	let mut program_state = ProgramState::new();

	loop {
		let status = execute_program(&mut program_state, &mut input, &mut output);
		let mut output_iter = output.iter();
		while let (Some(color), Some(turn)) = (output_iter.next(), output_iter.next()) {
			if *color == 0 && white_tiles.contains(&current_coord) {
				white_tiles.remove(&current_coord);
				painted_tiles.insert(current_coord.clone());
			} else if *color == 1 && !white_tiles.contains(&current_coord) {
				white_tiles.insert(current_coord.clone());
				painted_tiles.insert(current_coord.clone());
			}

			if *turn == 1 {
				current_direction = current_direction.turn_right();
			} else {
				current_direction = current_direction.turn_left();
			}
		}
		output.clear();
		current_coord.move_coord(&current_direction);
		if status != RunStatus::WaitingForInput {
			break;
		}
		if white_tiles.contains(&current_coord) {
			input.push_back(1);
		} else {
			input.push_back(0);
		}
	}
	println!("{}", painted_tiles.len());
}
