use std::fs;
use std::mem::swap;

const DECK_SIZE: i128 = 119315717514047;
const NUMBER_OF_SHUFFLES: u64 = 101741582076661;

fn mod_mult_inverse(a: i128, n: i128) -> i128 {
	let mut t = 0;
	let mut new_t = 1;
	let mut r = n;
	let mut new_r = a;

	while new_r != 0 {
		let quotient = r / new_r;
		t -= quotient * new_t;
		r -= quotient * new_r;
		swap(&mut t, &mut new_t);
		swap(&mut r, &mut new_r);
	}

	if t < 0 {
		t += n;
	}

	t
}

/// Represents a formula of the form ax + b, where a is represented as
/// `factor`, and b as `addend`.
#[derive(Debug)]
struct Formula {
	factor: i128,
	addend: i128,
}

impl Formula {
	fn new() -> Self {
		Self { factor: 1, addend: 0 }
	}

	fn _execute(&self, value: i128) -> i128 {
		(self.factor * value + self.addend).rem_euclid(DECK_SIZE)
	}

	fn update_apply(&mut self, other: &Self) {
		/*
		Here, we want to apply `second` after `first`
		`first` represents ax+b; `second` (which we'll shift to differentiate)
		represents cx+d
		The result of `first` applied to `second` looks like c(ax+b)+d (using
		substitution)
		Distributing the `c` gives us acx+bc+d. With this, we have a new factor
		for `x` of `ac` and a new addend of `bc+d`.
		Because this is modular math, both can be kept modulus the deck size to
		ensure the numbers don't overflow.
		*/
		self.factor = (self.factor * other.factor).rem_euclid(DECK_SIZE);
		self.addend = (self.addend * other.factor + other.addend).rem_euclid(DECK_SIZE);
	}

	fn compose(formula1: &Self, formula2: &Self) -> Self {
		let factor = (formula1.factor * formula2.factor).rem_euclid(DECK_SIZE);
		let addend = (formula1.addend * formula2.factor + formula2.addend).rem_euclid(DECK_SIZE);
		Formula { factor, addend }
	}

	fn execute_inverse(&self, value: i128) -> i128 {
		((value - self.addend) * mod_mult_inverse(self.factor, DECK_SIZE)).rem_euclid(DECK_SIZE)
	}
}

fn deal_into_new_stack() -> Formula {
	// DECK_SIZE - (ax + b)
	// -ax - b + DECK_SIZE mod DECK_SIZE
	// -ax - b
	Formula { factor: -1, addend: -1 }
}

fn cut_cards(num_cards: i128) -> Formula {
	// ax + b - num_cards
	Formula { factor: 1, addend: -num_cards }
}

fn deal_cards_with_increment(increment: i128) -> Formula {
	// (ax + b) * increment mod DECK_SIZE
	Formula { factor: increment, addend: 0 }
}

fn _repeat_shuffle_pow(mut base: i128) -> i128 {
	// A normal repeated exponentiation also takes far too long with this exponent (sigh).
	// This has taken far too long so I'm now just using this: https://en.wikipedia.org/wiki/Modular_exponentiation#Pseudocode
	let mut result = 1;
	let mut exponent = NUMBER_OF_SHUFFLES;
	while exponent > 0 {
		if exponent % 2 == 1 {
			result = (result * base).rem_euclid(DECK_SIZE);
		}
		exponent >>= 1;
		base = (base * base).rem_euclid(DECK_SIZE);
	}
	result
}

fn repeat_compose(mut formula: Formula) -> Formula {
	// Function composition can take the place of exponentiation.
	// To exponent quickly enough, I've adapted this for formula composition:
	// https://en.wikipedia.org/wiki/Modular_exponentiation#Pseudocode
	let mut result_formula = Formula::new();
	let mut composition_count = NUMBER_OF_SHUFFLES;
	while composition_count > 0 {
		if composition_count % 2 == 1 {
			result_formula = Formula::compose(&result_formula, &formula);
		}
		composition_count /= 2;
		formula = Formula::compose(&formula, &formula);
	}
	result_formula
}

fn main() {
	let shuffle_instructions: Vec<String> = fs::read_to_string("input.txt")
		.expect("Failed to read input file")
		.lines()
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();
	let position_of_card: i128 = 2020;

	let mut formula = Formula::new();
	for instruction in shuffle_instructions.iter() {
		if instruction == "deal into new stack" {
			formula.update_apply(&deal_into_new_stack());
		} else if instruction.len() > 20 && instruction[0..20] == *"deal with increment " {
			let increment = instruction[20..].parse().expect("Invalid increment");
			formula.update_apply(&deal_cards_with_increment(increment));
		} else if instruction.len() > 4 && instruction[0..4] == *"cut " {
			let num_cards: i128 = instruction[4..].parse().expect("Invalid number of cards");
			formula.update_apply(&cut_cards(num_cards));
		} else {
			unimplemented!()
		}
	}

	let complete_formula = repeat_compose(formula);
	println!("{:?}", complete_formula);
	let card = complete_formula.execute_inverse(position_of_card);

	println!("Card: {}", card);
}