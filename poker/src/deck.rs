use crate::card::Card;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
//data structure of a deck (collection of cards)
//cards is an array of cards
//capacity is an unsigned integer representing the size of the deck
pub struct Deck {
    cards: Vec<Card>,
    capacity: usize,
}

//function implementations of a deck
impl Deck {
    /*
    Function: new()
    Purpose: Construtor for the Deck class, initializing a new array of cards and setting the maximum capacity to cap, which is an argument it takes in
    Parameters: cap (IN) - maximum capacity of the deck
    Returns: A new instance of the Deck struct with an empty array of cards and a maximum capacity of 'cap'
    */
    pub fn new(cap: usize) -> Self {
        Self {
            cards: Vec::new(),
            capacity: cap,
        }
    }

    /*
    Function: fill_standard()
    Purpose: Fills the deck with cards with a standard set of 52 cards (all 13 ranks of all 4 suits)
    Parameters: N/A
    Returns: N/A
    */
    pub fn fill_standard(&mut self) {
        for i in 0..52 {
            //decides suit by dividing i by 13 and getting the result - 0...12 are spades, 13...25 are hearts, 26...38 are clubs, and 39...52 are diamonds
            let suit = match i / 13 {
                0 => "Spade",
                1 => "Heart",
                2 => "Club",
                3 => "Diamond",
                _ => "INVALID",
            };

            // add cards with values 2-14 (2-Ace) with each of the 4 suits
            self.add_card(Card::new(i % 13 + 2, suit));
        }
    }

    /*
    Function: shuffle()
    Purpose: Shuffles the deck (i.e. randomizes the order of the cards in the deck)
    Parameters: N/A
    Returns: N/A
    */
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /*
    Function: pop_top_card()
    Purpose: Removes and returns the top card on the deck (i.e. the card at array index n - 1)
    Parameters: N/A
    Returns: A "Card" struct that was at the top of the deck
    */
    pub fn pop_top_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    /*
    Function: peek_top_card()
    Purpose: returns the top card on the deck without removing (i.e. the card at array index n - 1)
    Parameters: N/A
    Returns: A "Card" struct that was at the top of the deck
    */
    pub fn peek_top_card(&mut self) -> &Card {
        self.cards.last().unwrap()
    }

    /*
    Function: add_card()
    Purpose: Adds a new card to the top of the deck (i.e. the card at array index n - 1)
    Parameters:
    card (IN) -- the card to be added to the deck
    Returns:
        true -- if the card was successfully added to the deck
        false -- if the deck is already full and the card could not be added
    */
    pub fn add_card(&mut self, card: Card) -> bool {
        if self.get_size() >= self.get_capacity() {
            return false;
        }

        self.cards.push(card);
        return true;
    }

    /*
    Function: fill_from_deck()
    Purpose: Transfer cards from another deck into this deck until the current deck is at capacity or the other deck is out of elements
    Parameters: other (IN/OUT) -- a reference to a Deck type to transfer cards from
    Returns: N/A
    */
    pub fn fill_from_deck(&mut self, other: &mut Self) {
        while self.cards.len() < self.capacity && other.cards.len() > 0 {
            self.add_card(other.pop_top_card());
        }
    }

    /*
    Function: empty_deck()
    Purpose: Empty out this deck.
    Parameters: N/A
    Returns: N/A
    */
    pub fn empty_deck(&mut self) {
        self.cards.clear();
    }

    /*
    Function: get_capacity()
    Purpose: Get the number of cards this deck can hold
    Parameters: N/A
    Returns: usize (unsigned integer) representing the number of cards this deck can hold
    */
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /*
    Function: get_size()
    Purpose: Get the number of cards currently in this deck
    Parameters: N/A
    Returns: usize (unsigned integer) representing the number of cards currently in this deck.
    */
    pub fn get_size(&self) -> usize {
        self.cards.len()
    }

    /*
    Function: get_cards()
    Purpose: Get the deck's backing array of cards
    Parameters: N/A
    Returns: &Vec<Card> - a reference to an array of cards
    */
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
}
