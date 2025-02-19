use eframe::egui::{self, Modifiers, Ui};

use crate::app::{text_utils, MyEguiApp};

use super::{language, Language};

pub struct TranslatePopUp {
    pub open: bool,
    pub id: egui::Id,
    pub language_from: Language,
    pub language_to: Language,
    pub text_from: String,
    pub text_to: String,
}

impl TranslatePopUp {
    pub fn new() -> Self {
        Self {
            open: false,
            id: egui::Id::new("translate id"),
            language_from: Language::German,
            language_to: Language::English,
            text_from: String::new(),
            text_to: String::new(),
        }
    }
}

impl MyEguiApp {
    pub fn open_translate_button(&mut self, ui: &mut Ui) {
        if ui
            .selectable_label(self.translate_pop_up.open, "Open Translate")
            .clicked()
        {
            self.toggle_translate_pop_up(ui.ctx());
        }

        if ui.button("Translate Paragraph").clicked() {
            self.translate_paragraph(ui.ctx());
        }
    }

    pub fn toggle_translate_pop_up(&mut self, ctx: &egui::Context) {
        self.translate_pop_up.open = !self.translate_pop_up.open;
        if self.translate_pop_up.open {
            ctx.memory_mut(|mem| mem.request_focus(self.translate_pop_up.id));
        }
    }

    pub fn translate_pop_up(&mut self, ctx: &egui::Context) {
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

    pub fn translate_paragraph(&mut self, ctx: &egui::Context) {
        if !self.translate_pop_up.open {
            self.toggle_translate_pop_up(ctx);
        }

        self.translate_pop_up.text_from = self.lines[self.index].clone();
        self.translate_pop_up.text_to = text_utils::translate_text(
            &self.translate_pop_up.text_from,
            self.translate_pop_up.language_from,
            self.translate_pop_up.language_to,
        );
    }
}
