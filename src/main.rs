use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use eframe::egui::{self, Color32, RichText, Widget};
use rust_translate::translate_to_english;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Tongue",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}

#[derive(Clone)]
enum WordStatus {
    Learning,
    Mastered,
}

struct MyEguiApp {
    lines: Lines<BufReader<File>>,
    paragraph: Vec<RichText>,
    word_list: HashMap<String, (String, WordStatus)>,
    index: usize,
    history: Vec<String>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let file = File::open("book.txt").expect("Failed to read file");
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let word_list = [(String::new(), (String::new(), WordStatus::Mastered))]
            .iter()
            .cloned()
            .collect::<HashMap<String, (String, WordStatus)>>();

        let mut temp = Self {
            lines,
            paragraph: vec![],
            word_list,
            index: 0,
            history: vec![],
        };
        temp.get_history_entry(0);
        temp
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|input_state| {
            if input_state.key_pressed(egui::Key::ArrowDown) {
                self.index += 1;
                self.get_history_entry(self.index);
            }

            if input_state.key_pressed(egui::Key::ArrowUp) && self.index != 0 {
                self.index -= 1;
                self.get_history_entry(self.index);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reading Area");

            // ? Why do I need horizontal with labels?
            ui.horizontal_wrapped(|ui| {
                // ! Try to remove clone here
                for token in self.paragraph.clone().iter() {
                    let label_button = egui::Label::new(token.clone())
                        .sense(egui::Sense::click())
                        .ui(ui);

                    if label_button.clicked() {
                        let word = token_to_word(token.text());
                        if !self.word_list.keys().any(|x| x == &word) {
                            let translated_word = translate_text(&word);
                            self.word_list
                                .insert(word, (translated_word, WordStatus::Learning));
                            self.get_history_entry(self.index);
                        }
                    }
                }
            })
        });
    }
}

impl MyEguiApp {
    fn get_history_entry(&mut self, index: usize) {
        while index >= self.history.len() {
            let text = next_paragraph(&mut self.lines);

            self.history.push(text);
        }

        self.paragraph = text_to_tokens(&self.history[self.index], &self.word_list);
    }
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

fn token_to_word(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

fn text_to_tokens(text: &str, word_list: &HashMap<String, (String, WordStatus)>) -> Vec<RichText> {
    text.split(" ")
        .map(|token| {
            let word = token_to_word(token);

            if let Some((_, word_status)) = word_list.get(&word) {
                return match &word_status {
                    WordStatus::Learning => RichText::from(token).color(Color32::YELLOW),
                    WordStatus::Mastered => RichText::from(token),
                };
            }
            RichText::from(token).color(Color32::RED)
        })
        .collect()
}

fn translate_text(text: &str) -> String {
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
