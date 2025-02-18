use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use eframe::egui::{self, Color32, Rect, RichText, Widget};
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    StaticRb,
};
use ritelinked::LinkedHashMap;

use crate::word_status::WordStatus;

mod text_utils;

pub struct MyEguiApp {
    lines: Lines<BufReader<File>>,
    paragraph: Vec<RichText>,

    dictionary: LinkedHashMap<String, (String, WordStatus)>,
    index: usize,
    dictionary_open: bool,
    search_text: String,

    text_history: Vec<String>,
    translate_history: StaticRb<(String, String), 100>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            .collect::<LinkedHashMap<String, (String, WordStatus)>>();

        let mut temp = Self {
            lines,
            paragraph: vec![],

            dictionary: word_list,
            index: 0,
            dictionary_open: false,
            search_text: String::new(),

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

                if ui
                    .button(if self.dictionary_open {
                        "Close Dictionary"
                    } else {
                        "Open Dictionary"
                    })
                    .clicked()
                {
                    self.dictionary_open = !self.dictionary_open;
                }

                egui::Window::new("Dictionary")
                    .open(&mut self.dictionary_open)
                    .resizable(true)
                    .max_width(200.0)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Search:");

                            ui.text_edit_singleline(&mut self.search_text);

                            if ui.button("❌").clicked() {
                                self.search_text = String::new();
                            }
                        });

                        for (from, (to, status)) in self.dictionary.iter().skip(1).rev() {
                            if !from.contains(&self.search_text) && !to.contains(&self.search_text)
                            {
                                continue;
                            }

                            ui.horizontal(|ui| {
                                let _ = match status {
                                    WordStatus::Learning => ui.label(
                                        RichText::from(format!("📖")).color(Color32::YELLOW),
                                    ),
                                    WordStatus::Mastered => ui
                                        .label(RichText::from(format!("✅")).color(Color32::GREEN)),
                                };

                                ui.label(format!("{from} - {to}"));
                            });
                        }
                    });

                let (mut learning, mut mastered) = (0, 0);
                for (_, (_, word_status)) in self.dictionary.iter() {
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
                        let (_, word_status) = self.dictionary.get_mut(word).unwrap();
                        *word_status = WordStatus::Mastered;

                        self.get_history_entry(self.index);
                    }
                }

                let mut iter = self.translate_history.iter().rev();
                if let Some((from, to)) = iter.next() {
                    let color = if self.dictionary.get(from).unwrap().1 == WordStatus::Learning {
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
                        let word = text_utils::token_to_word(token.text());
                        let translated_word = text_utils::translate_text(&word);

                        self.record_translate_history(&word, &translated_word);

                        if !self.dictionary.keys().any(|x| x == &word) {
                            self.dictionary
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
            let text = text_utils::next_paragraph(&mut self.lines);

            self.text_history.push(text);
        }

        self.paragraph =
            text_utils::text_to_tokens(&self.text_history[self.index], &self.dictionary);
    }

    fn record_translate_history(&mut self, from: &str, to: &str) {
        let (mut prod, mut cons) = self.translate_history.split_ref();
        if cons.is_full() {
            cons.try_pop();
        }
        let _ = prod.try_push((from.to_string(), to.to_string()));
    }
}
