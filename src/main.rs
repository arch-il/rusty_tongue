use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use eframe::egui::{self, Color32, Rect, RichText, Vec2, ViewportBuilder, Widget};
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    StaticRb,
};
use rust_translate::translate_to_english;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default().with_inner_size(Vec2::new(600.0, 400.0));

    eframe::run_native(
        "Rusty Tongue",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}

#[derive(Clone, PartialEq)]
enum WordStatus {
    Learning,
    Mastered,
}

struct MyEguiApp {
    lines: Lines<BufReader<File>>,
    paragraph: Vec<RichText>,
    word_list: HashMap<String, (String, WordStatus)>,
    index: usize,
    text_history: Vec<String>,
    translate_history: StaticRb<(String, String), 100>,
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
            text_history: vec![],
            translate_history: StaticRb::<(String, String), 100>::default(),
        };
        temp.get_history_entry(0);
        temp
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut screen_rect = Rect::ZERO;

        ctx.input(|input_state| {
            screen_rect = input_state.screen_rect();
            if input_state.key_pressed(egui::Key::ArrowDown) {
                self.index += 1;
                self.get_history_entry(self.index);
            }

            if input_state.key_pressed(egui::Key::ArrowUp) && self.index != 0 {
                self.index -= 1;
                self.get_history_entry(self.index);
            }
        });

        const SIDE_PANEL_WIDTH: f32 = 200.0;

        egui::SidePanel::right("Info Panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Dictionary");

                let (mut learning, mut mastered) = (0, 0);
                for (_, (_, word_status)) in self.word_list.iter() {
                    match word_status {
                        WordStatus::Learning => learning += 1,
                        WordStatus::Mastered => mastered += 1,
                    }
                }

                ui.label(format!("learning: {learning}"));
                ui.label(format!("mastered: {mastered}"));

                ui.separator();
                ui.heading("History");

                if !self.translate_history.is_empty() {
                    if ui.button("Mark Mastered").clicked() {
                        let word = &self.translate_history.last().unwrap().0;
                        let (_, word_status) = self.word_list.get_mut(word).unwrap();
                        *word_status = WordStatus::Mastered;

                        self.get_history_entry(self.index);
                    }
                }

                let mut iter = self.translate_history.iter().rev();
                if let Some((from, to)) = iter.next() {
                    let color = if self.word_list.get(from).unwrap().1 == WordStatus::Learning {
                        Color32::YELLOW
                    } else {
                        Color32::GRAY
                    };

                    ui.label(RichText::from(format!("{from} - {to}")).color(color));
                }

                for (from, to) in iter {
                    ui.label(format!("{from} - {to}"));
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
                        let translated_word = translate_text(&word);

                        self.record_translate_history(&word, &translated_word);

                        if !self.word_list.keys().any(|x| x == &word) {
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
        while index >= self.text_history.len() {
            let text = next_paragraph(&mut self.lines);

            self.text_history.push(text);
        }

        self.paragraph = text_to_tokens(&self.text_history[self.index], &self.word_list);
    }

    fn record_translate_history(&mut self, from: &str, to: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push((from.to_string(), to.to_string()));
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
