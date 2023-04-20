use crate::deck::Deck;
use crate::handtests::HandTests;

pub struct Player {
  money: i8,
  cards: Deck
}

impl Player {
  pub fn new() -> Self {
    Self {
      money: 0,
      cards: Deck::new(3)
    }
  }

  pub fn ante_bet(&mut self, amount:i8) {
    const STRAIGHT_FLUSH_PAYOFF:i8 = 5;
    const TRIPLE_PAYOFF:i8 = 4;
    let straight_flush = HandTests::test_for_straight_flush(&self.cards);
    let triple = HandTests::test_for_triple(&self.cards);
    let straight = HandTests::test_for_straight(&self.cards);

    if straight_flush {
      self.money += amount * STRAIGHT_FLUSH_PAYOFF;
    }
    else if triple {
      self.money += amount * TRIPLE_PAYOFF;
    }
    else if straight {
      self.money += amount;
    }
    else {
      self.money += 0;
    }

  }

  pub fn pair_plus_bet(&mut self, amount:i8) {
    const STRAIGHT_FLUSH_PAYOFF:i8 = 5;
    const TRIPLE_PAYOFF:i8 = 30;
    const STRAIGHT_PAYOFF:i8 = 6;
    const FLUSH_PAYOFF:i8 = 3;
    let straight_flush = HandTests::test_for_straight_flush(&self.cards);
    let triple = HandTests::test_for_triple(&self.cards);
    let straight = HandTests::test_for_straight(&self.cards);
    let flush = HandTests::test_for_flush(&self.cards);
    let pair = HandTests::test_for_pair(&self.cards);

    if straight_flush {
      self.money += amount * STRAIGHT_FLUSH_PAYOFF;
    }
    else if triple {
      self.money += amount * TRIPLE_PAYOFF;
    }
    else if straight {
      self.money += amount * STRAIGHT_PAYOFF;
    }
    else if flush {
      self.money += amount * FLUSH_PAYOFF;
    }
    else if pair {
      self.money += amount;
    }
    else {
      return;
    }
  }
}