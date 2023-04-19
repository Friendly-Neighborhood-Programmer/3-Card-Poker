use crate::{deck::Deck, card::Card};

pub struct HandTests;

impl HandTests {
  pub fn test_for_straight(deck: &Deck) -> bool {
    for i in 0..2 {
      let curr_card: &Card = deck.get_cards().get(i).unwrap();
      let next_card:&Card = deck.get_cards().get(i + 1).unwrap();
      if next_card.get_value() - curr_card.get_value() != 1 {
        return false;
      }
    }
    true  
  }

  pub fn test_for_flush(deck: &Deck) -> bool {
      for i in 0..2 {
        let curr_card: &Card = deck.get_cards().get(i).unwrap();
        let next_card:&Card = deck.get_cards().get(i + 1).unwrap();
        if next_card.get_suit() != curr_card.get_suit() {
          return false;
        }
      }
      return true;
  }

  pub fn test_for_3k(deck: &Deck) -> bool {
    let card1 = deck.get_cards().get(0).unwrap();
    let card2 = deck.get_cards().get(1).unwrap();
    let card3 = deck.get_cards().get(2).unwrap();
    if card1.get_value() == card2.get_value() && card2.get_value() == card3.get_value() && card1.get_value() == card3.get_value()  {
      return true;
    }
    return false;
  }

  pub fn test_for_pair(deck: &Deck) -> bool {
      let card1 = deck.get_cards().get(0).unwrap();
      let card2 = deck.get_cards().get(1).unwrap();
      let card3 = deck.get_cards().get(2).unwrap();
      if card1.get_value() == card2.get_value() || card2.get_value() == card3.get_value() || card1.get_value() == card3.get_value() {
        if HandTests::test_for_3k(deck) == false {
          return true;
        }
        else {
          return false;
        }
      }
      return false;
    } 
}

