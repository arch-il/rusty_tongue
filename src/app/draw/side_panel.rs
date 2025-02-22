use eframe::egui::{self, Color32, RichText, Sense, Ui, Vec2};
use ringbuf::traits::{Consumer, Observer};

use crate::{app::MyEguiApp, database::WordStatus};

mod dictionary_pop_up;
pub use dictionary_pop_up::DictionaryPopUp;

impl MyEguiApp {
    pub fn draw_side_panel(&mut self, ctx: &egui::Context) {
        const SIDE_PANEL_WIDTH: f32 = 200.0;

        egui::SidePanel::right("Info Panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.heading(RichText::from("Dictionary").strong());

                self.open_dictionary_button(ui);
                self.dictionary_pop_up(ctx);
                self.word_entry_pop_up(ctx);

                self.word_stats(ui);

                ui.separator();
                ui.heading(RichText::from("Translate").strong());

                ui.separator();
                ui.heading(RichText::from("History").strong());

                self.translate_history_buttons(ui);
                self.draw_translate_history(ui);
            });
    }

    fn word_stats(&mut self, ui: &mut Ui) {
        // ? Maybe make this better by iterating only once
        let learning = self.database.count_by_status(WordStatus::Learning);
        let mastered = self.database.count_by_status(WordStatus::Mastered);

        ui.horizontal(|ui| {
            // ui.label(format!("Learning: {learning}"));
            ui.label(RichText::from("Learning:").strong());
            ui.label(learning.to_string());
        });
        ui.horizontal(|ui| {
            ui.label(RichText::from("Mastered:").strong());
            ui.label(mastered.to_string());
        });
    }

    fn translate_history_buttons(&mut self, ui: &mut Ui) {
        if !self.translate_history.is_empty() {
            ui.horizontal(|ui| {
                let translation = self
                    .database
                    .get_by_word(self.translate_history.last().unwrap());

                let status = if let Some(translation) = translation {
                    translation.status
                } else {
                    return;
                };

                ui.label(RichText::from("Status:").strong());

                if ui
                    .selectable_label(
                        status == WordStatus::NotAWord,
                        RichText::from("❌").color(Color32::LIGHT_RED),
                    )
                    .clicked()
                {
                    self.update_last_words_status(WordStatus::NotAWord);
                } else if ui
                    .selectable_label(
                        status == WordStatus::Learning,
                        RichText::from("📖").color(Color32::LIGHT_YELLOW),
                    )
                    .clicked()
                {
                    self.update_last_words_status(WordStatus::Learning);
                } else if ui
                    .selectable_label(
                        status == WordStatus::Mastered,
                        RichText::from("✅").color(Color32::LIGHT_GREEN),
                    )
                    .clicked()
                {
                    self.update_last_words_status(WordStatus::Mastered);
                }
            });
        }
    }

    fn draw_translate_history(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_at_least(
                Vec2 {
                    x: ui.available_width(),
                    y: 0.0,
                },
                Sense::empty(),
            );

            let mut iter = self.translate_history.iter_mut().rev();
            if let Some(word) = iter.next() {
                ui.label(RichText::from(word).strong());
            }

            for word in iter {
                ui.label(word.clone());
            }
        });
    }

    pub fn update_last_words_status(&mut self, status: WordStatus) {
        let from = &self.translate_history.last().unwrap();

        self.database.update_status_by_word(from, status);

        self.get_history_entry();
    }
}
