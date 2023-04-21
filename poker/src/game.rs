use crate::hand::{
    get_hand,
    HandType::{Flush, Pair, Straight, StraightFlush, Triple},
};

struct GameManager {
	deck: Deck,
	dealer: Player,
	player: Player,
	ante: usize,
	play: usize,
}

//TODO: complete game manager logic: dealer qualify, player play, 
//      player fold, push bet

impl GameManager {
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