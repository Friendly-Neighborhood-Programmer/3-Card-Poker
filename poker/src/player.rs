use std::ops::{Add, Sub};
use crate::{deck::Deck, card::Card};

//data structure of a player
//money is an unsigned integer representing the amount of money a player has
//cards is a "Deck type"
pub struct Player {
    money: usize,
    cards: Deck,
}

//function implementations for a player type
impl Player {
    /*
    Function: new()
    Purpose: Construtor for the Player class, setting the player's initial amount of money at $100 and creating their hand as a "Deck" struct with capacity 3
    Parameters: N/A
    Returns: A new instance of the Player struct with an empty deck and $100 to start out
    */
    pub fn new() -> Self {
        Self {
            money: 100,
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
