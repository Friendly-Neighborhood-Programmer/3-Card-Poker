use crate::deck::Deck;

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

fn test_for_straight_flush(deck: &Deck) -> bool {
    test_for_straight(deck) && test_for_flush(deck)
}

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
