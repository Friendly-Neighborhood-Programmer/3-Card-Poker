use crate::hand::{get_hand, HandType};
use card::Card;
use deck::Deck;
mod card;
mod deck;
mod hand;
mod player;
mod view;

fn main() {
    view::init_app().unwrap();
}

////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_deck() {
    // create main deck, fill and shuffle
    let mut deck = Deck::new(52);
    deck.fill_standard();
    deck.shuffle();
    print!("main deck: \n{:#?}\n", deck);
    print!("deck size: {}\n\n", deck.get_size());

    // create smaller deck for player hand
    let mut player_hand = Deck::new(3);
    let top_card = deck.pop_top_card();

    print!("top card: \n{:#?}\n", top_card);
    print!("top value: {:#?}\n", top_card.get_value());
    print!("top suit: {:#?}\n\n", top_card.get_suit());
    print!("new deck size: {}\n", deck.get_size());
    print!("new deck capacity: {}\n\n", deck.get_capacity());

    player_hand.add_card(top_card);
    print!("player hand: \n{:#?}\n\n", player_hand);
    print!("pushing new top card to player \n");
    //dont ever do this in the real program
    player_hand.add_card(deck.pop_top_card());
    print!("new player hand: \n{:#?}\n", player_hand);

    player_hand.empty_deck();
    // only use this to tranfer cards
    player_hand.fill_from_deck(&mut deck);
    print!("new player hand: \n{:#?}\n", player_hand);
    print!("new deck size: {}\n", deck.get_size());
}

#[test]
fn test_hands() {
    let mut player = Deck::new(3);
    player.add_card(Card::new(10, "Spade"));
    player.add_card(Card::new(11, "Spade"));
    player.add_card(Card::new(12, "Spade"));

    match get_hand(&player, None) {
        HandType::StraightFlush => println!("Straight flush"),
        HandType::Triple => println!("Triple"),
        HandType::Straight => println!("Straight"),
        HandType::Flush => println!("Flush"),
        HandType::Pair => println!("Pair"),
        HandType::HighAce => println!("Ace high"),
        HandType::HighKing => println!("King high"),
        HandType::HighQueen => println!("Queen high"),
        HandType::HighJack => println!("Jack high"),
        HandType::Other => println!("Other"),
    }
}
