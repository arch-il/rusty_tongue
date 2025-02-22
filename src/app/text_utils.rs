use eframe::egui::{Color32, RichText};
use ringbuf::traits::{Consumer, Observer, Producer, SplitRef};

use crate::database::{Database, WordStatus};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn get_history_entry(&mut self) {
        if self.location >= self.lines.len() {
            self.location = self.lines.len() - 1;
        }

        self.paragraph = text_to_tokens(&self.lines[self.location], &self.database);
    }

    pub fn record_translate_history(&mut self, word: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push(word.to_string());
    }
}

pub fn token_to_word(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

fn text_to_tokens(text: &str, database: &Database) -> Vec<RichText> {
    text.split(" ")
        .map(|token| {
            let from = token_to_word(token);

            if let Some(t) = database.get_by_word(&from) {
                return match &t.status {
                    WordStatus::Learning => {
                        RichText::from(token).color(Color32::LIGHT_YELLOW).strong()
                    }
                    WordStatus::Mastered | WordStatus::NotAWord => RichText::from(token),
                };
            }
            RichText::from(token).color(Color32::LIGHT_RED)
        })
        .collect()
}
