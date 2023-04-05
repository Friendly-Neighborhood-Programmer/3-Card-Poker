use deck::Deck;

mod card;
mod deck;

fn main() {

    print!("Hello Poker");
    
}

#[test]
fn test_deck() {
    let mut deck = Deck::new(52);
    deck.fill_standard();
    deck.shuffle();

    print!("main deck: \n{:#?}\n", deck);
    print!("deck size: {}\n\n", deck.get_size());

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
    player_hand.add_card(deck.pop_top_card());
    print!("new player hand: \n{:#?}\n", player_hand);

}
