use egui::epaint::ahash::{HashSet, HashSetExt};
use crate::{deck::Deck, card::Card};

pub struct HandTests;

impl HandTests {

  pub fn get_deck_points(deck: &Deck) -> u8 {
    if HandTests::test_for_straight_flush(deck) == true {
      return 8;
    }
    else if HandTests::test_for_triple(deck) == true {
      return 7;
    }
    else if HandTests::test_for_straight(deck) == true {
      return 6;
    }
    else if HandTests::test_for_flush(deck) == true {
      return 5;
    }
    else if HandTests::test_for_pair(deck) == true {
      return 4;
    }
    else { return HandTests::test_for_highs(deck)}
  }

  pub fn test_for_straight_flush(deck: &Deck) -> bool {
    if HandTests::test_for_straight(deck) && HandTests::test_for_flush(deck) {
      return true;
    } 
    return false;
  }

  pub fn test_for_triple(deck: &Deck) -> bool {
    let card1 = deck.get_cards().get(0).unwrap();
    let card2 = deck.get_cards().get(1).unwrap();
    let card3 = deck.get_cards().get(2).unwrap();
    if card1.get_value() == card2.get_value() && card2.get_value() == card3.get_value() && card1.get_value() == card3.get_value()  {
      return true;
    } 
    return false;
  }
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
    let mut suits:HashSet<String> = HashSet::new();
      for card in deck.get_cards() {
        suits.insert(card.get_suit().to_string());
      }
    if suits.len() == 1 { return true } else { return false }
  }


  pub fn test_for_pair(deck: &Deck) -> bool {
      let card1 = deck.get_cards().get(0).unwrap();
      let card2 = deck.get_cards().get(1).unwrap();
      let card3 = deck.get_cards().get(2).unwrap();
      if card1.get_value() == card2.get_value() || card2.get_value() == card3.get_value() || card1.get_value() == card3.get_value() {
        return true;
      }
      false
    } 

    pub fn test_for_highs(deck: &Deck) -> u8 {
      let mut high_card:i8 = 0;
      for card in deck.get_cards() {
        if card.get_value() > high_card && card.get_value() > 11 {
          high_card = card.get_value();
        }
      }

      //returns 1-3 depending on if it's a queen, king, or ace high
      return match high_card {
        12 => 1,
        13 => 2,
        14 => 3,
        _ => 0
      }
    }

    
}

