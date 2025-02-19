use eframe::egui::{self, Color32, Modifiers, RichText, Sense, Ui, Vec2};
use ringbuf::traits::{Consumer, Observer};

use crate::{
    app::{text_utils, MyEguiApp},
    database::WordStatus,
};

pub mod language;
use language::Language;
mod translate_pop_up;
pub use translate_pop_up::TranslatePopUp;

impl MyEguiApp {
    pub fn draw_side_panel(&mut self, ctx: &egui::Context) {
        const SIDE_PANEL_WIDTH: f32 = 200.0;

        egui::SidePanel::right("Info Panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Dictionary");

                self.open_dictionary_button(ui);
                self.dictionary_pop_up(ctx);
                self.word_stats(ui);

                ui.separator();
                self.open_translate_button(ui);
                self.translate_pop_up(ctx);

                ui.separator();
                ui.heading("History");

                self.translate_history_buttons(ui);
                self.draw_translate_history(ui);
            });
    }

    fn open_dictionary_button(&mut self, ui: &mut Ui) {
        if ui
            .button(if self.dictionary_open {
                "Close Dictionary"
            } else {
                "Open Dictionary"
            })
            .clicked()
        {
            self.toggle_dictionary_pop_up(ui.ctx());
        }
    }

    pub fn toggle_dictionary_pop_up(&mut self, ctx: &egui::Context) {
        self.dictionary_open = !self.dictionary_open;
        if self.dictionary_open {
            ctx.memory_mut(|mem| mem.request_focus(self.search_id));
        }
    }

    fn dictionary_pop_up(&mut self, ctx: &egui::Context) {
        egui::Window::new("Dictionary")
            .open(&mut self.dictionary_open)
            .resizable(true)
            .max_width(200.0)
            .max_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");

                    egui::TextEdit::singleline(&mut self.search_text)
                        .id(self.search_id)
                        .show(ui);

                    if ui.button("❌").clicked() {
                        self.search_text = String::new();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Filter:");

                    if ui.selectable_label(self.search_filter.0, "❌").clicked() {
                        self.search_filter.0 = !self.search_filter.0;
                    }
                    if ui
                        .selectable_label(
                            self.search_filter.1,
                            RichText::from("📖").color(Color32::YELLOW),
                        )
                        .clicked()
                    {
                        self.search_filter.1 = !self.search_filter.1;
                    }
                    if ui
                        .selectable_label(
                            self.search_filter.2,
                            RichText::from("✅").color(Color32::GREEN),
                        )
                        .clicked()
                    {
                        self.search_filter.2 = !self.search_filter.2;
                    }

                    ui.add_space(10.0);
                    ui.label("Hide:");
                    ui.checkbox(&mut self.hide_translated, "");
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.allocate_at_least(
                        Vec2 {
                            x: ui.available_width(),
                            y: 0.0,
                        },
                        Sense::empty(),
                    );

                    // ? maybe move this to SQL
                    let mut status_filter = Vec::new();
                    if self.search_filter.0 {
                        status_filter.push(WordStatus::NotAWord);
                    }
                    if self.search_filter.1 {
                        status_filter.push(WordStatus::Learning);
                    }
                    if self.search_filter.2 {
                        status_filter.push(WordStatus::Mastered);
                    }

                    for t in self
                        .database
                        .get_by_search(&self.search_text)
                        .iter()
                        .filter(|t| status_filter.contains(&t.status))
                        .rev()
                    {
                        ui.horizontal(|ui| {
                            match t.status {
                                WordStatus::NotAWord => ui.label("❌"),
                                WordStatus::Learning => {
                                    ui.label(RichText::from("📖").color(Color32::YELLOW))
                                }
                                WordStatus::Mastered => {
                                    ui.label(RichText::from("✅").color(Color32::GREEN))
                                }
                            };

                            ui.label(&t.from);
                            if !self.hide_translated {
                                ui.label("-");
                                ui.label(&t.to);
                            }
                        });
                    }
                })
            });
    }

    fn word_stats(&mut self, ui: &mut Ui) {
        // ? Maybe make this better by iterating only once
        let learning = self.database.count_by_status(WordStatus::Learning);
        let mastered = self.database.count_by_status(WordStatus::Mastered);

        ui.label(format!("learning: {learning}"));
        ui.label(format!("mastered: {mastered}"));
    }

    fn translate_history_buttons(&mut self, ui: &mut Ui) {
        if !self.translate_history.is_empty() {
            ui.horizontal(|ui| {
                if ui.button("Mark Mastered").clicked() {
                    self.update_last_words_status(WordStatus::Mastered);
                }

                if ui.button("Not A Word").clicked() {
                    self.update_last_words_status(WordStatus::NotAWord);
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
            if let Some((from, to)) = iter.next() {
                ui.label(RichText::from(format!("{from} - {to}")).color(Color32::YELLOW));
            }

            for (from, to) in iter {
                ui.label(format!("{from} - {to}"));
            }
        });
    }

    pub fn update_last_words_status(&mut self, status: WordStatus) {
        let from = &self.translate_history.last().unwrap().0;

        self.database.update_status_by_from(from, status);

        self.get_history_entry();
    }

    fn open_translate_button(&mut self, ui: &mut Ui) {
        if ui
            .button(if self.translate_pop_up.open {
                "Close Translate"
            } else {
                "Open Translate"
            })
            .clicked()
        {
            self.toggle_translate_pop_up(ui.ctx());
        }
    }

    pub fn toggle_translate_pop_up(&mut self, ctx: &egui::Context) {
        self.translate_pop_up.open = !self.translate_pop_up.open;
        if self.translate_pop_up.open {
            ctx.memory_mut(|mem| mem.request_focus(self.translate_pop_up.id));
        }
    }

    fn translate_pop_up(&mut self, ctx: &egui::Context) {
        let pop_up = &mut self.translate_pop_up;

        egui::Window::new("Translate")
            .open(&mut pop_up.open)
            .resizable(true)
            .max_width(200.0)
            .max_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt(1)
                        .selected_text(language::language_to_string(pop_up.language_from))
                        .show_ui(ui, |ui| {
                            let languages =
                                [Language::English, Language::German, Language::Italian];
                            for language in languages {
                                ui.selectable_value(
                                    &mut pop_up.language_from,
                                    language,
                                    language::language_to_string(language),
                                );
                            }
                        });

                    if ui.button("🔀").clicked() {
                        std::mem::swap(&mut pop_up.language_from, &mut pop_up.language_to);
                        std::mem::swap(&mut pop_up.text_from, &mut pop_up.text_to);
                    }

                    egui::ComboBox::from_id_salt(2)
                        .selected_text(language::language_to_string(pop_up.language_to))
                        .show_ui(ui, |ui| {
                            let languages =
                                [Language::English, Language::German, Language::Italian];
                            for language in languages {
                                ui.selectable_value(
                                    &mut pop_up.language_to,
                                    language,
                                    language::language_to_string(language),
                                );
                            }
                        });
                });

                egui::TextEdit::multiline(&mut pop_up.text_from)
                    .id(pop_up.id)
                    .show(ui);

                let keyboard_shortcut =
                    ui.input_mut(|i| i.consume_key(Modifiers::CTRL, egui::Key::Enter));
                let translate_button = egui::Button::new("Translate");
                let button_press = ui
                    .add_sized([ui.available_width(), 0.0], translate_button)
                    .clicked();
                if keyboard_shortcut || button_press {
                    pop_up.text_to = text_utils::translate_text(
                        &pop_up.text_from,
                        pop_up.language_from,
                        pop_up.language_to,
                    );
                }

                let mut temp = pop_up.text_to.clone();
                ui.text_edit_multiline(&mut temp);
            });
    }
}
