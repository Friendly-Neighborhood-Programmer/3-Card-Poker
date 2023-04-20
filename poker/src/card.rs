#[derive(Debug, Clone)]

//data structure of a card
pub struct Card {
    //card value is stored as a signed 8-bit integer
    //values 2-10 represent their respective face cards, with 11 = J, 12 = Q, 13 = K, 14 = A (aces high)
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
            &_ => suit,
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
}
