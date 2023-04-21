use crate::{card::Card, deck::Deck};
use std::ops::{Add, Sub};

//data structure of a player
//money is an unsigned integer representing the amount of money a player has
//cards is a "Deck type"
pub struct Player {
    pub money: usize,
    pub cards: Deck,
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
            money: 1000,
            cards: Deck::new(3),
        }
    }
    /*
    Function: get_money()
    Purpose: Get the amount of money that the player has
    Parameters: N/A
    Returns: usize (unsigned integer) representing the amount of money that the palyer has
    */
    pub fn get_money(&self) -> usize {
        self.money
    }

    /*
    Function: get_cards()
    Purpose: Get the player's card array
    Parameters: N/A
    Returns: &Vec<Card> - a reference to the player's card array
    */
    pub fn get_cards(&self) -> &Vec<Card> {
        self.cards.get_cards()
    }

    /*
    Function: fill_from_deck()
    Purpose: Transfer cards from another deck into the player's deck using the Deck::fill_from_deck() function
    Parameters: other (IN/OUT) -- a reference to a Deck type to transfer cards from
    Returns: N/A
    */
    pub fn fill_from(&mut self, other: &mut Deck) {
        self.cards.fill_from_deck(other);
    }

    pub fn empty(&mut self) {
        self.cards.empty_deck();
    }
}

//overloading the + operator for the Player type
impl Add<usize> for Player {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            money: self.money + rhs,
            cards: self.cards,
        }
    }
}

//overloading the - operator for the Player type
impl Sub<usize> for Player {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            money: self.money - rhs,
            cards: self.cards,
        }
    }
}
