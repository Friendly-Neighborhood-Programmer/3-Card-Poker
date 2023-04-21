use crate::card::Card;

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
            
        });
    }
}


struct CardImage {
    texture: Option<egui::TextureHandle>,
}

impl CardImage {
    fn new() -> Self {
        Self {
            texture: None,
        }
    }

    fn genCard(&mut self, ui: &mut egui::Ui, card: Card, topLeft: egui::Pos2, bottomRight: egui::Pos2) { //will add stuff like adding the rect and card number
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // suit image load.
            let mut path = String::from(card.get_suit());
            path.push_str(".png");
            let _face = card.get_face();
            
            //rect for the card body 
            let rect = egui::Rect::from_min_max(
                egui::Pos2 {
                    x: (40.0),
                    y: (40.0),
                },
                egui::Pos2 {
                    x: (140.0),
                    y: (180.0),
                },
            );
            let rounding = egui::Rounding::same(5.0);
            let fill_color = egui::Color32::from_rgb(245,240,240);
            let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0,0,0));
            ui.painter().rect(rect, rounding, fill_color, stroke);
            //let mut image = CardImage::new().genCard(ui, );

            ui.ctx().load_texture(
                path,
                egui::ColorImage::example(),
                Default::default()
            )
        });

        // Show the image:
        //ui.add(egui::Image::new(texture, texture.size_vec2()));

        // Shorter version:
        ui.image(texture, texture.size_vec2());
    }
}