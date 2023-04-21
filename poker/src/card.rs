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

    pub fn get_value(&self) -> i8 {
        self.value
    }

    pub fn get_suit(&self) -> &str {
        self.suit.as_str()
    }

    pub fn get_face(&self) -> String {
        match self.value {
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            14 => String::from("A"),
            _ => self.value.to_string(),
        }
    }
}

impl Sub for Card {
    type Output = i8;

    fn sub(self, rhs: Self) -> i8 {
        self.value - rhs.value
    }
}

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