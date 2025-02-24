use eframe::egui::{Color32, RichText};
use ringbuf::traits::{Consumer, Observer, Producer, SplitRef};

use crate::database::{Database, Translation, WordStatus};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn refresh_page(&mut self) {
        if self.page_location >= self.lines.len() {
            self.page_location = self.lines.len() - 1;
        }

        self.paragraph = text_to_tokens(
            &self.lines[self.page_location],
            &self.database,
            self.word_location,
        );
    }

    pub fn record_translate_history(&mut self, word: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push(word.to_string());
    }

    pub fn select_word(&mut self, word_location: usize) {
        self.word_location = if word_location >= self.paragraph.len() {
            0
        } else {
            word_location
        };

        let word = token_to_word(self.paragraph[word_location].text());
        self.record_translate_history(&word);

        if Some(word.clone()) != self.dictionary_pop_up.curr_word {
            self.dictionary_pop_up.curr_word = Some(word.clone());
            self.dictionary_pop_up.curr_entries = None;
        }

        if self.database.get_by_word(&word).is_none() {
            self.database.insert(&Translation {
                word,
                status: WordStatus::Learning,
            });
        }

        self.refresh_page();
    }
}

pub fn token_to_word(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

fn text_to_tokens(text: &str, database: &Database, word_location: usize) -> Vec<RichText> {
    text.split(" ")
        .enumerate()
        .map(|(i, token)| {
            let from = token_to_word(token);

            let result = if let Some(t) = database.get_by_word(&from) {
                match &t.status {
                    WordStatus::Learning => {
                        RichText::from(token).color(Color32::LIGHT_YELLOW).strong()
                    }
                    WordStatus::Mastered | WordStatus::NotAWord => RichText::from(token),
                }
            } else {
                RichText::from(token).color(Color32::LIGHT_RED)
            };

            if i == word_location {
                result.underline()
            } else {
                result
            }
        })
        .collect()
}
