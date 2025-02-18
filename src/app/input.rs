use std::collections::HashSet;

use eframe::egui::{self, Key};
use ringbuf::traits::Observer;

use crate::database::WordStatus;

use super::MyEguiApp;

impl MyEguiApp {
    pub fn input(&mut self, ctx: &egui::Context) {
        if ctx.memory(|mem| mem.focused()).is_some() {
            return;
        }

        let keys_down = ctx.input(|input_state| input_state.keys_down.clone());

        let new_keys = self
            .prev_keys_down
            .difference(&keys_down)
            .cloned()
            .collect::<HashSet<Key>>();

        for key in new_keys.iter() {
            match key {
                Key::ArrowDown => {
                    self.index += 1;
                    self.get_history_entry();
                }
                Key::ArrowUp => {
                    if self.index != 0 {
                        self.index -= 1;
                    }
                    self.get_history_entry();
                }

                Key::M => {
                    if !self.translate_history.is_empty() {
                        self.update_last_words_status(WordStatus::Mastered);
                    }
                }
                Key::N => {
                    if !self.translate_history.is_empty() {
                        self.update_last_words_status(WordStatus::NotAWord);
                    }
                }

                Key::D => {
                    self.toggle_dictionary_pop_up(ctx);
                }
                Key::L => {
                    ctx.memory_mut(|mem| mem.request_focus(self.location_id));
                }

                Key::Escape => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                _ => (),
            }
        }

        self.prev_keys_down = keys_down;
    }
}
