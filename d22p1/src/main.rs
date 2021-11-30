use std::fs;

fn deal_into_new_stack(deck: Vec<usize>) -> Vec<usize> {
	deck.iter().rev().copied().collect()
}

fn cut_top_cards(deck: &mut Vec<usize>, num_cards: usize) {
	let mut cards_to_move: Vec<usize> = Vec::with_capacity(num_cards);
	for _ in 0..num_cards {
		cards_to_move.push(deck.pop().unwrap());
	}
	for card in cards_to_move {
		deck.insert(0, card);
	}
}

fn cut_bottom_cards(deck: &mut Vec<usize>, num_cards: usize) {
	let cards_to_move = deck[..num_cards].to_vec();
	*deck = deck[num_cards..].to_vec();
	deck.extend(cards_to_move);
}

fn deal_cards_with_increment(mut deck: Vec<usize>, increment: usize) -> Vec<usize> {
	let mut table: Vec<usize> = vec![0; deck.len()];
	let mut table_index_iter = (0..table.len()).rev().cycle();
	table[table_index_iter.next().unwrap()] = deck.pop().unwrap();
	let increment = increment - 1;
	while let Some(card) = deck.pop() {
		table[table_index_iter.nth(increment).unwrap()] = card;
	}
	table
}

fn main() {
	let shuffle_instructions: Vec<String> = fs::read_to_string("input.txt")
		.expect("Failed to read input file")
		.lines()
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();
	let mut deck = Vec::from_iter((0..10007).rev());

	for instruction in shuffle_instructions.iter() {
		if instruction == "deal into new stack" {
			deck = deal_into_new_stack(deck);
		} else if instruction.len() > 20 && instruction[0..20] == *"deal with increment " {
			let increment: usize = instruction[20..].parse().expect("Invalid increment");
			deck = deal_cards_with_increment(deck, increment as usize);
		} else if instruction.len() > 4 && instruction[0..4] == *"cut " {
			let num_cards: i64 = instruction[4..].parse().expect("Invalid number of cards");
			if num_cards >= 0 {
				cut_top_cards(&mut deck, num_cards as usize);
			} else {
				cut_bottom_cards(&mut deck, -num_cards as usize);
			}
		} else {
			panic!("Invalid instruction");
		}
	}

	deck = deck.iter().rev().copied().collect();

	let mut index_2019: Option<usize> = None;
	for (card_index, card) in deck.iter().enumerate() {
		if *card == 2019 {
			index_2019 = Some(card_index);
			break;
		}
	}
	println!("Position of card 2019: {}", index_2019.unwrap());
}
