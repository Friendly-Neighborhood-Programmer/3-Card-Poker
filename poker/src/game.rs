use crate::{
    deck::Deck,
    hand::{get_hand, HandType},
    player::Player,
};
use eframe::egui;
use egui::vec2;
use egui_extras::RetainedImage;
use std::{cmp::Ordering::*, fs::File, io::Read, path::PathBuf};

//initializes the app, sets up view
pub fn init_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "3 Card Poker",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

struct App {
    //cards
    deck: Deck,
    player: Player,
    dealer: Deck,
    //money
    ante: usize,
    pair_plus: usize,
    //suit images
    card_images: Vec<RetainedImage>,
    card_back: RetainedImage,
    //states //TODO: change to enum for states
    is_game: bool,
    raised: bool,
    dealt: bool,
    //message to user
    message: String,
}

impl App {
    //maps images to card values
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut card_vec: Vec<RetainedImage> = Vec::new();

        for card_number in 0..52 {
            let match_suit = |s: i32| -> &str {
                match s {
                    0 => "Spade",
                    1 => "Heart",
                    2 => "Club",
                    3 => "Diamond",
                    _ => "INVALID",
                }
            };

            let path = format!(
                "./images/cards/{}_{}.png",
                match_suit(card_number / 13), // the suit
                card_number % 13 + 2 // the card's value
            );

            let mut buffer = vec![];
            File::open(PathBuf::from(path.to_string()))
                .expect("could not find card image file")
                .read_to_end(&mut buffer)
                .expect("problem while reading image");
            
            let img = RetainedImage::from_image_bytes(path, &buffer[..]).unwrap();
            card_vec.push(img);
        }

