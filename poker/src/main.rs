#[allow(unused_imports)]
use deck::Deck;
#[allow(unused_imports)]
use handtests::HandTests;
mod card;
mod deck;
mod handtests;
mod player;

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
    match HandTests::get_deck_points(&player_hand) {
        8 => println!("Straight flush"),
        7 => println!("Triple"),
        6 => println!("Straight"),
        5 => println!("Flush"),
        4 => println!("Pair"),
        3 => println!("Ace high"),
        2 => println!("King high"),
        1 => println!("Queen high"),
        _ => println!("Nothing")
    }  
}
    