use rand::{thread_rng, seq::SliceRandom};
use crate::card::{Card, self};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    capacity: usize,
}

impl Deck {
    pub fn new(cap: usize) -> Self {
        Self {
            cards: Vec::new(),
            capacity: cap,
        }
    }

	// create a standard 52 card deck
    pub fn fill_standard(&mut self) {
		let mut suit_count = 0;

		for i in 0..52 {
			let suit = match suit_count {
				0 => "Spade",
				1 => "Heart",
				2 => "Club",
				3 => "Diamond",
				_ => "",
			};
			if i % 13 == 12 { suit_count += 1; }

			self.cards.push(Card::new(i % 13 + 2, suit));
		}
    }

	// randomize the cards in deck
	pub fn shuffle(&mut self) {
		self.cards.shuffle(&mut thread_rng());
	}

	// remove and return the top card
	pub fn pop_top_card(&mut self) -> Card {
		self.cards.pop().unwrap()
	}
	
	// remove and return the top card
	pub fn add_card(&mut self, card:Card) {
		self.cards.push(card);
	}

	pub fn get_capacity(&self) -> usize {
		self.capacity
	}

	pub fn get_size(&self) -> usize {
        self.cards.len()
    }
}
