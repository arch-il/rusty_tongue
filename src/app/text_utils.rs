use std::{
    fs::File,
    io::{BufReader, Lines},
};

use eframe::egui::{Color32, RichText};
use rust_translate::translate_to_english;

use crate::database::{Database, WordStatus};

pub fn next_paragraph(lines: &mut Lines<BufReader<File>>) -> String {
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

pub fn token_to_word(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

pub fn text_to_tokens(text: &str, database: &Database) -> Vec<RichText> {
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

pub fn translate_text(text: &str) -> String {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            translate_to_english(&text)
                .await
                .expect("Failed translating text")
        })
}
