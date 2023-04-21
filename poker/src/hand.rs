use crate::deck::Deck;

//enum detailing the types of hands one can get
//used when checking for hands
pub enum HandType {
    StraightFlush,
    Triple,
    Straight,
    Flush,
    Pair,
    HighAce,
    HighKing,
    HighQueen,
    HighJack,
    Other,
}

/*
Function: get_hand()
Purpose: Given one or two decks, determine the best 3 or 5 card poker hand that can be made from it
Parameters: 
    deck1 (IN) - The player's deck
    deck2 (IN - Optional) - The dealer's deck
Returns: A HandType that represents the best hand that can be made from the given cards
*/
pub fn get_hand(deck1: &Deck, deck2: Option<&Deck>) -> HandType {
    let mut deck = Deck::new(6);
    match deck2 {
        Some(d2) => {
            deck.fill_from_deck(&mut deck1.clone());
            deck.fill_from_deck(&mut d2.clone());
        }
        None => deck.fill_from_deck(&mut deck1.clone()),
    };

    if test_for_straight_flush(&deck) {
        return HandType::StraightFlush;
    }
    if test_for_triple(&deck) {
        return HandType::Triple;
    }
    if test_for_straight(&deck) {
        return HandType::Straight;
    }
    if test_for_flush(&deck) {
        return HandType::Flush;
    }
    if test_for_pair(&deck) {
        return HandType::Pair;
    }
    test_for_high(&deck)
}

/* 
Function: test_for_straight_flush()
Purpose: Test a given deck to see if it contains a straight flush
Parameters: 
    deck (IN) - A deck to test
Returns:
    true -- if the deck contains a straight flush
    false -- if the deck does not contain a straight flush
*/
fn test_for_straight_flush(deck: &Deck) -> bool {
    test_for_straight(deck) && test_for_flush(deck)
}

/* 
Function: test_for_triple()
Purpose: Test a given deck to see if it contains 3 cards of the same rank
Parameters: 
    deck (IN) - A deck to test
Returns:
    true -- if the deck contains a triple
    false -- if the deck does not contain a triple
*/
fn test_for_triple(deck: &Deck) -> bool {
    let cards = deck.get_cards();

    for i in 0..deck.get_size() {
        for j in 0..deck.get_size() {
            for k in 0..deck.get_size() {
                if cards[i] == cards[j]
                    && cards[j] == cards[k]
                    && cards[i].get_suit() != cards[j].get_suit()
                    && cards[j].get_suit() != cards[k].get_suit()
                    && cards[i].get_suit() != cards[k].get_suit()
                {
                    return true;
                }
            }
        }
    }
    false
}

/* 
Function: test_for_straight()
Purpose: Test a given deck to see if it contains cards of increasing rank
Parameters: 
    deck (IN) - A deck to test
Returns:
    true -- if the deck contains a straight
    false -- if the deck does not contain a straight
*/
fn test_for_straight(deck: &Deck) -> bool {
    let cards = deck.get_cards();

    for i in 0..deck.get_size() {
        for j in 0..deck.get_size() {
            for k in 0..deck.get_size() {
                if (cards[i].get_value() - cards[j].get_value() == 1)
                    && (cards[j].get_value() - cards[k].get_value() == 1)
                {
                    return true;
                }
            }
        }
    }
    false
}

/* 
Function: test_for_flush()
Purpose: Test a given deck to see if it contains cards of the same suit
Parameters: 
    deck (IN) - A deck to test
Returns:
    true -- if the deck contains a flush
    false -- if the deck does not contain a flush
*/
fn test_for_flush(deck: &Deck) -> bool {
    // ordered as (Spade, Heart, Club, Diamond)
    let (s, h, c, d) = deck
        .get_cards()
        .iter()
        .fold((0, 0, 0, 0), |t, c| match c.get_suit() {
            "Spade" => (t.0 + 1, t.1, t.2, t.3),
            "Heart" => (t.0, t.1 + 1, t.2, t.3),
            "Club" => (t.0, t.1, t.2 + 1, t.3),
            "Diamond" => (t.0, t.1, t.2, t.3 + 1),
            _ => t,
        });
    vec![s, h, c, d].iter().max().unwrap() >= &3
}

/* 
Function: test_for_pair()
Purpose: Test a given deck to see if it contains two cards of the same rank
Parameters: 
    deck (IN) - A deck to test
Returns:
    true -- if the deck contains a pair
    false -- if the deck does not contain a pair
*/
fn test_for_pair(deck: &Deck) -> bool {
    let cards = deck.get_cards();

    for i in 0..deck.get_size() {
        for j in 0..deck.get_size() {
            if cards[i] == cards[j] && cards[i].get_suit() != cards[j].get_suit() {
                return true;
            }
        }
    }
    false
}

/* 
Function: test_for_high()
Purpose: Test a given deck to see if it contains one of the high cards (J,Q,K,A)
Parameters: 
    deck (IN) - A deck to test
Returns:
    11 -- if the deck contains a Jack 
    12 -- if the deck contains a Queen
    13 -- if the deck contains a King
    14 -- if the deck contains an Ace
    _ -- if the deck does not contain any of those
*/
fn test_for_high(deck: &Deck) -> HandType {
    let high_card = deck.get_cards().iter().max().unwrap();

    return match high_card.get_value() {
        11 => HandType::HighJack,
        12 => HandType::HighQueen,
        13 => HandType::HighKing,
        14 => HandType::HighAce,
        _ => HandType::Other,
    };
}
