use eframe::egui;
use ringbuf::traits::Observer;

use crate::database::WordStatus;

use super::MyEguiApp;

impl MyEguiApp {
    pub fn input(&mut self, ctx: &egui::Context) {
        ctx.input(|input_state| {
            if input_state.key_pressed(egui::Key::ArrowDown) {
                self.index += 1;
                self.get_history_entry();
            }

            if input_state.key_pressed(egui::Key::ArrowUp) && self.index != 0 {
                self.index -= 1;
                self.get_history_entry();
            }

            if !self.translate_history.is_empty() {
                if input_state.key_pressed(egui::Key::M) {
                    self.update_last_words_status(WordStatus::Mastered);
                }

                if input_state.key_pressed(egui::Key::N) {
                    self.update_last_words_status(WordStatus::NotAWord);
                }
            }

            if input_state.key_pressed(egui::Key::D) {
                self.dictionary_open = !self.dictionary_open;
            }
        });
    }
}
