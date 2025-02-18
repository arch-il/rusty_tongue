use std::{
    fs::File,
    io::{BufReader, Lines},
};

use eframe::egui::{Color32, RichText};
use ringbuf::traits::{Consumer, Observer, Producer, SplitRef};
use rust_translate::translate_to_english;

use crate::database::{Database, WordStatus};

use super::MyEguiApp;

impl MyEguiApp {
    pub fn get_history_entry(&mut self, index: usize) {
        while index >= self.text_history.len() {
            let text = next_paragraph(&mut self.lines);

            self.text_history.push(text);
        }

        self.paragraph = text_to_tokens(&self.text_history[self.index], &self.database);
    }

    pub fn record_translate_history(&mut self, from: &str, to: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push((from.to_string(), to.to_string()));
    }
}

pub fn translate_text(text: &str) -> String {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            translate_to_english(text)
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

fn next_paragraph(lines: &mut Lines<BufReader<File>>) -> String {
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => continue,
        };

        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading a line {e}");
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        return line;
    }
}

fn text_to_tokens(text: &str, database: &Database) -> Vec<RichText> {
    text.split(" ")
        .map(|token| {
            let from = token_to_word(token);

            if let Some(t) = database.get_by_from(&from) {
                return match &t.status {
                    WordStatus::Learning => RichText::from(token).color(Color32::YELLOW),
                    WordStatus::Mastered | WordStatus::NotAWord => RichText::from(token),
                };
            }
            RichText::from(token).color(Color32::RED)
        })
        .collect()
}