        let mut d = Deck::new(52);
        d.fill_standard();
        d.shuffle();
        Self {
            deck: d,
            player: Player::new(),
            dealer: Deck::new(3),
            ante: 25,
            pair_plus: 5,
            card_images: card_vec,
            card_back: RetainedImage::from_image_bytes(
                "../images/cards/Back.png",
                include_bytes!("../images/cards/Back.png"),
            )
            .unwrap(),
            is_game: false,
            raised: false,
            dealt: false,
            message: String::from("Good luck!"),
        }
    }

    // start the round and deal the player their cards
    pub fn new_cards(&mut self) {
        self.raised = false;
        self.player.empty();
        self.player.fill_from(&mut self.deck);
        self.dealer.empty_deck();
        self.dealer.fill_from_deck(&mut self.deck);

        // if the main deck empties, refill it
        if self.deck.get_size() == 0 {
            self.deck.fill_standard();
            self.deck.shuffle();
            self.message = "Wow you really love this game".to_string();
        }
    }

    //deal cards
    pub fn deal(&mut self) {
        self.raised = false;
        self.new_cards();
        self.place_ante();
        self.place_pair_plus();
        let hand = match get_hand(&self.player.cards, None) {
            HandType::Other => "less than high card",
            HandType::HighJack => "a Jack High",
            HandType::HighQueen => "a Queen High",
            HandType::HighKing => "a King High",
            HandType::HighAce => "an Ace High",
            HandType::Pair => "a Pair",
            HandType::Flush => "a Flush",
            HandType::Straight => "a Straight",
            HandType::Triple => "a Triple",
            HandType::StraightFlush => "a Straight Flush",
        };
        self.message = format!("{}\nYour hand is {}", self.message, hand);
    }

    fn place_ante(&mut self) {
        self.player.money -= self.ante;
    }

    fn place_pair_plus(&mut self) {
        self.player.money -= self.pair_plus;
    }

    pub fn fold(&mut self) {
        self.message = "You folded".to_string();
        self.dealt = false;
    }

    pub fn play(&mut self) {
        self.raised = true;
        self.player.money -= self.ante;

        if !self.check_dealer_qualify() {
            self.message = format!(
                "The dealer did not qualify, your play wager of ${} has been refunded",
                self.ante
            );
            self.player.money += self.ante;
            match get_hand(&self.player.cards, None).cmp(&get_hand(&self.dealer, None)) {
                Greater => {
                    self.player.money += self.ante * 2;
                    self.message = format!("{}\n Your hand ranked higher then the dealers, your Ante wager of ${} has been payed out",self.message, self.ante);
                }
                _ => {
                    self.player.money += self.ante;
                    self.message = format!("{}\n Your hand did not rank higher then the dealers, your Ante wager of ${} has been refunded out",self.message, self.ante);
                }
            }
        } else {
            match get_hand(&self.player.cards, None).cmp(&get_hand(&self.dealer, None)) {
                Greater => {
                    self.player.money += self.ante * 4;
                    self.message = format!("You won ${}, Congratulations! :)", self.ante * 2);
                }
                Equal => {
                    self.player.money += self.ante * 2;
                    self.message = "You tied with the dealer".to_string();
                }
                Less => {
                    self.message = format!("You lost ${}, better luck next time :(", self.ante * 2);
                }
            };
            let pair_winnings = self.check_pair_plus();
            if pair_winnings > 0 {
                self.message = format!(
                    "{}\n You won ${} in pair plus wager, Congratulations!",
                    self.message, pair_winnings
                );
            } else {
                self.message = format!(
                    "{}\n You lost ${} on your pair plus wager",
                    self.message, self.pair_plus
                );
            }
        }
    }

    pub fn check_dealer_qualify(&self) -> bool {
        get_hand(&self.dealer, None) > HandType::HighJack
    }

    pub fn check_pair_plus(&mut self) -> usize {
        const STRAIGHT_FLUSH_PAYOFF: usize = 40;
        const TRIPLE_PAYOFF: usize = 30;
        const STRAIGHT_PAYOFF: usize = 6;
        const FLUSH_PAYOFF: usize = 3;

        let amount = match get_hand(&self.player.cards, None) {
            HandType::StraightFlush => self.pair_plus * STRAIGHT_FLUSH_PAYOFF,
            HandType::Triple => self.pair_plus * TRIPLE_PAYOFF,
            HandType::Straight => self.pair_plus * STRAIGHT_PAYOFF,
            HandType::Flush => self.pair_plus * FLUSH_PAYOFF,
            HandType::Pair => self.pair_plus,
            _ => 0,
        };

        if amount == 0 {
            self.player.money -= self.pair_plus;
        } else {
            self.player.money += self.pair_plus;
        }
        amount
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(" ");
            if self.is_game {
                //if the game is in play -> game ui
                //think of these ui.horizontal calls as divs, just a way to structure the screen
                //Divs are as follows: dealers cards, players cards
                ui.horizontal_top(|ui| {
                    ui.label("Dealer          ");
                    if self.raised {
                        let dealer_card =
                            &self.card_images[(self.dealer.get_cards()[0]).get_value_raw()];
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card =
                            &self.card_images[(self.dealer.get_cards()[1]).get_value_raw()];
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card =
                            &self.card_images[(self.dealer.get_cards()[2]).get_value_raw()];
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                    } else {
                        let dealer_card = &self.card_back;
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card = &self.card_back;
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card = &self.card_back;
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                    }
                    ui.vertical_centered( |ui| {
                        ui.label("STRATEGY");
                        ui.label("
                        FOLD: anytime your hand is worse than Queen, Six, Four (Q-6-4).");
                        ui.label("PLAY: with all other hands. In other words, whenever your hand is Q-6-4 or better.");
                    });
                });
                ui.horizontal_top(|ui| {
                    ui.label(format!("Player ${} ", self.player.money));
                    if !self.dealt {
                        let player_card = &self.card_back;
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card = &self.card_back;
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card = &self.card_back;
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                    } else {
                        let player_card =
                            &self.card_images[(self.player.get_cards()[0]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card =
                            &self.card_images[(self.player.get_cards()[1]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card =
                            &self.card_images[(self.player.get_cards()[2]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                    }
                    ui.vertical_centered( |ui| {
                        ui.label(self.message.clone());
                    });
                });
                ui.horizontal_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("           Ante Bet: ${}", self.ante));
                        if ui.button("-").clicked() && self.ante > 25 && !self.dealt {
                            self.ante -= 1;
                        }
                        if ui.button("+").clicked() && !self.dealt {
                            self.ante += 1;
                        }
                    });
                    ui.label("          ");
                    ui.horizontal_centered(|ui| {
                        if ui.button("Raise").clicked() {
                            if !self.raised && self.dealt {
                            self.play();
                        }
                            else if self.raised{
                                self.message = "Can not raise again".to_string(); 
                            }
                            else if !self.dealt{
                                self.message = "Can not raise before dealing".to_string(); 
                            }
                        }
                        if ui.button("Fold").clicked() {
                            if self.dealt && !self.raised {
                            self.fold();
                            }
                            else if self.raised{
                                self.message = "Can not fold after raising".to_string(); 
                            }
                            else if !self.dealt{
                                self.message = "Can not fold before dealing".to_string(); 
                            }
                        }
                        if ui.button("Deal").clicked() {
                            if self.player.money >= 2 * self.ante && !self.raised && !self.dealt {
                            self.dealt = true;
                            self.deal();
                            }
                            else if self.raised{
                                self.message = "You can not deal after raising, click 'next hand' to start another round".to_string();  
                            }
                            else if self.dealt{
                                self.message = "You can not deal after already dealing".to_string();  
                            }
                            else {
                                self.message = "You do not have enough money to make that bet".to_string();  
                            }
                        }
                        if ui.button("Next hand").clicked() {
                            if self.raised && self.dealt {
                                self.raised = false;
                                self.dealt = false;
                                self.message = "A new round begins...".to_string();
                                self.new_cards();
                            }
                            else {
                                self.message = "Can not deal next hand until round is over".to_string(); 
                            }
                        }
                    });
                    ui.label("          ");
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("Pair Plus Bet: ${}", self.pair_plus));
                        if ui.button("-").clicked() && self.pair_plus > 0 && !self.dealt {
                            self.pair_plus -= 1;
                        }
                        if ui.button("+").clicked() && !self.dealt {
                            self.pair_plus += 1;
                        }
                    });
                });
            } else {
                //if the game is not in play -> main menu
                ui.vertical_centered(|ui| {
                    ui.label(" ");
                    ui.label("Welcome to the Casino... That exclusively plays 3 card poker");
                    ui.label(" ");
                    if ui.button("Start Game").clicked() {
                        self.is_game = true;
                        self.new_cards();
                    }
                });
            }
        });
    }
}
