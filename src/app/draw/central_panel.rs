use eframe::egui::{self, RichText, Sense, Vec2, Widget};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn draw_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::from("Reading Area").strong());

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.allocate_at_least(
                    Vec2 {
                        x: ui.available_width(),
                        y: 0.0,
                    },
                    Sense::empty(),
                );

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.5, 3.0);

                    // ! Try to remove clone here
                    for (i, token) in self.paragraph.clone().iter().enumerate() {
                        if egui::Label::new(token.clone())
                            .sense(egui::Sense::click())
                            .ui(ui)
                            .clicked()
                        {
                            self.select_word(i);
                        }
                    }
                })
            });
        });
    }
}
