use crate::deck::Deck;
use crate::handtests::HandTests;

pub struct Player {
  money: i32,
  cards: Deck
}

impl Player {
  pub fn new() -> Self {
    Self {
      money: 0,
      cards: Deck::new(3)
    }
  }

  pub fn ante_bet(&self, amount:u8) -> u8 {
    const STRAIGHT_FLUSH_PAYOFF:u8 = 5;
    const TRIPLE_PAYOFF:u8 = 4;
    let straight_flush = HandTests::test_for_straight_flush(&self.cards);
    let triple = HandTests::test_for_triple(&self.cards);
    let straight = HandTests::test_for_straight(&self.cards);

    if straight_flush {
      return amount * STRAIGHT_FLUSH_PAYOFF;
    }
    else if triple {
      return amount * TRIPLE_PAYOFF;
    }
    else if straight {
      return amount;
    }
    else {
      return 0;
    }

  }

  pub fn pair_plus_bet(&self, amount:u8) -> u8 {
    const STRAIGHT_FLUSH_PAYOFF:u8 = 5;
    const TRIPLE_PAYOFF:u8 = 30;
    const STRAIGHT_PAYOFF:u8 = 6;
    const FLUSH_PAYOFF:u8 = 3;
    let straight_flush = HandTests::test_for_straight_flush(&self.cards);
    let triple = HandTests::test_for_triple(&self.cards);
    let straight = HandTests::test_for_straight(&self.cards);
    let flush = HandTests::test_for_flush(&self.cards);
    let pair = HandTests::test_for_pair(&self.cards);

    if straight_flush {
      return amount * STRAIGHT_FLUSH_PAYOFF;
    }
    else if triple {
      return amount * TRIPLE_PAYOFF;
    }
    else if straight {
      return amount * STRAIGHT_PAYOFF;
    }
    else if flush {
      return amount * FLUSH_PAYOFF;
    }
    else if pair {
      return amount;
    }
    else {
      return 0;
    }
  }
}