pub struct Card {
    value: u8,    //Aces run high, value 2..14, with Ace = 14
    suit: String, //Spade, Heart, Club, Diamond
}

impl Card {
    pub fn new(v: u8, s: &str) -> Self {
        let s = match s {
            "s" => "Spade",
            "h" => "Heart",
            "c" => "Club",
            "d" => "Diamond",
            &_ => s,
        };

        Self {
            value: v,
            suit: String::from(s),
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_suit(&self) -> &str {
        self.suit.as_str()
    }
}
