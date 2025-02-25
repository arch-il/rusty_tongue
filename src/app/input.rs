use std::collections::HashSet;

use eframe::egui::{self, Key};
use ringbuf::traits::Observer;

use crate::{
    app::text_utils,
    database::{Translation, WordStatus},
};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn input(&mut self, ctx: &egui::Context) {
        let keys_down = ctx.input(|input_state| input_state.keys_down.clone());

        if ctx.memory(|mem| mem.focused()).is_some() {
            self.prev_keys_down = keys_down;
            return;
        }

        let new_keys = self
            .prev_keys_down
            .difference(&keys_down)
            .cloned()
            .collect::<HashSet<Key>>();

        for key in new_keys.iter() {
            match key {
                Key::ArrowDown => {
                    self.page_location += 1;
                    self.refresh_page();

                    self.select_word(0);
                }
                Key::ArrowUp => {
                    if self.page_location != 0 {
                        self.page_location -= 1;
                    }
                    self.refresh_page();

                    self.select_word(0);
                }

                Key::ArrowRight => {
                    let mut location = 0;

                    for (i, token) in self
                        .paragraph
                        .iter()
                        .enumerate()
                        .skip(self.word_location + 1)
                    {
                        // ! fix this. i am getting token information for second time
                        let word = text_utils::token_to_word(token.text());
                        let entry = self.database.get_by_word(&word);

                        let hit = if let Some(entry) = entry {
                            entry.status == WordStatus::Learning
                        } else {
                            self.database.insert(&Translation {
                                word: word.clone(),
                                status: WordStatus::Learning,
                            });
                            true
                        };

                        if hit {
                            location = i;
                            break;
                        }
                    }

                    if location != self.word_location {
                        self.select_word(location);
                    }
                }
                Key::ArrowLeft => {
                    if self.word_location != 0 {
                        self.select_word(self.word_location - 1);
                    }
                }

                Key::N => {
                    if !self.translate_history.is_empty() {
                        self.update_last_words_status(WordStatus::NotAWord);
                    }
                }
                Key::L => {
                    if !self.translate_history.is_empty() {
                        self.update_last_words_status(WordStatus::Learning);
                    }
                }
                Key::M => {
                    if !self.translate_history.is_empty() {
                        self.update_last_words_status(WordStatus::Mastered);
                    }
                }

                Key::D => {
                    self.toggle_dictionary_pop_up(ctx);
                }
                Key::G => {
                    ctx.memory_mut(|mem| mem.request_focus(self.location_box_id));
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
