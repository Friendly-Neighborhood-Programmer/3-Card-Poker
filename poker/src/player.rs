//importing decks and hands
use crate::deck::Deck;
use crate::hand::{
    get_hand,
    HandType::{Flush, Straight, StraightFlush, Triple, Pair},
};

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

    /*
    Function: ante_bet()
    Purpose: Allows the player to place an ante bet and determines the payoff depending on their hand
    Parameters:
    amount (IN) -- the amount of the player's bet
    Returns: N/A
    */
    pub fn ante_bet(&mut self, amount: usize) {
        const STRAIGHT_FLUSH_PAYOFF: usize = 5;
        const TRIPLE_PAYOFF: usize = 4;

        let hand = get_hand(&self.cards, None);

        match hand {
            StraightFlush => self.money += amount * STRAIGHT_FLUSH_PAYOFF,
            Triple => self.money += amount * TRIPLE_PAYOFF,
            Straight => self.money += amount,
            _ => self.money -= amount
        }
    }

    /*
    Function: pair_plus_bet()
    Purpose: Allows the player to place a pair plus bet and determines the payoff depending on their hand
    Parameters:
    amount (IN) -- the amount of the player's bet
    Returns: N/A
    */
    pub fn pair_plus_bet(&mut self, amount: usize) {
        const STRAIGHT_FLUSH_PAYOFF: usize = 40;
        const TRIPLE_PAYOFF: usize = 30;
        const STRAIGHT_PAYOFF: usize = 6;
        const FLUSH_PAYOFF: usize = 3;
        
        let hand = get_hand(&self.cards, None);

        match hand {
            StraightFlush => self.money += amount * STRAIGHT_FLUSH_PAYOFF,
            Triple => self.money += amount * TRIPLE_PAYOFF,
            Straight => self.money += amount * STRAIGHT_PAYOFF,
            Flush => self.money += amount * FLUSH_PAYOFF,
            Pair => self.money += amount,
            _ => self.money -= amount,
        }
    }
}
