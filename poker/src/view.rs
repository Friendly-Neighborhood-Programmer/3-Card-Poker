use crate::{
    card::Card,
    deck::Deck,
    hand::{get_hand, HandType},
    player::Player,
};
use eframe::egui;
use egui::{vec2, Color32, FontId, Pos2, Response, RichText};
use egui_extras::RetainedImage;

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
    //states
    is_game: bool,
    rasied: bool,
    dealt: bool,
    //message to user
    message: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut card_vec: Vec<RetainedImage> = Vec::new();
        for i in 0..52 {
            //decides suit by dividing i by 13 and getting the result - 0...12 are spades, 13...25 are hearts, 26...38 are clubs, and 39...52 are diamonds
            let img: RetainedImage = match i {
                //CLUBS
                0 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_2.png",
                    include_bytes!("../images/cards/Club_2.png"),
                )
                .unwrap(),
                1 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_3.png",
                    include_bytes!("../images/cards/Club_3.png"),
                )
                .unwrap(),
                2 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_4.png",
                    include_bytes!("../images/cards/Club_4.png"),
                )
                .unwrap(),
                3 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_5.png",
                    include_bytes!("../images/cards/Club_5.png"),
                )
                .unwrap(),
                4 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_6.png",
                    include_bytes!("../images/cards/Club_6.png"),
                )
                .unwrap(),
                5 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_7.png",
                    include_bytes!("../images/cards/Club_7.png"),
                )
                .unwrap(),
                6 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_8.png",
                    include_bytes!("../images/cards/Club_8.png"),
                )
                .unwrap(),
                7 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_9.png",
                    include_bytes!("../images/cards/Club_9.png"),
                )
                .unwrap(),
                8 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_10.png",
                    include_bytes!("../images/cards/Club_10.png"),
                )
                .unwrap(),
                9 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_11.png",
                    include_bytes!("../images/cards/Club_11.png"),
                )
                .unwrap(),
                10 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_12.png",
                    include_bytes!("../images/cards/Club_12.png"),
                )
                .unwrap(),
                11 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_13.png",
                    include_bytes!("../images/cards/Club_13.png"),
                )
                .unwrap(),
                12 => RetainedImage::from_image_bytes(
                    "../images/cards/Club_14.png",
                    include_bytes!("../images/cards/Club_14.png"),
                )
                .unwrap(),
                //DIAMONDS
                13 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_2.png",
                    include_bytes!("../images/cards/Diamond_2.png"),
                )
                .unwrap(),
                14 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_3.png",
                    include_bytes!("../images/cards/Diamond_3.png"),
                )
                .unwrap(),
                15 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_4.png",
                    include_bytes!("../images/cards/Diamond_4.png"),
                )
                .unwrap(),
                16 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_5.png",
                    include_bytes!("../images/cards/Diamond_5.png"),
                )
                .unwrap(),
                17 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_6.png",
                    include_bytes!("../images/cards/Diamond_6.png"),
                )
                .unwrap(),
                18 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_7.png",
                    include_bytes!("../images/cards/Diamond_7.png"),
                )
                .unwrap(),
                19 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_8.png",
                    include_bytes!("../images/cards/Diamond_8.png"),
                )
                .unwrap(),
                20 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_9.png",
                    include_bytes!("../images/cards/Diamond_9.png"),
                )
                .unwrap(),
                21 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_10.png",
                    include_bytes!("../images/cards/Diamond_10.png"),
                )
                .unwrap(),
                22 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_11.png",
                    include_bytes!("../images/cards/Diamond_11.png"),
                )
                .unwrap(),
                23 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_12.png",
                    include_bytes!("../images/cards/Diamond_12.png"),
                )
                .unwrap(),
                24 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_13.png",
                    include_bytes!("../images/cards/Diamond_13.png"),
                )
                .unwrap(),
                25 => RetainedImage::from_image_bytes(
                    "../images/cards/Diamond_14.png",
                    include_bytes!("../images/cards/Diamond_14.png"),
                )
                .unwrap(),
                //HEARTS
                26 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_2.png",
                    include_bytes!("../images/cards/Heart_2.png"),
                )
                .unwrap(),
                27 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_3.png",
                    include_bytes!("../images/cards/Heart_3.png"),
                )
                .unwrap(),
                28 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_4.png",
                    include_bytes!("../images/cards/Heart_4.png"),
                )
                .unwrap(),
                29 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_5.png",
                    include_bytes!("../images/cards/Heart_5.png"),
                )
                .unwrap(),
                30 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_6.png",
                    include_bytes!("../images/cards/Heart_6.png"),
                )
                .unwrap(),
                31 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_7.png",
                    include_bytes!("../images/cards/Heart_7.png"),
                )
                .unwrap(),
                32 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_8.png",
                    include_bytes!("../images/cards/Heart_8.png"),
                )
                .unwrap(),
                33 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_9.png",
                    include_bytes!("../images/cards/Heart_9.png"),
                )
                .unwrap(),
                34 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_10.png",
                    include_bytes!("../images/cards/Heart_10.png"),
                )
                .unwrap(),
                35 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_11.png",
                    include_bytes!("../images/cards/Heart_11.png"),
                )
                .unwrap(),
                36 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_12.png",
                    include_bytes!("../images/cards/Heart_12.png"),
                )
                .unwrap(),
                37 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_13.png",
                    include_bytes!("../images/cards/Heart_13.png"),
                )
                .unwrap(),
                38 => RetainedImage::from_image_bytes(
                    "../images/cards/Heart_14.png",
                    include_bytes!("../images/cards/Heart_14.png"),
                )
                .unwrap(),
                //SPADES
                39 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_2.png",
                    include_bytes!("../images/cards/Spade_2.png"),
                )
                .unwrap(),
                40 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_3.png",
                    include_bytes!("../images/cards/Spade_3.png"),
                )
                .unwrap(),
                41 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_4.png",
                    include_bytes!("../images/cards/Spade_4.png"),
                )
                .unwrap(),
                42 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_5.png",
                    include_bytes!("../images/cards/Spade_5.png"),
                )
                .unwrap(),
                43 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_6.png",
                    include_bytes!("../images/cards/Spade_6.png"),
                )
                .unwrap(),
                44 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_7.png",
                    include_bytes!("../images/cards/Spade_7.png"),
                )
                .unwrap(),
                45 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_8.png",
                    include_bytes!("../images/cards/Spade_8.png"),
                )
                .unwrap(),
                46 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_9.png",
                    include_bytes!("../images/cards/Spade_9.png"),
                )
                .unwrap(),
                47 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_10.png",
                    include_bytes!("../images/cards/Spade_10.png"),
                )
                .unwrap(),
                48 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_11.png",
                    include_bytes!("../images/cards/Spade_11.png"),
                )
                .unwrap(),
                49 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_12.png",
                    include_bytes!("../images/cards/Spade_12.png"),
                )
                .unwrap(),
                50 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_13.png",
                    include_bytes!("../images/cards/Spade_13.png"),
                )
                .unwrap(),
                51 => RetainedImage::from_image_bytes(
                    "../images/cards/Spade_14.png",
                    include_bytes!("../images/cards/Spade_14.png"),
                )
                .unwrap(),
                _ => RetainedImage::from_image_bytes(
                    "../images/cards/Back.png",
                    include_bytes!("../images/cards/Back.png"),
                )
                .unwrap(),
            };
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
            rasied: false,
            dealt: false,
            message: String::from("Good luck!"),
        }
    }

    // start the round and deal the player their cards
    pub fn deal(&mut self) {
        self.rasied = false;
        self.player.empty();
        self.player.fill_from(&mut self.deck);
        self.dealer.empty_deck();
        self.dealer.fill_from_deck(&mut self.deck);

        // if the main deck empties, refill it
        if self.deck.get_size() == 0 {
            self.deck.fill_standard();
            self.deck.shuffle();
        }

        self.place_ante();
        self.place_pair_plus();
    }

    fn place_ante(&mut self) {
        self.player.money -= self.ante;
    }

    fn place_pair_plus(&mut self) {
        self.player.money -= self.pair_plus;
    }

    pub fn fold(&mut self) {}

    pub fn play(&mut self) {
        self.rasied = true;
        self.player.money -= self.ante;
        self.ante *= 2;
    }

    pub fn check_dealer_qualify(&mut self) -> bool {
        match get_hand(&self.dealer, None) {
            HandType::HighJack => false,
            HandType::Other => false,
            _ => true,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(" ");
            if self.is_game {
                //if the game is in play -> game ui
                //think of these ui.horizontal calls as divs, just a way to structure the screen
                //Divs are as follows: dealers cards, players cards
                ui.horizontal_top(|ui| {
                    if self.player.money < 99 {
                        ui.label("Dealer          ");
                    } else if self.player.money > 999 {
                        ui.label("Dealer            ");
                    } else {
                        ui.label("Dealer           ");
                    }
                    if self.rasied {
                        let dealer_card =
                            &self.card_images[(&self.dealer.get_cards()[0]).get_value_raw()];
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card =
                            &self.card_images[(&self.dealer.get_cards()[1]).get_value_raw()];
                        ui.add(egui::Image::new(
                            dealer_card.texture_id(ctx),
                            dealer_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let dealer_card =
                            &self.card_images[(&self.dealer.get_cards()[2]).get_value_raw()];
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
                    ui.label(format!("Player | ${}", self.player.money));
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
                            &self.card_images[(&self.player.get_cards()[0]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card =
                            &self.card_images[(&self.player.get_cards()[1]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                        let player_card =
                            &self.card_images[(&self.player.get_cards()[2]).get_value_raw()];
                        ui.add(egui::Image::new(
                            player_card.texture_id(ctx),
                            player_card.size_vec2() / vec2(2.0, 2.0),
                        ));
                    }
                });
                ui.horizontal_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("Ante Bet: ${}", self.ante));
                        if ui.button("-").clicked() && self.ante > 25 && !self.dealt {
                            self.ante -= 1;
                        }
                        if ui.button("+").clicked() && !self.dealt {
                            self.ante += 1;
                        }
                    });
                    ui.label("          ");
                    ui.horizontal_centered(|ui| {
                        if ui.button("Raise").clicked() && !self.rasied {
                            self.play()
                        }
                        else if self.rasied {
                            self.message = format!("Can not raise again"); 
                        }
                        if ui.button("Fold").clicked() && !self.rasied {
                            self.fold()
                        }
                        else if self.rasied {
                            self.message = format!("Can not fold after raising"); 
                        }
                        if ui.button("Deal").clicked()
                            && self.player.money >= 2 * self.ante
                            && !self.rasied
                        {
                            self.dealt = true;
                            self.deal();
                        } else if self.player.money <= 2 * self.ante {
                            self.message = format!("You do not have enough money to make that bet");
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
                    ui.label("Welcome to the Casino... That exclusivly plays 3 card poker");
                    ui.label(" ");
                    if ui.button("Start Game").clicked() {
                        self.is_game = true;
                        self.deal();
                    }
                });
            }
        });
    }
}

//fn gen_card(&mut self, card: &Card, top_left: egui::Pos2, bottom_right: egui::Pos2) -> ImageData {}

struct ImageData {
    image: String,
    texture: egui::TextureHandle,
    suit_color: egui::Color32,
    rect: egui::Rect,
    rounding: egui::Rounding,
    fill_color: egui::Color32,
    stroke: egui::Stroke,
}

//Graveyard for storage perposes
// player_images: Vec::new(),
// dealer_images: Vec::new(),
// club_image: RetainedImage::from_image_bytes(
//     "../images/Club.png",
//     include_bytes!("../images/Club.png"),
// )
// .unwrap(),
// diamond_image: RetainedImage::from_image_bytes(
//     "../images/Diamond.png",
//     include_bytes!("../images/Diamond.png"),
// )
// .unwrap(),
// heart_image: RetainedImage::from_image_bytes(
//     "../images/Heart.png",
//     include_bytes!("../images/Heart.png"),
// )
// .unwrap(),
// spade_image: RetainedImage::from_image_bytes(
//     "../images/Spade.png",
//     include_bytes!("../images/Spade.png"),
// )
// .unwrap(),

//will add stuff like adding the rect and card number
// suit image load.
// let mut image = match card.get_suit() {
//     "Spade" => self.spade_image,
//     "Club" => self.club_image,
//     "Diamond" => self.diamond_image,
//     "Heart" => self.heart_image,
// };

//rect for the card body
// let rect = egui::Rect::from_min_max(top_left, bottom_right);
// let rounding = egui::Rounding::same(5.0);
// let fill_color = egui::Color32::from_rgb(0, 0, 0);
// let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 0, 0));
// ui.painter().rect(rect, rounding, fill_color, stroke);

//color for text
// let suit_color = match card.get_suit() {
//     "Spade" => egui::Color32::from_rgb(255, 255, 255),
//     "Club" => egui::Color32::from_rgb(255, 255, 255),
//     "Diamond" => egui::Color32::from_rgb(255, 0, 0),
//     "Heart" => egui::Color32::from_rgb(255, 0, 0),
//     _ => egui::Color32::from_rgb(0, 0, 255), //For error detection and testing ONLY
// };
// ui.label(RichText::new(card.get_face()).color(suit_color));

// Show the image:
//ui.add(egui::Image::new(texture, texture.size_vec2()));

// Shorter version:
//ui.image(&tex, tex.size_vec2());

// ImageData {
//     image: (image),
//     texture: (tex),
//     suit_color: (suit_color),
//     rect: (rect),
//     rounding: (rounding),
//     fill_color: (fill_color),
//     stroke: (stroke),
// }
