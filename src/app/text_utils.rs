use dictcc::{Dict, DictEntry};
use eframe::egui::{Color32, RichText};
use ringbuf::traits::{Consumer, Observer, Producer, SplitRef};
use rust_translate::translate;

use crate::database::{Database, WordStatus};

use super::{
    draw::side_panel::language::{self, Language},
    MyEguiApp,
};

impl MyEguiApp {
    pub fn get_history_entry(&mut self) {
        if self.location >= self.lines.len() {
            self.location = self.lines.len() - 1;
        }

        self.paragraph = text_to_tokens(&self.lines[self.location], &self.user_database);
    }

    pub fn record_translate_history(&mut self, word: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push(word.to_string());
    }
}

pub fn find_in_dict(dict_database: &Option<Dict>, word: &str) -> Option<Vec<DictEntry>> {
    if let Some(dict_database) = &dict_database {
        let mut temp = dict_database
            .get_entries()
            .iter()
            .filter(|entry| entry.left_word.plain_word().to_lowercase().contains(word))
            .cloned()
            .collect::<Vec<_>>();
        temp.sort_by_key(|entry| entry.left_word.plain_word().len());
        temp.truncate(100);
        Some(temp)
    } else {
        None
    }
}

pub fn translate_text(text: &str, from: Language, to: Language) -> String {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            translate(
                text,
                &language::language_to_code(from),
                &language::language_to_code(to),
            )
            .await
            .expect("Failed translating text")
        })
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
                    WordStatus::Learning => RichText::from(token).color(Color32::YELLOW),
                    WordStatus::Mastered | WordStatus::NotAWord => RichText::from(token),
                    _ => panic!("Invalid status in database"),
                };
            }
            RichText::from(token).color(Color32::RED)
        })
        .collect()
}
