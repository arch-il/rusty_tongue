use eframe::egui::{self, Sense, Vec2, Widget};

use crate::{
    app::text_utils,
    database::{Translation, WordStatus},
};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn draw_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reading Area");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.allocate_at_least(
                    Vec2 {
                        x: ui.available_width(),
                        y: 0.0,
                    },
                    Sense::empty(),
                );
                // ? Why do I need horizontal with labels?
                ui.horizontal_wrapped(|ui| {
                    // ! Try to remove clone here
                    for token in self.paragraph.clone().iter() {
                        let label_button = egui::Label::new(token.clone())
                            .sense(egui::Sense::click())
                            .ui(ui);

                        if label_button.clicked() {
                            let from = text_utils::token_to_word(token.text());
                            let to = self.translate_text(&from);

                            self.record_translate_history(&from, &to);

                            if self.database.get_by_from(&from).is_none() {
                                self.database.insert(&Translation {
                                    from,
                                    to,
                                    status: WordStatus::Learning,
                                });

                                self.get_history_entry();
                            }
                        }
                    }
                })
            });
        });
    }
}
