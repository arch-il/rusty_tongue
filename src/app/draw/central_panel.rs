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

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.5, 3.0);

                    // ! Try to remove clone here
                    for token in self.paragraph.clone().iter() {
                        let label_button = egui::Label::new(token.clone())
                            .sense(egui::Sense::click())
                            .ui(ui);

                        if label_button.clicked() {
                            let word = text_utils::token_to_word(token.text());

                            self.record_translate_history(&word);

                            self.set_entry_pop_up_word(&word);

                            if self.database.get_by_word(&word).is_none() {
                                self.database.insert(&Translation {
                                    word,
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
