use std::collections::{HashMap, HashSet, VecDeque};

fn initial_program() -> Vec<i64> {
	vec![
		3, 1033, 1008, 1033, 1, 1032, 1005, 1032, 31, 1008, 1033, 2, 1032, 1005, 1032, 58, 1008, 1033, 3, 1032, 1005,
		1032, 81, 1008, 1033, 4, 1032, 1005, 1032, 104, 99, 1002, 1034, 1, 1039, 102, 1, 1036, 1041, 1001, 1035, -1,
		1040, 1008, 1038, 0, 1043, 102, -1, 1043, 1032, 1, 1037, 1032, 1042, 1105, 1, 124, 101, 0, 1034, 1039, 102, 1,
		1036, 1041, 1001, 1035, 1, 1040, 1008, 1038, 0, 1043, 1, 1037, 1038, 1042, 1106, 0, 124, 1001, 1034, -1, 1039,
		1008, 1036, 0, 1041, 1002, 1035, 1, 1040, 1001, 1038, 0, 1043, 101, 0, 1037, 1042, 1106, 0, 124, 1001, 1034, 1,
		1039, 1008, 1036, 0, 1041, 101, 0, 1035, 1040, 102, 1, 1038, 1043, 1002, 1037, 1, 1042, 1006, 1039, 217, 1006,
		1040, 217, 1008, 1039, 40, 1032, 1005, 1032, 217, 1008, 1040, 40, 1032, 1005, 1032, 217, 1008, 1039, 35, 1032,
		1006, 1032, 165, 1008, 1040, 9, 1032, 1006, 1032, 165, 1101, 0, 2, 1044, 1105, 1, 224, 2, 1041, 1043, 1032,
		1006, 1032, 179, 1102, 1, 1, 1044, 1105, 1, 224, 1, 1041, 1043, 1032, 1006, 1032, 217, 1, 1042, 1043, 1032,
		1001, 1032, -1, 1032, 1002, 1032, 39, 1032, 1, 1032, 1039, 1032, 101, -1, 1032, 1032, 101, 252, 1032, 211,
		1007, 0, 26, 1044, 1105, 1, 224, 1101, 0, 0, 1044, 1106, 0, 224, 1006, 1044, 247, 102, 1, 1039, 1034, 101, 0,
		1040, 1035, 102, 1, 1041, 1036, 1002, 1043, 1, 1038, 1001, 1042, 0, 1037, 4, 1044, 1106, 0, 0, 22, 11, 19, 72,
		14, 9, 6, 73, 82, 17, 41, 18, 83, 18, 49, 19, 12, 14, 39, 17, 20, 69, 20, 12, 48, 8, 8, 59, 36, 7, 33, 1, 15,
		13, 10, 46, 96, 15, 2, 22, 80, 99, 12, 68, 99, 79, 22, 84, 16, 45, 25, 51, 4, 20, 95, 4, 51, 43, 13, 89, 2, 91,
		48, 2, 46, 55, 24, 84, 8, 88, 10, 98, 46, 57, 15, 27, 7, 1, 19, 20, 63, 24, 50, 13, 63, 13, 59, 19, 13, 53, 75,
		8, 20, 8, 44, 44, 21, 5, 11, 76, 9, 21, 2, 11, 27, 61, 6, 12, 72, 22, 40, 11, 9, 50, 18, 2, 38, 21, 78, 18, 13,
		99, 9, 74, 5, 22, 30, 35, 5, 16, 34, 91, 55, 4, 19, 28, 42, 21, 62, 12, 74, 94, 16, 40, 2, 95, 54, 21, 2, 23,
		56, 34, 9, 49, 47, 14, 39, 9, 65, 35, 53, 23, 25, 68, 15, 95, 25, 70, 27, 3, 33, 2, 31, 17, 40, 60, 24, 94, 34,
		6, 99, 9, 92, 1, 92, 7, 49, 32, 8, 46, 47, 13, 37, 15, 11, 2, 15, 24, 8, 73, 8, 21, 64, 19, 74, 24, 5, 60, 9,
		21, 47, 12, 12, 72, 18, 39, 90, 16, 6, 85, 13, 71, 19, 14, 24, 2, 65, 11, 51, 9, 19, 23, 34, 12, 9, 88, 77, 17,
		6, 72, 19, 79, 39, 19, 21, 95, 87, 24, 91, 53, 7, 29, 20, 25, 11, 39, 38, 24, 72, 6, 1, 97, 15, 87, 11, 77, 64,
		17, 57, 95, 9, 85, 19, 77, 8, 18, 97, 8, 39, 49, 4, 16, 81, 12, 36, 7, 7, 81, 22, 52, 56, 22, 47, 42, 4, 46,
		75, 21, 19, 85, 37, 22, 90, 20, 10, 56, 24, 85, 55, 4, 91, 7, 22, 86, 1, 89, 13, 68, 35, 14, 27, 35, 9, 44, 79,
		12, 42, 20, 16, 28, 89, 11, 57, 10, 60, 15, 13, 95, 3, 48, 24, 90, 86, 51, 18, 8, 71, 11, 80, 91, 5, 4, 93, 9,
		80, 94, 9, 31, 7, 6, 90, 6, 57, 18, 19, 41, 69, 57, 8, 3, 42, 21, 16, 5, 79, 9, 13, 56, 99, 98, 19, 22, 85, 14,
		35, 12, 21, 69, 16, 23, 3, 5, 78, 68, 2, 24, 12, 35, 36, 24, 93, 72, 12, 16, 7, 7, 19, 56, 8, 69, 45, 94, 18,
		49, 44, 61, 21, 25, 19, 96, 7, 13, 27, 50, 76, 14, 5, 60, 4, 11, 90, 60, 9, 31, 85, 17, 11, 18, 74, 37, 20, 53,
		53, 1, 42, 93, 66, 24, 10, 10, 73, 36, 19, 84, 14, 87, 71, 18, 64, 58, 3, 9, 70, 14, 10, 62, 81, 25, 19, 52, 5,
		3, 78, 10, 66, 84, 84, 14, 66, 9, 19, 81, 8, 56, 11, 7, 39, 84, 31, 98, 22, 25, 56, 4, 12, 43, 78, 20, 19, 43,
		88, 23, 10, 62, 90, 22, 38, 29, 5, 29, 32, 20, 14, 1, 3, 44, 13, 92, 79, 11, 59, 22, 77, 38, 3, 83, 18, 22, 37,
		24, 32, 8, 19, 47, 20, 23, 32, 14, 72, 80, 24, 37, 33, 20, 8, 12, 17, 31, 20, 13, 51, 68, 65, 19, 31, 1, 1, 47,
		88, 15, 31, 25, 94, 4, 11, 95, 87, 16, 77, 86, 92, 3, 2, 48, 39, 52, 62, 22, 63, 1, 70, 18, 61, 78, 14, 12, 50,
		75, 10, 30, 2, 10, 96, 13, 58, 87, 9, 90, 3, 83, 5, 13, 28, 3, 67, 66, 21, 46, 10, 1, 70, 64, 8, 10, 50, 13,
		22, 93, 3, 58, 13, 58, 2, 69, 1, 44, 2, 18, 22, 61, 61, 25, 36, 20, 7, 31, 6, 2, 7, 29, 2, 27, 22, 93, 16, 25,
		8, 79, 93, 22, 2, 29, 27, 12, 56, 48, 34, 6, 40, 14, 13, 8, 14, 2, 8, 64, 32, 19, 18, 99, 22, 83, 83, 79, 16,
		84, 58, 22, 88, 19, 31, 18, 35, 18, 31, 85, 20, 30, 16, 75, 16, 46, 16, 65, 16, 3, 44, 6, 2, 65, 97, 24, 40,
		20, 25, 31, 88, 14, 66, 20, 13, 11, 76, 18, 43, 67, 13, 92, 47, 9, 81, 78, 20, 51, 12, 7, 43, 17, 24, 99, 14,
		4, 89, 13, 84, 48, 13, 60, 13, 51, 23, 66, 7, 61, 19, 91, 17, 72, 64, 48, 10, 74, 13, 85, 8, 76, 11, 72, 3, 32,
		22, 37, 80, 44, 18, 86, 50, 71, 5, 36, 21, 76, 23, 64, 23, 61, 40, 62, 24, 61, 0, 0, 21, 21, 1, 10, 1, 0, 0, 0,
		0, 0, 0,
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
	x: i64,
	y: i64,
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

	fn next_in_direction(&self, direction: &Direction) -> Coordinate {
		match direction {
			Direction::North => self.next_north(),
			Direction::South => self.next_south(),
			Direction::East => self.next_east(),
			Direction::West => self.next_west(),
		}
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
	Oxygen,
}

impl TileType {
	fn character_representation(&self) -> char {
		match self {
			Self::Empty => '.',
			Self::Wall => '#',
			Self::Oxygen => 'O',
		}
	}
}

impl Default for TileType {
	fn default() -> Self {
		Self::Empty
	}
}

enum Direction {
	North,
	South,
	East,
	West,
}

impl Direction {
	fn instruction(&self) -> i64 {
		match self {
			Self::North => 1,
			Self::South => 2,
			Self::East => 3,
			Self::West => 4,
		}
	}
}

#[derive(Clone)]
struct BranchState {
	number_of_hops: usize,
	current_state: ProgramState,
	current_location: Coordinate,
	found_oxygen: bool,
}

impl BranchState {
	fn new(current_state: &ProgramState, current_location: Coordinate) -> Self {
		let current_state = current_state.clone();
		Self {
			number_of_hops: 0,
			current_state,
			current_location,
			found_oxygen: false,
		}
	}
}

fn search_next(states: &[BranchState], tiles: &mut HashMap<Coordinate, TileType>) -> Vec<BranchState> {
	let mut new_states: Vec<BranchState> = Vec::new();

	for state in states.iter() {
		let north_state = advance_branch_state(state.clone(), tiles, Direction::North);
		let south_state = advance_branch_state(state.clone(), tiles, Direction::South);
		let east_state = advance_branch_state(state.clone(), tiles, Direction::East);
		let west_state = advance_branch_state(state.clone(), tiles, Direction::West);

		if let Some(state) = north_state {
			new_states.push(state);
		}
		if let Some(state) = south_state {
			new_states.push(state);
		}
		if let Some(state) = east_state {
			new_states.push(state);
		}
		if let Some(state) = west_state {
			new_states.push(state);
		}
	}

	new_states
}

fn advance_branch_state(
	mut state: BranchState,
	tiles: &mut HashMap<Coordinate, TileType>,
	direction: Direction,
) -> Option<BranchState> {
	let dest_coordinate = state.current_location.next_in_direction(&direction);
	if tiles.contains_key(&dest_coordinate) {
		return None;
	}

	let mut input: VecDeque<i64> = VecDeque::new();
	input.push_back(direction.instruction());
	let mut output: Vec<i64> = Vec::new();
	execute_program(&mut state.current_state, &mut input, &mut output);
	let output = if let Some(result) = output.get(0) {
		*result
	} else {
		return None;
	};
	if output == 0 {
		tiles.insert(dest_coordinate, TileType::Wall);
		None
	} else if output == 1 {
		state.number_of_hops += 1;
		state.current_location = dest_coordinate.clone();
		tiles.insert(dest_coordinate, TileType::Empty);
		Some(state)
	} else if output == 2 {
		state.found_oxygen = true;
		state.number_of_hops += 1;
		state.current_location = dest_coordinate.clone();
		tiles.insert(dest_coordinate, TileType::Oxygen);
		Some(state)
	} else {
		None
	}
}

fn print_tiles(tiles: &HashMap<Coordinate, TileType>) {
	let mut lowest_x: Option<i64> = None;
	let mut highest_x: Option<i64> = None;
	let mut lowest_y: Option<i64> = None;
	let mut highest_y: Option<i64> = None;

	for (tile_coord, _) in tiles.iter() {
		if let Some(x) = lowest_x {
			if tile_coord.x < x {
				lowest_x = Some(tile_coord.x);
			}
		} else {
			lowest_x = Some(tile_coord.x);
		}
		if let Some(x) = highest_x {
			if tile_coord.x > x {
				highest_x = Some(tile_coord.x);
			}
		} else {
			highest_x = Some(tile_coord.x);
		}
		if let Some(y) = lowest_y {
			if tile_coord.y < y {
				lowest_y = Some(tile_coord.y);
			}
		} else {
			lowest_y = Some(tile_coord.y);
		}
		if let Some(y) = highest_y {
			if tile_coord.y > y {
				highest_y = Some(tile_coord.y);
			}
		} else {
			highest_y = Some(tile_coord.y);
		}
	}

	let lowest_x = if let Some(x) = lowest_x {
		x
	} else {
		return;
	};
	let highest_x = if let Some(x) = highest_x {
		x
	} else {
		return;
	};
	let lowest_y = if let Some(y) = lowest_y {
		y
	} else {
		return;
	};
	let highest_y = if let Some(y) = highest_y {
		y
	} else {
		return;
	};

	for y in lowest_y..=highest_y {
		for x in lowest_x..=highest_x {
			let print_repr = if let Some(tile_type) = tiles.get(&Coordinate { x, y }) {
				tile_type.character_representation()
			} else {
				' '
			};
			print!("{}", print_repr);
		}
		println!();
	}
	println!();
}

fn main() {
	let mut tiles: HashMap<Coordinate, TileType> = HashMap::new();
	tiles.insert(Coordinate::default(), TileType::Empty);
	let program_state = ProgramState::new();
	let mut branch_states: Vec<BranchState> = vec![BranchState::new(&program_state, Coordinate::default())];

	loop {
		branch_states = search_next(&branch_states, &mut tiles);

		if branch_states.is_empty() {
			break;
		}
	}
	print_tiles(&tiles);

	let mut current_search_tiles: Vec<Coordinate> = Vec::new();
	let mut minutes_passed: usize = 0;
	let oxygen_tile = {
		let mut tile_iter = tiles.iter();
		loop {
			let (tile_coord, tile_type) = if let Some(tile) = tile_iter.next() {
				tile
			} else {
				unreachable!()
			};
			if *tile_type == TileType::Oxygen {
				break tile_coord;
			}
		}
	};
	let mut oxygenated_tiles: HashSet<Coordinate> = HashSet::new();
	oxygenated_tiles.insert(oxygen_tile.clone());
	current_search_tiles.push(oxygen_tile.clone());

	loop {
		let mut next_search_tiles: Vec<Coordinate> = Vec::new();
		for tile in current_search_tiles.iter() {
			let next_north = tile.next_north();
			if *tiles.get(&next_north).unwrap() != TileType::Wall && !oxygenated_tiles.contains(&next_north) {
				next_search_tiles.push(next_north.clone());
				oxygenated_tiles.insert(next_north);
			}
			let next_south = tile.next_south();
			if *tiles.get(&next_south).unwrap() != TileType::Wall && !oxygenated_tiles.contains(&next_south) {
				next_search_tiles.push(next_south.clone());
				oxygenated_tiles.insert(next_south);
			}
			let next_east = tile.next_east();
			if *tiles.get(&next_east).unwrap() != TileType::Wall && !oxygenated_tiles.contains(&next_east) {
				next_search_tiles.push(next_east.clone());
				oxygenated_tiles.insert(next_east);
			}
			let next_west = tile.next_west();
			if *tiles.get(&next_west).unwrap() != TileType::Wall && !oxygenated_tiles.contains(&next_west) {
				next_search_tiles.push(next_west.clone());
				oxygenated_tiles.insert(next_west);
			}
		}
		if next_search_tiles.is_empty() {
			break;
		}
		current_search_tiles = next_search_tiles;
		minutes_passed += 1;
	}
	println!("{}", minutes_passed);
}
