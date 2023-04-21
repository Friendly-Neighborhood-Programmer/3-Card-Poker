use std::ops::{Add, Sub};
use crate::{deck::Deck, card::Card};

pub struct Player {
    money: usize,
    cards: Deck,
}

impl Player {
    pub fn new() -> Self {
        Self {
            money: 0,
            cards: Deck::new(3),
        }
    }

    pub fn get_money(&self) -> usize {
        self.money
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        self.cards.get_cards()
    }

    pub fn fill_from(&mut self, other: &mut Deck) {
        self.cards.fill_from_deck(other);
    }
}

impl Add<usize> for Player {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            money: self.money + rhs,
            cards: self.cards,
        }
    }
}

impl Sub<usize> for Player {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            money: self.money - rhs,
            cards: self.cards,
        }
    }
}
