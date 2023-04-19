use deck::Deck;
use handtests::HandTests;
mod card;
mod deck;
mod handtests;

fn main() {
    print!("Hello Poker");   
}

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
    if(HandTests::test_for_straight(&player_hand) == true) {
        println!("got a straight!");
    }
    if(HandTests::test_for_flush(&player_hand) == true) {
        println!("got a flush!")
    }
    if(HandTests::test_for_3k(&player_hand) == true) {
        println!("got 3 of a kind!")
    }
    if(HandTests::test_for_pair(&player_hand) == true) {
        println!("got a pair!")
    }
}
    