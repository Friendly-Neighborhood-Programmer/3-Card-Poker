use std::{cmp::Ordering, ops::Sub};

#[derive(Debug, Clone, Hash)]
//data structure of a card
pub struct Card {
    //card value is stored as a signed 8-bit integer
    //values 2-10 represent their respective face cards,
    //with 11 = J, 12 = Q, 13 = K, 14 = A (aces high)
    value: i8,
    //card suit is stored as a string
    suit: String,
}

//function implementations of a card
impl Card {
    /*
    Function: new()
    Purpose: Construtor for the Card struct, which sets the card's rank value and suit
    Parameters:
        val (IN) - the face value of the card (explained above)
        suit (IN) - the suit of the card
    Returns: A new instance of the Card struct with an assigned value and suit
    */
    pub fn new(val: i8, suit: &str) -> Self {
        let suit = match suit {
            "s" => "Spade",
            "h" => "Heart",
            "c" => "Club",
            "d" => "Diamond",
            _ => suit,
        };

        Self {
            value: val,
            suit: String::from(suit),
        }
    }
    /*
    Function: get_value()
    Purpose: Get the face value of a card
    Parameters: N/A
    Returns: i8 (signed 8-bit integer) representing the face value of the card
    */
    pub fn get_value(&self) -> i8 {
        self.value
    }

    /*
    Function: get_suit()
    Purpose: Get the suit of a card
    Parameters: N/A
    Returns: String representing the suit of a card
    */
    pub fn get_suit(&self) -> &str {
        self.suit.as_str()
    }

    /*
    Function: get_face()
    Purpose: Get the face value of a card given its rank value
    Parameters: N/A
    Returns: String representing the value of a card if it is a face card and a blank string otherwise
    */
    pub fn get_face(&self) -> String {
        match self.value {
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            14 => String::from("A"),
            _ => self.value.to_string(),
        }
    }
    pub fn get_value_raw(&self) -> usize {
        let multi: i8 = match self.suit.as_str() {
            "Club" => 0,
            "Diamond" => 1,
            "Heart" => 2,
            "Spade" => 3,
            &_ => 99,
        };
        let rtr = self.value - 2 + (multi * 13);
        rtr as usize
    }
}

//override the subtract operator for a card so that we can perform arithmetic on card face values
//used when checking for straights
impl Sub for Card {
    type Output = i8;

    fn sub(self, rhs: Self) -> i8 {
        self.value - rhs.value
    }
}

/*
    the next four functions are overload the == operator for a Card so that we may check if two cards' face values are equal
    we use this when checking for triples
*/
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
