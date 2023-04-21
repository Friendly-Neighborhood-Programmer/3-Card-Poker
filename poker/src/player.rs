use crate::deck::Deck;
use crate::hand::HandType;
use crate::hand::{HandType::{Flush, Straight, StraightFlush, Triple, Pair}, get_hand};

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

    pub fn pair_plus_bet(&mut self, amount: usize) {
        const STRAIGHT_FLUSH_PAYOFF: usize = 40;
        const TRIPLE_PAYOFF: usize = 30;
        const STRAIGHT_PAYOFF: usize = 6;
        const FLUSH_PAYOFF: usize = 3;
        
        let hand:HandType = get_hand(&self.cards, None);

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
