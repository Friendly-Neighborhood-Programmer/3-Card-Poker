use rand::{thread_rng, seq::SliceRandom};
use crate::card::Card;

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
		for i in 0..52 {
			let suit = match i / 13 {
				0 => "Spade",
				1 => "Heart",
				2 => "Club",
				3 => "Diamond",
				_ => "INVALID",
			};
			
			// add cards with values 2-14 (2-Ace) with each of the 4 suits
			self.add_card(Card::new(i % 13 + 2, suit));
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
	pub fn add_card(&mut self, card: Card) -> bool {
		if self.get_size() >= self.get_capacity() { return false; } 

		self.cards.push(card);
		return true;
	}

	// pull cards from other into calling deck
	pub fn fill_from_deck(&mut self, other: &mut Deck) {
		while self.cards.len() < self.capacity {
			self.add_card(other.pop_top_card());
		}
	}

	// pull cards from other into calling deck
	pub fn empty_deck(&mut self) {
		self.cards.clear();
	}

	pub fn get_capacity(&self) -> usize {
		self.capacity
	}

	pub fn get_size(&self) -> usize {
        self.cards.len()
    }
}
