#[derive(Debug)]
pub struct Card {
    value: u8,    //Aces run high, value 2..14, with Ace = 14
    suit: String, //Spade, Heart, Club, Diamond
}

impl Card {
    pub fn new(val: u8, suit: &str) -> Self {
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

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_suit(&self) -> &str {
        self.suit.as_str()
    }
}
