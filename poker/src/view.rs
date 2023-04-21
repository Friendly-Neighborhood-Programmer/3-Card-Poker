use crate::card::Card;
use eframe::egui;
use egui::{RichText, FontId, Color32};

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

#[derive(Default)]
struct App {
    times_clicked: u32,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Kill me {} times", self.times_clicked));
            if ui.button("click me").clicked() {
                self.times_clicked += 1;
            }
            //let mut image = CardImage::new().genCard(ui, );
        });
    }
}

struct CardImage {
    texture: Option<egui::TextureHandle>,
}

impl CardImage {
    fn new() -> Self {
        Self { texture: None }
    }

    fn gen_card(
        &mut self,
        ui: &mut egui::Ui,
        card: Card,
        top_left: egui::Pos2,
        bottom_right: egui::Pos2,
    ) {
        //will add stuff like adding the rect and card number
        // suit image load.
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            let mut path = String::from(card.get_suit());
            path.push_str(".png");
            let _face = card.get_face();
            ui.ctx()
                .load_texture(path, egui::ColorImage::example(), Default::default())
        });

        //rect for the card body
        let rect = egui::Rect::from_min_max(top_left, bottom_right);
        let rounding = egui::Rounding::same(5.0);
        let fill_color = egui::Color32::from_rgb(0, 0, 0);
        let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 0, 0));
        ui.painter().rect(rect, rounding, fill_color, stroke);

        //color for text
        let suit_color = match card.get_suit() {
            "Spade" => egui::Color32::from_rgb(255, 255, 255),
            "Club" => egui::Color32::from_rgb(255, 255, 255),
            "Diamond" => egui::Color32::from_rgb(255, 0, 0),
            "Heart" => egui::Color32::from_rgb(255, 0, 0),
            _ => egui::Color32::from_rgb(0, 0, 255), //For error detection and testing ONLY
        };
        ui.label(RichText::new(card.get_face()).color(suit_color));

        // Show the image:
        //ui.add(egui::Image::new(texture, texture.size_vec2()));

        // Shorter version:
        ui.image(texture, texture.size_vec2());
    }
}
